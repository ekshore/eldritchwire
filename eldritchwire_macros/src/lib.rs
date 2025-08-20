use proc_macro2::TokenStream;
use quote::quote;
use syn::{parenthesized, parse_macro_input, DeriveInput, Error, Ident, Result};

#[proc_macro_derive(CommandGroup, attributes(parameter, data_type, command))]
pub fn command_group(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let commands: Result<Vec<_>> = if let syn::Data::Enum(data) = &input.data {
        data.variants.iter().map(handle_variant_attr).collect()
    } else {
        Err(Error::new_spanned(&input, "CommandGroup must be an enum"))
    };

    let parse_command_fn = build_parse_command_fn(name, &commands.expect("Commands are expected"));

    quote! {
        #parse_command_fn
    }
    .into()
}

fn build_parse_command_fn(name: &Ident, commands: &Vec<CommandMetaData>) -> TokenStream {
    let match_branches: Vec<proc_macro2::TokenStream> = commands
        .iter()
        .map(|variant| {
            let variant_name = variant.name;
            let param = &variant.parameter;
            let variant_parser = build_variant_parser(name, variant);
            let arm_return = if variant.data_type.is_some() {
                quote! { #variant_parser, }
            } else {
                quote! { Ok(#name::#variant_name), }
            };
            quote! { #param => #arm_return }
        })
        .collect();

    quote! {
        pub fn parse_command(command_data: CommandData) -> Result<#name, EldritchError> {
            match command_data.parameter() {
                #(#match_branches)*
                _ => Err(EldritchError::InvalidCommandData),
            }
        }
    }
}

fn build_variant_parser(name: &Ident, command: &CommandMetaData) -> TokenStream {
    let command_name = &command.name;
    let data_type = command.data_type;
    let data = match data_type {
        Some(0x00) => {
            return quote! {
                Ok(#name::#command_name(
                        if *command_data.operation() == 0 {
                            Operation::Assign
                        } else {
                            return Err(EldritchError::InvalidCommandData);
                        },
                        command_data.data_buff()[0] != 0,
                ))
            }
        }
        Some(0x01) => quote! { let data = i8::from_le_bytes(data); },
        Some(0x02) => quote! { let data = i16::from_le_bytes(data); },
        Some(0x03) => quote! { let data = i32::from_le_bytes(data); },
        Some(0x04) => quote! { let data = i64::from_le_bytes(data); },
        Some(0x05) => todo!("String"),
        Some(0x80) => quote! { let data = FixedPointDecimal::from_data(data); },
        Some(_) => todo!("Unknown data type"),
        None => return quote! {},
    };

    #[cfg(feature = "bounds-checked")]
    let bounds_check = if let Some(bounds) = &command.bounds {
        match data_type {
            Some(0x00) => todo!("Bounds, not supported on Bool"),
            Some(0x01) => {
                let lower = if let Some(lower) = &bounds.lower {
                    if let syn::Lit::Int(constraint) = lower {
                        constraint
                            .base10_parse::<i8>()
                            .expect("Should be a valid i8")
                    } else {
                        panic!("Inalid state");
                    }
                } else {
                    i8::MIN
                };

                let upper = if let Some(upper) = &bounds.upper {
                    if let syn::Lit::Int(constraint) = upper {
                        constraint
                            .base10_parse::<i8>()
                            .expect("Should be a valid i8")
                    } else {
                        panic!("Invalid state");
                    }
                } else {
                    i8::MAX
                };

                quote! {
                    if !(#lower..=#upper).contains(&data) {
                        Err(EldritchError::DataOutOfBounds)
                    } else
                }
            }
            Some(0x02) => {
                let lower = if let Some(lower) = &bounds.lower {
                    if let syn::Lit::Int(constraint) = lower {
                        constraint
                            .base10_parse::<i16>()
                            .expect("Should be a valid i16")
                    } else {
                        panic!("Inalid state");
                    }
                } else {
                    i16::MIN
                };

                let upper = if let Some(upper) = &bounds.upper {
                    if let syn::Lit::Int(constraint) = upper {
                        constraint
                            .base10_parse::<i16>()
                            .expect("Should be a valid i16")
                    } else {
                        panic!("Invalid state");
                    }
                } else {
                    i16::MAX
                };

                quote! {
                    if !(#lower..=#upper).contains(&data) {
                        Err(EldritchError::DataOutOfBounds)
                    } else
                }
            }
            Some(0x03) => {
                let lower = if let Some(lower) = &bounds.lower {
                    if let syn::Lit::Int(constraint) = lower {
                        constraint
                            .base10_parse::<i32>()
                            .expect("Should be a valid i32")
                    } else {
                        panic!("Inalid state");
                    }
                } else {
                    i32::MIN
                };

                let upper = if let Some(upper) = &bounds.upper {
                    if let syn::Lit::Int(constraint) = upper {
                        constraint
                            .base10_parse::<i32>()
                            .expect("Should be a valid i32")
                    } else {
                        panic!("Invalid state");
                    }
                } else {
                    i32::MAX
                };

                quote! {
                    if !(#lower..=#upper).contains(&data) {
                        Err(EldritchError::DataOutOfBounds)
                    } else
                }
            }
            Some(0x04) => {
                let lower = if let Some(lower) = &bounds.lower {
                    if let syn::Lit::Int(constraint) = lower {
                        constraint
                            .base10_parse::<i64>()
                            .expect("Should be a valid i64")
                    } else {
                        panic!("Inalid state");
                    }
                } else {
                    i64::MIN
                };

                let upper = if let Some(upper) = &bounds.upper {
                    if let syn::Lit::Int(constraint) = upper {
                        constraint
                            .base10_parse::<i64>()
                            .expect("Should be a valid i64")
                    } else {
                        panic!("Invalid state");
                    }
                } else {
                    i64::MAX
                };

                quote! {
                    if !(#lower..=#upper).contains(&data) {
                        Err(EldritchError::DataOutOfBounds)
                    } else
                }
            }
            Some(0x05) => todo!("String"),
            Some(0x80) => {
                let lower = if let Some(lower) = &bounds.lower {
                    if let syn::Lit::Float(constraint) = lower {
                        constraint
                            .base10_parse::<f32>()
                            .expect("Should be a valid f32")
                    } else {
                        panic!("Inalid state");
                    }
                } else {
                    -16.0
                };

                let upper = if let Some(upper) = &bounds.upper {
                    if let syn::Lit::Float(constraint) = upper {
                        constraint
                            .base10_parse::<f32>()
                            .expect("Should be a valid f32")
                    } else {
                        panic!("Invalid state");
                    }
                } else {
                    15.9995
                };

                quote! {
                    if !(#lower..=#upper).contains(&data.get_real_val()) {
                        Err(EldritchError::DataOutOfBounds)
                    } else
                }
            },
            Some(_) => todo!(),
            None => return quote! {},
        }
    } else {
        quote! {}
    };

    let inc_or_toggle = if let Some(data_type) = command.data_type {
        if data_type == 0x00 {
            quote! { Operation::Toggle }
        } else {
            quote! { Operation::Increment }
        }
    } else {
        quote! {}
    };

    quote! {
        if *command_data.data_type() == #data_type {
            if let Ok(data) = command_data.data_buff().try_into() {
                #data
                #bounds_check {
                    Ok(#name::#command_name(
                        if *command_data.operation() == 0 {
                            Operation::Assign
                        } else {
                            #inc_or_toggle
                        },
                        data
                    ))
                }
            } else {
                Err(EldritchError::InvalidCommandData)
            }
        } else {
            Err(EldritchError::InvalidCommandData)
        }
    }
}

#[cfg(feature = "bounds-checked")]
#[derive(Debug, PartialEq)]
struct DataBounds {
    upper: Option<syn::Lit>,
    lower: Option<syn::Lit>,
}

#[derive(Debug, PartialEq)]
struct CommandMetaData<'a> {
    name: &'a Ident,
    parameter: u8,
    data_type: Option<u8>,
    #[cfg(feature = "bounds-checked")]
    bounds: Option<DataBounds>,
}

#[derive(Default)]
struct CommandMetaDataBuilder<'a> {
    name: Option<&'a Ident>,
    parameter: Option<u8>,
    data_type: Option<u8>,
    #[cfg(feature = "bounds-checked")]
    bounds: Option<DataBounds>,
}

impl CommandMetaData<'_> {
    pub fn builder() -> CommandMetaDataBuilder<'static> {
        CommandMetaDataBuilder::default()
    }
}

impl<'a> CommandMetaDataBuilder<'a> {
    pub fn name(mut self, name: &'a Ident) -> Self {
        self.name = Some(name);
        self
    }

    pub fn parameter(mut self, parameter: u8) -> Self {
        self.parameter = Some(parameter);
        self
    }

    pub fn data_type(mut self, data_type: Option<u8>) -> Self {
        self.data_type = data_type;
        self
    }

    #[cfg(feature = "bounds-checked")]
    pub fn bounds(mut self, bounds: DataBounds) -> Self {
        self.bounds = if bounds.upper.is_some() || bounds.lower.is_some() {
            Some(bounds)
        } else {
            None
        };
        self
    }

    pub fn build(self) -> Result<CommandMetaData<'a>> {
        Ok(CommandMetaData {
            name: self.name.expect("name is required"),
            parameter: self.parameter.expect("parameter is required"),
            data_type: self.data_type,
            #[cfg(feature = "bounds-checked")]
            bounds: self.bounds,
        })
    }
}

fn handle_variant_attr(variant: &syn::Variant) -> Result<CommandMetaData<'_>> {
    let mut parameter = 0;
    let mut data_type = None;

    #[cfg(feature = "bounds-checked")]
    let mut bounds = DataBounds {
        lower: None,
        upper: None,
    };

    for attr in &variant.attrs {
        if attr.path().is_ident("command") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("parameter") {
                    let content;
                    parenthesized!(content in meta.input);
                    let lit: syn::LitInt = content.parse()?;
                    let val: u8 = lit.base10_parse()?;
                    parameter = val;
                }

                if meta.path.is_ident("data_type") {
                    let content;
                    parenthesized!(content in meta.input);
                    let lit: syn::LitInt = content.parse()?;
                    let val: u8 = lit.base10_parse()?;
                    data_type = Some(val);
                }

                #[cfg(feature = "bounds-checked")]
                if meta.path.is_ident("bounds") {
                    meta.parse_nested_meta(|inner_meta| {
                        if inner_meta.path.is_ident("lower") {
                            let content;
                            parenthesized!(content in inner_meta.input);
                            let lit: syn::Lit = content.parse()?;
                            bounds.lower = Some(lit);
                        }

                        if inner_meta.path.is_ident("upper") {
                            let content;
                            parenthesized!(content in inner_meta.input);
                            let lit: syn::Lit = content.parse()?;
                            bounds.upper = Some(lit);
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })?;
        }
    }
    let builder = CommandMetaData::builder()
        .name(&variant.ident)
        .parameter(parameter)
        .data_type(data_type);

    #[cfg(feature = "bounds-checked")]
    let builder = builder.bounds(bounds);
    builder.build()
}

#[cfg(test)]
mod macro_tests {
    use super::*;
    use proc_macro2::Span;
    use syn::parse_quote;

    #[test]
    fn handle_variant_single_attr_test() {
        let input: syn::ItemEnum = parse_quote! {
            enum LensCommands {
                #[command(parameter(0x00), data_type(128))]
                Focus(Operation, FixedPointDecimal),
            }
        };

        let variant = input.variants.get(0).unwrap();
        let output = handle_variant_attr(variant);

        assert_eq!(
            output.unwrap(),
            CommandMetaData {
                name: &Ident::new("Focus", Span::call_site()),
                parameter: 0,
                data_type: Some(128),
                #[cfg(feature = "bounds-checked")]
                bounds: None,
            }
        );
    }

    #[test]
    fn handle_variant_single_attr_order_test() {
        let input: syn::ItemEnum = parse_quote! {
            enum LensCommands {
                #[command(data_type(128), parameter(0x00))]
                Focus(Operation, FixedPointDecimal),
            }
        };

        let variant = input.variants.get(0).unwrap();
        let output = handle_variant_attr(variant);

        assert_eq!(
            output.unwrap(),
            CommandMetaData {
                name: &Ident::new("Focus", Span::call_site()),
                parameter: 0,
                data_type: Some(128),
                #[cfg(feature = "bounds-checked")]
                bounds: None,
            }
        );
    }
}
