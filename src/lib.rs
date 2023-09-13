//! # enum-stringify
//!
//! Derive [`std::fmt::Display`], [`std::str::FromStr`], [`TryFrom<&str>`] and
//! [`TryFrom<String>`] with a simple derive macro: [`EnumStringify`].

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use proc_macro::TokenStream;
use quote::quote;

/// Derive [`std::fmt::Display`], [`std::str::FromStr`], [`TryFrom<&str>`] and
/// [`TryFrom<String>`] for an enum.
///
/// They simply take the name of the enum variant and convert it to a string.
///
/// # Example
///
/// ```
/// use enum_stringify::EnumStringify;
/// use std::str::FromStr;
///
/// #[derive(EnumStringify, Debug, PartialEq)]
/// enum Numbers {
///    One,
///    Two,
/// }
///
/// assert_eq!(Numbers::One.to_string(), "One");
/// assert_eq!(Numbers::Two.to_string(), "Two");
///
///
/// assert_eq!(Numbers::try_from("One").unwrap(), Numbers::One);
/// assert_eq!(Numbers::try_from("Two").unwrap(), Numbers::Two);
///
/// assert!(Numbers::try_from("Three").is_err());
/// ```
///
/// # Details
///
/// The implementations of the above traits corresponds to this:
///
/// ```rust, no_run
/// enum Numbers {
///   One,
///   Two,
/// }
///
/// impl std::fmt::Display for Numbers {
///     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
///         match self {
///             Self::One => write!(f, "One"),
///             Self::Two => write!(f, "Two"),
///         }
///     }
/// }
///
/// impl TryFrom<&str> for Numbers {
///     type Error = ();
///
///     fn try_from(s: &str) -> Result<Self, Self::Error> {
///         match s {    
///             "One" => Ok(Self::One),
///             "Two" => Ok(Self::Two),
///             _ => Err(()),
///         }
///     }
/// }
///
/// impl TryFrom<String> for Numbers {
///     type Error = ();
///         
///     fn try_from(s: String) -> Result<Self, Self::Error> {
///         s.as_str().try_into()
///     }
/// }
///
/// impl std::str::FromStr for Numbers {
///     type Err = ();
///
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         s.try_into()
///     }
/// }
/// ```
#[proc_macro_derive(EnumStringify)]
pub fn enum_stringify(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_enum_to_string(&ast)
}

fn impl_enum_to_string(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let variants = match ast.data {
        syn::Data::Enum(ref e) => &e.variants,
        _ => panic!("EnumToString only works with Enums"),
    };

    let names = variants.iter().map(|v| &v.ident).collect::<Vec<_>>();

    let mut gen = impl_display(name, &names);
    gen.extend(impl_from_str(name, &names));
    gen.extend(impl_from_string(name));
    gen.extend(impl_from_str_trait(name));

    gen
}

/// Implementation of [`std::fmt::Display`].
fn impl_display(name: &syn::Ident, names: &Vec<&syn::Ident>) -> TokenStream {
    let gen = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #(Self::#names => write!(f, stringify!(#names))),*
                }
            }
        }
    };

    gen.into()
}

/// Implementation of [`TryFrom<&str>`].
fn impl_from_str(name: &syn::Ident, names: &Vec<&syn::Ident>) -> TokenStream {
    let gen = quote! {
        impl TryFrom<&str> for #name {
            type Error = ();

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                match s {
                    #(stringify!(#names) => Ok(Self::#names),)*
                    _ => Err(()),
                }
            }
        }
    };

    gen.into()
}

/// Implementation of [`TryFrom<String>`].
fn impl_from_string(name: &syn::Ident) -> TokenStream {
    let gen = quote! {
        impl TryFrom<String> for #name {
            type Error = ();

            fn try_from(s: String) -> Result<Self, Self::Error> {
                s.as_str().try_into()
            }
        }
    };

    gen.into()
}

/// Implementation of [`std::str::FromStr`].
fn impl_from_str_trait(name: &syn::Ident) -> TokenStream {
    let gen = quote! {
        impl std::str::FromStr for #name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.try_into()
            }
        }
    };

    gen.into()
}
