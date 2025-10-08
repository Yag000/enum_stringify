use std::borrow::Cow;
use std::collections::HashMap;

use crate::case::Case;
use proc_macro2::{Ident, TokenStream};
use syn::{DeriveInput, Meta};

/// The attribute name used for enum variant renaming.
static ATTRIBUTE_NAME: &str = "enum_stringify";

/// Parses a string literal by removing surrounding double quotes if present.
///
/// # Arguments
/// * `s` - A string slice containing the quoted string.
///
/// # Returns
/// * `Ok(String)` if the string is correctly formatted.
/// * `Err(&'static str)` if the string is not enclosed in double quotes.
fn parse_string(s: &str) -> Result<String, &'static str> {
    s.strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .map(std::string::ToString::to_string)
        .ok_or("String must be enclosed in double quotes")
}

/// Parses a list of attribute tokens into a vector of type `T`.
///
/// # Arguments
/// * `tokens` - A reference to a token stream containing attributes.
///
/// # Returns
/// * `Ok(Vec<T>)` if parsing succeeds.
/// * `Err(String)` if parsing fails due to incorrect syntax.
fn parse_token_list<T>(tokens: &TokenStream) -> Result<Vec<T>, String>
where
    T: TryFrom<(String, String)>,
{
    let mut result = Vec::new();
    let mut tokens = tokens.clone().into_iter();

    while let Some(attribute_type) = tokens.next() {
        let attribute_type = attribute_type.to_string();

        let Some(eq_token) = tokens.next() else {
            return Err(format!("Expected '=' after '{attribute_type}'"));
        };
        if eq_token.to_string() != "=" {
            return Err(format!("Unexpected token '{eq_token}', expected '='"));
        }

        let value = tokens.next().ok_or("Value must be specified")?.to_string();

        match T::try_from((attribute_type.clone(), value)) {
            Ok(value) => result.push(value),
            Err(_) => return Err(format!("Invalid argument: {attribute_type}")),
        }

        if let Some(comma_separator) = tokens.next() {
            if comma_separator.to_string() != "," {
                return Err("Expected a comma-separated attribute list".to_string());
            }
        }
    }
    Ok(result)
}

/// Represents a rename attribute for an enum variant.
#[derive(Clone, Debug, PartialEq)]
struct VariantRename(String);

impl TryFrom<(String, String)> for VariantRename {
    type Error = &'static str;

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        if value.0 == "rename" {
            Ok(Self(parse_string(&value.1)?))
        } else {
            Err("Not a rename string")
        }
    }
}

impl VariantRename {
    /// Parses the rename attribute from a given `syn::Attribute`.
    fn parse_args(attribute: &syn::Attribute) -> Option<Self> {
        if !attribute.path().is_ident(ATTRIBUTE_NAME) {
            return None;
        }

        match &attribute.meta {
            Meta::List(list) => parse_token_list::<Self>(&list.tokens)
                .ok()?
                .first()
                .cloned(),
            _ => None,
        }
    }
}

/// Represents attribute configurations for renaming enum variants.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Attributes {
    case: Option<Case>,
    prefix: Option<String>,
    suffix: Option<String>,
}

impl Attributes {
    /// Constructs an `Attributes` instance by parsing the attributes of a derive input.
    pub(crate) fn new(ast: &DeriveInput) -> Self {
        let mut new = Self::default();
        ast.attrs.iter().for_each(|attr| {
            if let Some(rename_rules) = Self::parse_args(attr) {
                new.prefix = rename_rules.prefix;
                new.suffix = rename_rules.suffix;
                new.case = rename_rules.case;
            }
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
                let attributes = parse_token_list::<(String, String)>(&list.tokens).ok()?;
                for value in attributes {
                    new.update_attribute(value);
                }
                Some(new)
            }
            _ => None,
        }
    }

    fn update_attribute(&mut self, value: (String, String)) {
        match value.0.as_str() {
            "prefix" => self.prefix = parse_string(&value.1).ok(),
            "suffix" => self.suffix = parse_string(&value.1).ok(),
            "case" => self.case = Case::try_from(value).ok(),
            _ => {}
        }
    }

    /// Applies renaming rules (prefix, suffix, case) to a given string.
    fn rename<'a>(&self, s: &'a str) -> Cow<'a, str> {
        let mut new_name = Cow::Borrowed(s);

        if let Some(prefix) = &self.prefix {
            new_name = Cow::Owned(format!("{prefix}{new_name}"));
        }
        if let Some(suffix) = &self.suffix {
            new_name = Cow::Owned(format!("{new_name}{suffix}"));
        }
        if let Some(case) = &self.case {
            new_name = Cow::Owned(case.to_case(&new_name));
        }
        new_name
    }
}

/// Stores renaming information for enum variants.
pub struct Variants {
    variant_renames: HashMap<Ident, Option<VariantRename>>,
}

impl Variants {
    /// Constructs a `Variants` instance by parsing the derive input.
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

    /// Parses attributes for a given enum variant.
    fn parse_variant_attribute(&mut self, variant: &syn::Variant) {
        let rename = variant
            .attrs
            .iter()
            .filter_map(VariantRename::parse_args)
            .reduce(|_, new| new);
        self.variant_renames.insert(variant.ident.clone(), rename);
    }

    /// Applies renaming rules to each enum variant name.
    pub(crate) fn apply(&self, attributes: &Attributes) -> Vec<(syn::Ident, String)> {
        self.variant_renames
            .iter()
            .map(|(ident, rename)| {
                let new_name = if let Some(rename) = rename {
                    rename.0.clone()
                } else {
                    attributes.rename(ident.to_string().as_str()).into_owned()
                };
                (ident.clone(), new_name)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn test_parse_string() {
        assert_eq!(parse_string("\"hello\""), Ok("hello".to_string()));
        assert_eq!(
            parse_string("hello"),
            Err("String must be enclosed in double quotes")
        );

        assert_eq!(
            parse_string("\"hello"),
            Err("String must be enclosed in double quotes")
        );
        assert_eq!(
            parse_string("hello\""),
            Err("String must be enclosed in double quotes")
        );
        assert_eq!(parse_string("\"he\"llo\""), Ok("he\"llo".to_string()));

        assert_eq!(parse_string("\"\""), Ok(String::new()));
        assert_eq!(parse_string("\"\"\""), Ok("\"".to_string()));
    }

    fn assert_parse_token_list<T>(tokens: TokenStream, expected: Vec<T>)
    where
        T: TryFrom<(String, String)> + PartialEq + std::fmt::Debug,
    {
        let result = parse_token_list::<T>(&tokens).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_token_list_string_string() {
        assert_parse_token_list::<(String, String)>(quote! {}, vec![]);

        assert_parse_token_list::<(String, String)>(
            quote! { rename = "hello" },
            vec![("rename".to_string(), "\"hello\"".to_string())],
        );

        assert_parse_token_list(
            quote! { rename = "hello", rename = "world" },
            vec![
                ("rename".to_string(), "\"hello\"".to_string()),
                ("rename".to_string(), "\"world\"".to_string()),
            ],
        );
    }

    #[derive(Debug, PartialEq)]
    struct TestStruct(String);

    impl TryFrom<(String, String)> for TestStruct {
        type Error = &'static str;

        fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
            Ok(Self(parse_string(value.1.as_str())?))
        }
    }

    #[test]
    fn test_parse_token_list_struct() {
        assert_parse_token_list::<TestStruct>(
            quote! { rename = "hello" },
            vec![TestStruct("hello".to_string())],
        );

        assert_parse_token_list::<TestStruct>(
            quote! { rename = "hello", rename = "world" },
            vec![
                TestStruct("hello".to_string()),
                TestStruct("world".to_string()),
            ],
        );
    }

    #[test]
    fn test_variant_rename_try_from() {
        assert_eq!(
            VariantRename::try_from(("rename".to_string(), "\"hello\"".to_string())),
            Ok(VariantRename("hello".to_string()))
        );

        assert_eq!(
            VariantRename::try_from(("rename".to_string(), "hello".to_string())),
            Err("String must be enclosed in double quotes")
        );

        assert_eq!(
            VariantRename::try_from(("not_rename".to_string(), "\"hello\"".to_string())),
            Err("Not a rename string")
        );
    }

    #[test]
    fn test_parse_token_list_variant_rename() {
        assert_parse_token_list::<VariantRename>(
            quote! { rename = "hello" },
            vec![VariantRename("hello".to_string())],
        );

        assert_parse_token_list::<VariantRename>(
            quote! { rename = "hello", rename = "world" },
            vec![
                VariantRename("hello".to_string()),
                VariantRename("world".to_string()),
            ],
        );
    }

    #[test]
    fn test_attributes_parse_args() {
        let attribute =
            syn::parse_quote! { #[enum_stringify(prefix = "pre", suffix = "suf", case = "snake")] };
        let attributes = Attributes::parse_args(&attribute).unwrap();
        assert_eq!(attributes.prefix, Some("pre".to_string()));
        assert_eq!(attributes.suffix, Some("suf".to_string()));
        assert_eq!(
            attributes.case.map(|a| a.to_string()),
            Some("snake".to_string())
        );

        let attribute = syn::parse_quote! { #[enum_stringify(prefix = "pre", suffix = "suf")] };
        let attributes = Attributes::parse_args(&attribute).unwrap();
        assert_eq!(attributes.prefix, Some("pre".to_string()));
        assert_eq!(attributes.suffix, Some("suf".to_string()));
        assert_eq!(attributes.case, None);

        let attribute = syn::parse_quote! { #[enum_stringify(case = "snake")] };
        let attributes = Attributes::parse_args(&attribute).unwrap();
        assert_eq!(attributes.prefix, None);
        assert_eq!(attributes.suffix, None);
        assert_eq!(
            attributes.case.map(|a| a.to_string()),
            Some("snake".to_string())
        );

        let attribute = syn::parse_quote! { #[enum_stringify] };
        assert_eq!(Attributes::parse_args(&attribute), None);
    }

    #[test]
    fn test_attributes_update_attribute() {
        let mut attributes = Attributes::default();
        attributes.update_attribute(("prefix".to_string(), "\"pre\"".to_string()));
        assert_eq!(attributes.prefix, Some("pre".to_string()));

        attributes.update_attribute(("suffix".to_string(), "\"suf\"".to_string()));
        assert_eq!(attributes.suffix, Some("suf".to_string()));

        attributes.update_attribute(("case".to_string(), "\"snake\"".to_string()));
        assert_eq!(
            attributes.case.clone().map(|a| a.to_string()),
            Some("snake".to_string())
        );

        attributes.update_attribute(("invalid".to_string(), "\"value\"".to_string()));
        assert_eq!(attributes.prefix, Some("pre".to_string()));
        assert_eq!(attributes.suffix, Some("suf".to_string()));
        assert_eq!(
            attributes.case.clone().map(|a| a.to_string()),
            Some("snake".to_string())
        );

        attributes.update_attribute(("prefix".to_string(), "\"new1\"".to_string()));
        assert_eq!(attributes.prefix, Some("new1".to_string()));

        attributes.update_attribute(("suffix".to_string(), "\"new2\"".to_string()));
        assert_eq!(attributes.suffix, Some("new2".to_string()));

        attributes.update_attribute(("case".to_string(), "\"upper\"".to_string()));
        assert_eq!(
            attributes.case.clone().map(|a| a.to_string()),
            Some("upper".to_string())
        );
    }

    #[test]
    fn test_attributes_rename() {
        let mut attributes = Attributes {
            prefix: Some("pre".to_string()),
            suffix: Some("suf".to_string()),
            case: None,
        };

        assert_eq!(attributes.rename("name"), "prenamesuf");
        assert_eq!(attributes.rename("Name"), "preNamesuf");
        assert_eq!(attributes.rename("NAME"), "preNAMEsuf");

        attributes.update_attribute(("case".to_string(), "\"upper_flat\"".to_string()));

        assert_eq!(attributes.rename("name"), "PRENAMESUF");
        assert_eq!(attributes.rename("Name"), "PRENAMESUF");
        assert_eq!(attributes.rename("NAME"), "PRENAMESUF");
    }
}
