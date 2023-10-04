use syn::{DeriveInput, Meta};

pub(crate) enum Case {
    Camel,
    Snake,
    None,
}

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

        ast.attrs.iter().for_each(|attr| match &attr.meta {
            Meta::Path(_) => {
                panic!("Unexpected argument");
            }
            Meta::List(list) => {
                let path = list
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>();

                if path == vec!["enum_stringify"] {
                    let mut tokens = list.tokens.clone().into_iter();

                    while let Some(attribute_type) = tokens.next() {
                        let attribute_type = attribute_type.to_string();

                        if tokens.next().expect("type must be specified").to_string() != "=" {
                            panic!("too many arguments");
                        }
                        let value = tokens.next().expect("value must be specified").to_string();

                        match attribute_type.as_str() {
                            "case" => {
                                let case = match value.as_str() {
                                    "camel" => Case::Camel,
                                    "snake" => Case::Snake,
                                    _ => Case::None,
                                };
                                new.case = Some(case);
                            }
                            "prefix" => {
                                new.prefix = Some(value);
                            }
                            "suffix" => {
                                new.suffix = Some(value);
                            }
                            _ => {
                                panic!("Attribute not supported");
                            }
                        }

                        if let Some(comma_separator) = tokens.next() {
                            if comma_separator.to_string() != "," {
                                panic!("Expected a commaseparated attribute list");
                            }
                        }
                    }
                }
            }
            Meta::NameValue(_) => {
                panic!("Unexpected argument");
            }
        });

        new
    }

    pub(crate) fn apply(&self, names: &Vec<&syn::Ident>) -> Vec<syn::Ident> {
        let mut new_names = Vec::new();
        for name in names {
            let mut new_name = String::new();
            if let Some(prefix) = &self.prefix {
                new_name.push_str(prefix);
            }

            // Add here case logic
            new_name.push_str(&name.to_string());

            if let Some(suffix) = &self.suffix {
                new_name.push_str(suffix);
            }
            new_names.push(syn::Ident::new(&new_name, name.span()));
        }
        new_names
    }
}
