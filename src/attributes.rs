use std::collections::HashMap;

use crate::case::Case;
use proc_macro2::{Ident, TokenStream};
use syn::{DeriveInput, Meta};

static ATTRIBUTE_NAME: &str = "enum_stringify";

/// Parses a string literal by removing surrounding quotes if present.
fn parse_string(s: &str) -> Result<String, &'static str> {
    if let Some(stripped) = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        Ok(stripped.to_string())
    } else {
        Err("String must be enclosed in double quotes")
    }
}

fn parse_token_list<T>(tokens: &TokenStream) -> Result<Vec<T>, String>
where
    T: TryFrom<(String, String)>,
{
    let mut result = Vec::new();
    let mut tokens = tokens.clone().into_iter();

    while let Some(attribute_type) = tokens.next() {
        let attribute_type = attribute_type.to_string();

        assert!(
            tokens.next().expect("type must be specified").to_string() == "=",
            "too many arguments"
        );
        let value = tokens.next().expect("value must be specified").to_string();

        match T::try_from((attribute_type.clone(), value)) {
            Ok(value) => result.push(value),
            Err(_) => return Err(format!("Invalid argument: {attribute_type}")),
        }

        if let Some(comma_separator) = tokens.next() {
            assert!(
                comma_separator.to_string() == ",",
                "Expected a comma separated attribute list"
            );
        }
    }
    Ok(result)
}

#[derive(Clone)]
/// Represents a rename attribute applied to an enum variant.
struct VariantRename(String);

impl TryFrom<(String, String)> for VariantRename {
    type Error = &'static str;

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        if value.0 == "rename" {
            Ok(Self(parse_string(value.1.as_str())?))
        } else {
            Err("Not a rename string")
        }
    }
}

impl VariantRename {
    /// Parses an attribute to determine if it is a rename directive.
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
                        parse_token_list::<Self>(&list.tokens)
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

#[derive(Default)]
pub(crate) struct Attributes {
    case: Option<Case>,
    prefix: Option<String>,
    suffix: Option<String>,
}

impl Attributes {
    /// Constructs an `Attributes` instance by parsing derive attributes from an AST.
    pub(crate) fn new(ast: &DeriveInput) -> Self {
        let mut new = Self {
            case: None,
            prefix: None,
            suffix: None,
        };

        ast.attrs.iter().for_each(|attr| {
            let rename_rules = Self::parse_args(attr);
            if let Some(rename_rules) = rename_rules {
                new.prefix = rename_rules.prefix;
                new.suffix = rename_rules.suffix;
                new.case = rename_rules.case;
            };
        });

        new
    }

    /// Parses attributes related to casing, prefixes, and suffixes.
    fn parse_args(attribute: &syn::Attribute) -> Option<Self> {
        if !attribute.path().is_ident(ATTRIBUTE_NAME) {
            return None;
        }

        let mut new = Self::default();

        match &attribute.meta {
            Meta::List(list) => {
                let path = list
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>();

                if path == vec![ATTRIBUTE_NAME] {
                    let attributes = parse_token_list::<(String, String)>(&list.tokens).ok()?;
                    for value in attributes {
                        new.update_attribute(value);
                    }
                    Some(new)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Updates an attribute with a new parameter
    fn update_attribute(&mut self, value: (String, String)) {
        if value.0 == "prefix" {
            self.prefix = Some(parse_string(value.1.as_str()).expect("Not a rename attribute"));
        } else if value.0 == "suffix" {
            self.suffix = Some(parse_string(value.1.as_str()).expect("Not a rename attribute"));
        } else if value.0 == "case" {
            self.case = Some(Case::try_from(value).expect("Not a rename attribute"));
        }
    }

    fn rename(&self, s: &str) -> String {
        let mut new_name = String::new();
        if let Some(prefix) = &self.prefix {
            new_name.push_str(prefix);
        }

        new_name.push_str(s);

        if let Some(suffix) = &self.suffix {
            new_name.push_str(suffix);
        }

        if let Some(case) = &self.case {
            new_name = case.to_case(&new_name);
        }

        new_name
    }
}

/// Stores enum variants and their optional renaming attributes.
pub(crate) struct Variants {
    variant_renames: HashMap<Ident, Option<VariantRename>>,
}

impl Variants {
    /// Parses an AST to extract enum variants and their attributes.
    pub(crate) fn new(ast: &DeriveInput) -> Self {
        let mut new = Self {
            variant_renames: HashMap::new(),
        };

        let variants = match ast.data {
            syn::Data::Enum(ref e) => &e.variants,
            _ => panic!("EnumToString only works with Enums"),
        };

        variants
            .iter()
            .for_each(|variant| new.parse_variant_attribute(variant));

        new
    }

    /// Extracts renaming attributes from an enum variant.
    fn parse_variant_attribute(&mut self, variant: &syn::Variant) {
        let attribute_renames = variant.attrs.iter().filter_map(VariantRename::parse_args);

        let rename = attribute_renames.last();

        self.variant_renames.insert(variant.ident.clone(), rename);
    }

    /// Applies attributes (prefix, suffix, case) to enum variant names.
    pub(crate) fn apply(&self, attributes: &Attributes) -> Vec<(syn::Ident, String)> {
        let mut new_names = Vec::new();

        for (name, rename) in &self.variant_renames {
            if let Some(rename) = rename {
                new_names.push(rename.0.clone());
                continue;
            }
            new_names.push(attributes.rename(name.to_string().as_str()));
        }

        let tmp = self
            .variant_renames
            .keys()
            .cloned()
            .zip(new_names)
            .collect::<Vec<_>>();

        tmp
    }
}
