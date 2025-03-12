use std::collections::HashMap;

use crate::case::Case;
use proc_macro2::{Ident, TokenStream};
use syn::{DeriveInput, Meta};

static ATTRIBUTE_NAME: &str = "enum_stringify";

/// Parses a string literal by removing surrounding quotes if present.
fn parse_string(s: &str) -> Result<String, ()> {
    if s.starts_with('"') && s.ends_with('"') {
        Ok(s[1..s.len() - 1].to_string())
    } else {
        Err(())
    }
}

#[derive(Clone)]
/// Represents a rename attribute applied to an enum variant.
pub struct VariantRename(String);

impl TryFrom<(String, String)> for VariantRename {
    type Error = ();

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        if value.0 == "rename" {
            Ok(Self(parse_string(value.1.as_str())?))
        } else {
            Err(())
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

// Represents different renaming attributes that can be applied to enum variants.
pub enum Attribute {
    Case(Case),
    Prefix(String),
    Suffix(String),
}

impl TryFrom<(String, String)> for Attribute {
    type Error = ();

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        if value.0 == "prefix" {
            Ok(Self::Prefix(parse_string(value.1.as_str())?))
        } else if value.0 == "suffix" {
            Ok(Self::Suffix(parse_string(value.1.as_str())?))
        } else if value.0 == "case" {
            Ok(Self::Case(Case::try_from(value)?))
        } else {
            Err(())
        }
    }
}

#[derive(Default)]
pub(crate) struct Attributes {
    pub(crate) case: Option<Case>,
    pub(crate) prefix: Option<String>,
    pub(crate) suffix: Option<String>,
}

impl Attributes {
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
                    let attributes =
                        Attributes::parse_token_list::<Attribute>(&list.tokens).ok()?;
                    for attr in attributes {
                        new.merge_attribute(attr);
                    }
                    Some(new)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn merge_attribute(&mut self, attr: Attribute) {
        match attr {
            Attribute::Prefix(s) => self.prefix = Some(s),
            Attribute::Suffix(s) => self.suffix = Some(s),
            Attribute::Case(s) => self.case = Some(s),
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
}

pub(crate) struct Variants {
    variant_renames: HashMap<Ident, Option<VariantRename>>,
}

impl Variants {
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

    fn parse_variant_attribute(&mut self, variant: &syn::Variant) {
        let attribute_renames = variant.attrs.iter().filter_map(VariantRename::parse_args);

        let rename = attribute_renames.last();

        self.variant_renames.insert(variant.ident.clone(), rename);
    }

    pub(crate) fn apply(&self, attributes: &Attributes) -> Vec<(syn::Ident, String)> {
        let mut new_names = Vec::new();

        for (name, rename) in &self.variant_renames {
            if let Some(rename) = rename {
                new_names.push(rename.0.clone());
                continue;
            }
            let mut new_name = String::new();
            if let Some(prefix) = &attributes.prefix {
                new_name.push_str(prefix);
            }

            new_name.push_str(&name.to_string());

            if let Some(suffix) = &attributes.suffix {
                new_name.push_str(suffix);
            }

            if let Some(case) = &attributes.case {
                new_name = case.to_case(&new_name);
            }

            new_names.push(new_name);
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
