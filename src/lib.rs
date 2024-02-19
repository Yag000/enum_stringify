//! # enum-stringify
//!
//! Derive [`std::fmt::Display`], [`std::str::FromStr`], [`TryFrom<&str>`] and
//! [`TryFrom<String>`] with a simple derive macro: [`EnumStringify`].

use attributes::Attributes;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod attributes;

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
/// # Prefix and suffix
///
/// You can add a prefix and/or a suffix to the string representation of the
/// enum variants.
///
/// ```
/// use enum_stringify::EnumStringify;
/// use std::str::FromStr;
///
/// #[derive(EnumStringify, Debug, PartialEq)]
/// #[enum_stringify(prefix = "MyPrefix", suffix = "MySuffix")]
/// enum Numbers {
///   One,
///   Two,
/// }
///
/// assert_eq!(Numbers::One.to_string(), "MyPrefixOneMySuffix");
/// assert_eq!(Numbers::Two.to_string(), "MyPrefixTwoMySuffix");
///
/// assert_eq!(Numbers::try_from("MyPrefixOneMySuffix").unwrap(), Numbers::One);
/// assert_eq!(Numbers::try_from("MyPrefixTwoMySuffix").unwrap(), Numbers::Two);
/// ```
///
/// # Case
///
/// You can also set the case ("lower" or "upper") of the string representation of the enum variants.
///
/// ```
/// use enum_stringify::EnumStringify;
/// use std::str::FromStr;
///
/// #[derive(EnumStringify, Debug, PartialEq)]
/// #[enum_stringify(case = "lower")]
/// enum Numbers {
///  One,
///  Two,
/// }
///
/// assert_eq!(Numbers::One.to_string(), "one");
/// assert_eq!(Numbers::Two.to_string(), "two");
///
/// assert_eq!(Numbers::try_from("one").unwrap(), Numbers::One);
/// assert_eq!(Numbers::try_from("two").unwrap(), Numbers::Two);
/// ```
///
/// # Rename variants
///
/// You can rename the variants of the enum.
/// This is useful if you want to have a different name for the enum variants
/// and the string representation of the enum variants.
///
/// ```
/// use enum_stringify::EnumStringify;
/// use std::str::FromStr;
///
/// #[derive(EnumStringify, Debug, PartialEq)]
/// enum Istari {
///  #[enum_stringify(rename = "Ólorin")]
///  Gandalf,
///  Saruman,
///  Radagast,
///  Alatar,
///  Pallando,
/// }
///
/// assert_eq!(Istari::Gandalf.to_string(), "Ólorin");
/// assert_eq!(Istari::Saruman.to_string(), "Saruman");
/// assert_eq!(Istari::Radagast.to_string(), "Radagast");
///
/// assert_eq!(Istari::try_from("Ólorin").unwrap(), Istari::Gandalf);
/// assert_eq!(Istari::try_from("Saruman").unwrap(), Istari::Saruman);
/// assert_eq!(Istari::try_from("Radagast").unwrap(), Istari::Radagast);
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

#[proc_macro_derive(EnumStringify, attributes(enum_stringify))]
pub fn enum_stringify(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_enum_to_string(&ast)
}

fn impl_enum_to_string(ast: &syn::DeriveInput) -> TokenStream {
    let attributes = Attributes::new(ast);
    let name = &ast.ident;
    let coplues = attributes.apply();

    let identifiers: Vec<&syn::Ident> = coplues.iter().map(|(i, _)| i).collect();
    let names: Vec<syn::Ident> = coplues.iter().map(|(_, n)| n.clone()).collect();

    let mut gen = impl_display(name, &identifiers, &names);
    gen.extend(impl_from_str(name, &identifiers, &names));
    gen.extend(impl_from_string(name));
    gen.extend(impl_from_str_trait(name));

    gen
}

/// Implementation of [`std::fmt::Display`].
fn impl_display(
    name: &syn::Ident,
    identifiers: &Vec<&syn::Ident>,
    names: &Vec<syn::Ident>,
) -> TokenStream {
    let gen = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #(Self::#identifiers=> write!(f, stringify!(#names))),*
                }
            }
        }
    };

    gen.into()
}

/// Implementation of [`TryFrom<&str>`].
fn impl_from_str(
    name: &syn::Ident,
    identifiers: &Vec<&syn::Ident>,
    names: &Vec<syn::Ident>,
) -> TokenStream {
    let gen = quote! {
        impl TryFrom<&str> for #name {
            type Error = ();

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                match s {
                    #(stringify!(#names) => Ok(Self::#identifiers),)*
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
