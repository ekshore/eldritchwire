use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error, Result};


#[proc_macro_derive(CommandGroup, attributes(parameter, data_type))]
pub fn command_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let commands:Result<Vec<_>> = if let syn::Data::Enum(data) = input.data {
         data
            .variants
            .iter()
            .map(|variant| handle_variant(variant))
            .collect()
    } else {
        Err(Error::new_spanned(input, "CommandGroup must be an enum"))
    };

    panic!("Commands {:#?}", commands);

    TokenStream::from(quote!())
}

#[derive(Debug, PartialEq)]
enum CommandAttribute {
    Parameter(u8),
    DataType(u8),
}

#[derive(Debug)]
struct CommandMetaData {
    name: String,
    parameter: u8,
    data_type: Option<u8>,
}

#[derive(Default)]
struct CommandMetaDataBuilder {
    name: Option<String>,
    parameter: Option<u8>,
    data_type: Option<u8>,
}

impl CommandMetaData {
    pub fn builder() -> CommandMetaDataBuilder {
        CommandMetaDataBuilder::default()
    }
}

impl CommandMetaDataBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn parameter(mut self, parameter: u8) -> Self {
        self.parameter = Some(parameter);
        self
    }

    pub fn data_type(mut self, data_type: u8) -> Self {
        self.data_type = Some(data_type);
        self
    }

    pub fn build(self) -> Result<CommandMetaData> {
        Ok(CommandMetaData {
            name: self.name.unwrap(),
            parameter: self.parameter.unwrap(),
            data_type: self.data_type,
        })
    }
}
fn handle_variant(variant: &syn::Variant) -> Result<CommandMetaData> {
    let builder = CommandMetaData::builder()
        .name(variant.ident.to_string());
    let attrs: Result<Vec<_>> = variant.attrs
        .iter()
        .map(|attr| handle_attr(attr))
        .collect();

    let builder = attrs?.iter().fold(builder, |builder, attr| { 
            match attr {
                CommandAttribute::Parameter(val) => builder.parameter(*val),
                CommandAttribute::DataType(val) => builder.data_type(*val),
            }
        });

    builder.build()
}

fn handle_attr(attr: &syn::Attribute) -> Result<CommandAttribute> {
    match &attr.meta {
        syn::Meta::NameValue(val) if attr.path().is_ident("parameter") => {
            Ok(CommandAttribute::Parameter(parse_int_attr_val(val)?))
        }
        syn::Meta::NameValue(val) if attr.path().is_ident("data_type") => {
            Ok(CommandAttribute::DataType(parse_int_attr_val(val)?))
        }
        _ => todo!(),
    }
}

fn parse_int_attr_val(val: &syn::MetaNameValue) -> Result<u8> {
    if let syn::Expr::Lit(val) = &val.value {
        if let syn::Lit::Int(val) = &val.lit {
            Ok(val.base10_parse()?)
        } else {
            Err(Error::new_spanned(
                val,
                "Attribute Value must be an Integer",
            ))
        }
    } else {
        Err(Error::new_spanned(
            val,
            "Attribute Value must be an expression",
        ))
    }
}
