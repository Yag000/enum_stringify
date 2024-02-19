use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use syn::{DeriveInput, Meta};

fn parse_string(s: String) -> Result<String, ()> {
    if s.starts_with('"') && s.ends_with('"') {
        Ok(s[1..s.len() - 1].to_string())
    } else {
        Err(())
    }
}

pub(crate) enum Case {
    Lower,
    Upper,
}

impl TryFrom<(String, String)> for Case {
    type Error = ();

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        if value.0 == "case" {
            value.1.try_into()
        } else {
            Err(())
        }
    }
}

impl TryFrom<String> for Case {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "\"lower\"" => Self::Lower,
            "\"upper\"" => Self::Upper,
            _ => Err(())?,
        })
    }
}

#[derive(Default)]
pub(crate) struct Rename {
    prefix: Option<String>,
    suffix: Option<String>,
    case: Option<Case>,
}

impl TryFrom<(String, String)> for Rename {
    type Error = ();

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        if value.0 == "prefix" {
            Ok(Self {
                prefix: Some(parse_string(value.1)?),
                suffix: None,
                case: None,
            })
        } else if value.0 == "suffix" {
            Ok(Self {
                prefix: None,
                suffix: Some(parse_string(value.1)?),
                case: None,
            })
        } else if value.0 == "case" {
            Ok(Self {
                prefix: None,
                suffix: None,
                case: Some(Case::try_from(value)?),
            })
        } else {
            Err(())
        }
    }
}

impl Rename {
    fn parse_args(attribute: &syn::Attribute) -> Option<Self> {
        if !attribute.path().is_ident(ATTRIBUTE_NAME) {
            return None;
        }

        match &attribute.meta {
            Meta::List(list) => {
                let path = list
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>();

                if path == vec![ATTRIBUTE_NAME] {
                    let attributes = Attributes::parse_token_list::<Self>(&list.tokens).ok()?;
                    Some(Self::merge_renames(attributes))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn merge_renames(renames: Vec<Self>) -> Self {
        let mut merge = Rename::default();

        for rename in renames {
            if rename.prefix.is_some() {
                merge.prefix = rename.prefix;
            }
            if rename.suffix.is_some() {
                merge.suffix = rename.suffix;
            }
            if rename.case.is_some() {
                merge.case = rename.case;
            }
        }

        merge
    }
}

#[derive(Clone)]
pub struct VariantRename(String);

impl TryFrom<(String, String)> for VariantRename {
    type Error = ();

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        if value.0 == "rename" {
            Ok(Self(parse_string(value.1)?))
        } else {
            Err(())
        }
    }
}

impl VariantRename {
    fn parse_args(attribute: &syn::Attribute) -> Option<Self> {
        if !attribute.path().is_ident(ATTRIBUTE_NAME) {
            return None;
        }

        match &attribute.meta {
            Meta::List(list) => {
                let path = list
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>();

                if path == vec![ATTRIBUTE_NAME] {
                    Some(
                        Attributes::parse_token_list::<Self>(&list.tokens)
                            .ok()?
                            .first()?
                            .clone(),
                    )
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

pub(crate) struct Attributes {
    pub(crate) case: Option<Case>,
    pub(crate) prefix: Option<String>,
    pub(crate) suffix: Option<String>,

    variant_renames: HashMap<Ident, Option<VariantRename>>,
}

static ATTRIBUTE_NAME: &str = "enum_stringify";

impl Attributes {
    pub(crate) fn new(ast: &DeriveInput) -> Self {
        let mut new = Self {
            case: None,
            prefix: None,
            suffix: None,
            variant_renames: HashMap::new(),
        };

        ast.attrs.iter().for_each(|attr| {
            let rename_rules = Rename::parse_args(attr);
            if let Some(rename_rules) = rename_rules {
                new.prefix = rename_rules.prefix;
                new.suffix = rename_rules.suffix;
                new.case = rename_rules.case;
            };
        });

        let variants = match ast.data {
            syn::Data::Enum(ref e) => &e.variants,
            _ => panic!("EnumToString only works with Enums"),
        };

        variants
            .iter()
            .for_each(|variant| new.parse_variant_attribute(variant));

        new
    }

    pub fn parse_variant_attribute(&mut self, variant: &syn::Variant) {
        let attribute_renames = variant.attrs.iter().filter_map(VariantRename::parse_args);

        let rename = attribute_renames.last();

        self.variant_renames.insert(variant.ident.clone(), rename);
    }

    pub(crate) fn apply(&self) -> Vec<(syn::Ident, syn::Ident)> {
        let mut new_names = Vec::new();

        for (name, rename) in self.variant_renames.iter() {
            if let Some(rename) = rename {
                new_names.push(syn::Ident::new(&rename.0, name.span()));
                continue;
            }
            let mut new_name = String::new();
            if let Some(prefix) = &self.prefix {
                new_name.push_str(prefix);
            }

            // Add here case logic
            new_name.push_str(&name.to_string());

            if let Some(suffix) = &self.suffix {
                new_name.push_str(suffix);
            }

            if let Some(case) = &self.case {
                new_name = match case {
                    Case::Lower => new_name.to_lowercase(),
                    Case::Upper => new_name.to_uppercase(),
                };
            }

            new_names.push(syn::Ident::new(&new_name, name.span()));
        }

        let tmp = self
            .variant_renames
            .keys()
            .cloned()
            .zip(new_names)
            .collect::<Vec<_>>();

        tmp
    }

    fn parse_token_list<T>(tokens: &TokenStream) -> Result<Vec<T>, String>
    where
        T: TryFrom<(String, String)>,
    {
        let mut result = Vec::new();
        let mut tokens = tokens.clone().into_iter();

        while let Some(attribute_type) = tokens.next() {
            let attribute_type = attribute_type.to_string();

            if tokens.next().expect("type must be specified").to_string() != "=" {
                panic!("too many arguments");
            }
            let value = tokens.next().expect("value must be specified").to_string();

            match T::try_from((attribute_type.clone(), value)) {
                Ok(value) => result.push(value),
                Err(_) => return Err(format!("Invalid argument: {}", attribute_type)),
            }

            if let Some(comma_separator) = tokens.next() {
                if comma_separator.to_string() != "," {
                    panic!("Expected a commaseparated attribute list");
                }
            }
        }
        Ok(result)
    }
}
