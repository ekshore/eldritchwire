use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

struct CommandMetaData {
    name: String,
    parameter: u8,
    data_type: Option<u8>,
}

#[proc_macro_derive(CommandGroup, attributes(parameter, data_type))]
pub fn command_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let commands = if let syn::Data::Enum(data) = input.data {
        let variants: Vec<syn::Variant> = data
            .variants
            .iter()
            .map(|variant| {
                let name = variant.ident.clone();
                let attrs: Vec<_> = variant
                    .attrs
                    .clone()
                    .into_iter()
                    .filter(|attr| attr.meta.require_name_value().is_ok())
                    .map(|attr| {
                        let key = attr.path().clone();
                        let val: u8 = if let syn::Meta::NameValue(name) = attr.meta.clone() {
                            if let syn::Expr::Lit(val) = name.value {
                                if let syn::Lit::Int(val) = val.lit {
                                    val.base10_parse().unwrap()
                                } else {
                                    panic!("We shouldn't be here");
                                }
                            } else {
                                panic!("We shouldn't be here");
                            }
                        } else {
                            panic!("We shouldn't be here");
                        };
                        (key, val)
                    })
                    .collect();
                panic!(
                    "Variant Name: {}, Attributes: {}, Value: {}",
                    name,
                    attrs.get(1).unwrap().0.get_ident().unwrap(),
                    attrs.get(1).unwrap().1
                );
            })
            .collect();
    } else {
        panic!("CommandGroup can only be derived on enums");
    };

    TokenStream::from(quote!())
}
