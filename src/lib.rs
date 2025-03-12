//! # enum-stringify
//!
//! A procedural macro that derives implementations for:
//! - [`std::fmt::Display`]: Converts enum variants to their string representations.
//! - [`std::str::FromStr`]: Parses a string into an enum variant.
//! - [`TryFrom<&str>`] and [`TryFrom<String>`]: Alternative conversion methods.
//!
//! ## Example
//!
//! ```
//! use enum_stringify::EnumStringify;
//! use std::str::FromStr;
//!
//! #[derive(EnumStringify, Debug, PartialEq)]
//! enum Numbers {
//!    One,
//!    Two,
//! }
//!
//! assert_eq!(Numbers::One.to_string(), "One");
//! assert_eq!(Numbers::Two.to_string(), "Two");
//!
//!
//! assert_eq!(Numbers::try_from("One").unwrap(), Numbers::One);
//! assert_eq!(Numbers::try_from("Two").unwrap(), Numbers::Two);
//!
//! assert!(Numbers::try_from("Three").is_err());
//! ```
//!
//! ## Custom Prefix and Suffix
//!
//! You can add a prefix and/or suffix to the string representation:
//!
//! ```
//! use enum_stringify::EnumStringify;
//!
//! #[derive(EnumStringify, Debug, PartialEq)]
//! #[enum_stringify(prefix = "Pre", suffix = "Post")]
//! enum Numbers {
//!     One,
//!     Two,
//! }
//!
//! assert_eq!(Numbers::One.to_string(), "PreOnePost");
//! assert_eq!(Numbers::try_from("PreOnePost").unwrap(), Numbers::One);
//! ```
//!
//! ## Case Conversion
//!
//! Convert enum variant names to different cases using the [`convert_case`] crate.
//!
//! ```
//! use enum_stringify::EnumStringify;
//!
//! #[derive(EnumStringify, Debug, PartialEq)]
//! #[enum_stringify(case = "flat")]
//! enum Numbers {
//!     One,
//!     Two,
//! }
//!
//! assert_eq!(Numbers::One.to_string(), "one");
//! assert_eq!(Numbers::try_from("one").unwrap(), Numbers::One);
//! ```
//!
//! ## Rename Variants
//!
//! Customize the string representation of specific variants:
//!
//! ```
//! use enum_stringify::EnumStringify;
//!
//! #[derive(EnumStringify, Debug, PartialEq)]
//! enum Istari {
//!     #[enum_stringify(rename = "Ólorin")]
//!     Gandalf,
//!     Saruman,
//! }
//!
//! assert_eq!(Istari::Gandalf.to_string(), "Ólorin");
//! assert_eq!(Istari::try_from("Ólorin").unwrap(), Istari::Gandalf);
//! ```
//!
//! This takes precedence over the other attributes :
//!
//! ```
//! use enum_stringify::EnumStringify;
//!
//! #[derive(EnumStringify, Debug, PartialEq)]
//! #[enum_stringify(prefix = "Pre", suffix = "Post", case = "upper")]
//! enum Istari {
//!     #[enum_stringify(rename = "Ólorin")]
//!     Gandalf,
//! }
//!
//! assert_eq!(Istari::Gandalf.to_string(), "Ólorin");
//! assert_eq!(Istari::try_from("Ólorin").unwrap(), Istari::Gandalf);
//! ```
//!
//! ## Using All Options Together
//!
//! You can combine all options: renaming, prefix, suffix, and case conversion.
//!
//! ```
//! use enum_stringify::EnumStringify;
//!
//! #[derive(EnumStringify, Debug, PartialEq)]
//! #[enum_stringify(prefix = "Pre", suffix = "Post", case = "upper_flat")]
//! enum Status {
//!     #[enum_stringify(rename = "okay")]
//!     Okk,
//!     Error3,
//! }
//!
//! assert_eq!(Status::Okk.to_string(), "okay");
//! assert_eq!(Status::Error3.to_string(), "PREERROR3POST");
//!
//! assert_eq!(Status::try_from("okay").unwrap(), Status::Okk);
//! assert_eq!(Status::try_from("PREERROR3POST").unwrap(), Status::Error3);
//! ```
//!
//! And using another case :
//!
//!
//! ```
//! use enum_stringify::EnumStringify;
//!
//! #[derive(EnumStringify, Debug, PartialEq)]
//! #[enum_stringify(prefix = "Pre", suffix = "Post", case = "upper")]
//! enum Status {
//!     #[enum_stringify(rename = "okay")]
//!     Okk,
//!     Error3,
//! }
//!
//! assert_eq!(Status::Okk.to_string(), "okay");
//! assert_eq!(Status::Error3.to_string(), "PRE ERROR 3 POST");
//!
//! assert_eq!(Status::try_from("okay").unwrap(), Status::Okk);
//! assert_eq!(Status::try_from("PRE ERROR 3 POST").unwrap(), Status::Error3);
//! ```
//!
//! ## Error Handling
//!
//! When conversion from a string fails, the error type is `String`, containing a descriptive message:
//!
//! ```
//! use enum_stringify::EnumStringify;
//!
//! #[derive(EnumStringify, Debug, PartialEq)]
//! #[enum_stringify(case = "lower")]
//! enum Numbers {
//!     One,
//!     Two,
//! }
//!
//! let result = Numbers::try_from("Three");
//! assert!(result.is_err());
//! assert_eq!(result.unwrap_err(), "Failed to parse string 'Three' for enum Numbers");
//! ```
//!
//! ## Generated Implementations
//!
//! The macro generates the following trait implementations:
//!
//! ```rust, no_run
//! enum Numbers { One, Two }
//!
//! impl ::std::fmt::Display for Numbers {
//!     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//!         match self {
//!             Self::One => write!(f, "One"),
//!             Self::Two => write!(f, "Two"),
//!         }
//!     }
//! }
//!
//! impl TryFrom<&str> for Numbers {
//!     type Error = String;
//!
//!     fn try_from(s: &str) -> Result<Self, Self::Error> {
//!         match s {
//!             "One" => Ok(Self::One),
//!             "Two" => Ok(Self::Two),
//!             _ => Err(format!("Invalid value '{}'", s)),
//!         }
//!     }
//! }
//!
//! impl TryFrom<String> for Numbers {
//!     type Error = String;
//!
//!     fn try_from(s: String) -> Result<Self, Self::Error> {
//!         s.as_str().try_into()
//!     }
//! }
//!
//! impl ::std::str::FromStr for Numbers {
//!     type Err = String;
//!
//!     fn from_str(s: &str) -> Result<Self, Self::Err> {
//!         s.try_into()
//!     }
//! }
//! ```

use attributes::{Attributes, Variants};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod attributes;
mod case;

#[proc_macro_derive(EnumStringify, attributes(enum_stringify))]
pub fn enum_stringify(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_enum_to_string(&ast)
}

/// Generates the implementation of `Display`, `FromStr`, `TryFrom<&str>`, and `TryFrom<String>`
/// for the given enum.
fn impl_enum_to_string(ast: &syn::DeriveInput) -> TokenStream {
    // Extract attributes and variant information from the given AST.
    let attributes = Attributes::new(ast);
    let variants = Variants::new(ast);

    // Apply rename attributes to the enum variants.
    let pairs = variants.apply(&attributes);

    // Extract the enum name.
    let name = &ast.ident;

    let identifiers: Vec<&syn::Ident> = pairs.iter().map(|(i, _)| i).collect();
    let names: Vec<String> = pairs.iter().map(|(_, n)| n.clone()).collect();

    // Generate implementations for each trait
    let mut gen = impl_display(name, &identifiers, &names);
    gen.extend(impl_try_from_str(name, &identifiers, &names));
    gen.extend(impl_try_from_string(name));
    gen.extend(impl_from_str(name));
    gen
}

/// Implementation of [`std::fmt::Display`].
fn impl_display(name: &syn::Ident, identifiers: &[&syn::Ident], names: &[String]) -> TokenStream {
    quote! {
        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #(Self::#identifiers => write!(f, #names),)*
                }
            }
        }
    }
    .into()
}

/// Implementation of [`TryFrom<&str>`].
fn impl_try_from_str(
    name: &syn::Ident,
    identifiers: &[&syn::Ident],
    names: &[String],
) -> TokenStream {
    quote! {
        impl TryFrom<&str> for #name {
            type Error = String;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                match s {
                    #(#names => Ok(Self::#identifiers),)*
                    _ => Err(format!("Failed to parse string '{}' for enum {}", s, stringify!(#name))),
                }
            }
        }
    }
    .into()
}

/// Implementation of [`TryFrom<String>`].
fn impl_try_from_string(name: &syn::Ident) -> TokenStream {
    quote! {
        impl TryFrom<String> for #name {
            type Error = String;

            fn try_from(s: String) -> Result<Self, Self::Error> {
                s.as_str().try_into()
            }
        }
    }
    .into()
}

/// Implementation of [`std::str::FromStr`].
fn impl_from_str(name: &syn::Ident) -> TokenStream {
    quote! {
        impl ::std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.try_into()
            }
        }
    }
    .into()
}
