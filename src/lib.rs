use proc_macro::TokenStream;
use quote::quote;

/// Derive the [`std::fmt::Display`] trait for an enum.
///
/// This macro will derive the [`std::fmt::Display`] trait for an enum. The
/// trait will print the name of the variant.
///
/// # Example
///
/// ```
/// use enum_string::EnumToString;
///
/// #[derive(EnumToString)]
/// enum Numbers {
///    One,
///    Two,
/// }
///
/// assert_eq!(Numbers::One.to_string(), "One");
/// assert_eq!(Numbers::Two.to_string(), "Two");
/// ```
///
#[proc_macro_derive(EnumToString)]
pub fn enum_to_string(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_enum_to_string(&ast)
}

fn impl_enum_to_string(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let variants = match ast.data {
        syn::Data::Enum(ref e) => &e.variants,
        _ => panic!("EnumToString only works with Enums"),
    };

    let names = variants.iter().map(|v| &v.ident);

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
