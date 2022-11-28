use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, DeriveInput, Fields, Path, Variant};

use crate::attribute::JavaRsAttribute;
use crate::enum_handler::generators::EnumGenerator;
use crate::struct_handler;
use crate::struct_handler::StructGenerationOptions;
use crate::utils::conversion::ConvertTo;
use crate::utils::generation;

pub struct CodeGenerator;

impl EnumGenerator for CodeGenerator {
    fn generate(input: &DeriveInput, data: &DataEnum) -> TokenStream {
        generation::generate_class_file_part_impl(&input.ident, generate_read_body(data), generate_write_body(data))
    }
}

pub struct CodeOptions {
    // TODO: This is not optional
    opcode: u8,
    io_implementation: Option<Path>,
}

fn generate_read_body(data: &DataEnum) -> TokenStream {
    let cases = generate_cases(data, |variant| generate_read_case(variant));

    quote! {
        let opcode: u8 = java_rs_base::io::ClassFilePart::read(reader, ctx)?;

        match opcode {
            #(#cases)*
            _ => unimplemented!("opcode unknown: {}", opcode)
        }
    }
}

fn generate_read_case(variant: &Variant) -> TokenStream {
    let options = derive_options(variant);
    let opcode = options.opcode;

    let body = generate_read_case_body(variant);

    if let Some(io_implementation) = options.io_implementation {
        quote! {
            #opcode => #io_implementation::read(reader, ctx),
        }
    } else {
        quote! {
            #opcode => {
                #body
            }
        }
    }
}

fn generate_read_case_body(variant: &Variant) -> TokenStream {
    let ident = &variant.ident;
    let options = {
        let mut options = StructGenerationOptions::new();
        options.return_type(quote! { Self::#ident });
        options
    };

    match &variant.fields {
        Fields::Named(fields) => struct_handler::named::generate_read_body(fields, &options),
        Fields::Unnamed(fields) => struct_handler::unnamed::generate_read_body(fields, &options),
        Fields::Unit => quote! { Ok(Self::#ident) },
    }
}

fn generate_write_body(data: &DataEnum) -> TokenStream {
    let cases = generate_cases(data, |variant| generate_write_case(variant));

    quote! {
        match self {
            #(#cases)*
        }
    }
}

fn generate_write_case(variant: &Variant) -> TokenStream {
    let parameters = generate_parameters(&variant.fields);
    let body = generate_write_case_body(variant);
    let ident = &variant.ident;

    quote! {
        Self::#ident #parameters => {
            #body
        }
    }
}

fn generate_write_case_body(variant: &Variant) -> TokenStream {
    let options = derive_options(variant);
    let opcode = options.opcode;

    if let Some(io_implementation) = options.io_implementation {
        quote! {
            #opcode.write(writer, ctx)?;
            #io_implementation::write(self, writer, ctx)?;
            Ok(())
        }
    } else {
        let options = {
            let mut options = StructGenerationOptions::new();
            options.use_self(false);
            options
        };

        let main = match &variant.fields {
            Fields::Named(fields) => struct_handler::named::generate_write_body(fields, &options),
            Fields::Unnamed(fields) => struct_handler::unnamed::generate_write_body(fields, &options),
            Fields::Unit => quote! { Ok(()) },
        };

        quote! {
            #opcode.write(writer, ctx)?;
            #main
        }
    }
}

fn generate_parameters(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(fields) => {
            let parameters = fields.custom_into();
            quote! { { #(#parameters),* } }
        }
        Fields::Unnamed(fields) => {
            let parameters = fields.custom_into();
            quote! { ( #(#parameters),* ) }
        }
        Fields::Unit => TokenStream::new(),
    }
}

fn generate_cases<F: Fn(&Variant) -> TokenStream>(data: &DataEnum, callback: F) -> Vec<TokenStream> {
    let mut cases = Vec::new();

    for variant in &data.variants {
        cases.push(callback(variant));
    }

    cases
}

fn derive_options(variant: &Variant) -> CodeOptions {
    let attributes = JavaRsAttribute::from_attributes(&variant.attrs);

    let mut opcode = None;
    let mut io_implementation = None;

    // TODO:Emit proper error if opcode is missing or invalid attributes are being used
    for attribute in attributes {
        match attribute {
            JavaRsAttribute::OpCode { value, .. } => opcode = Some(value),
            JavaRsAttribute::IOImplementation { value, .. } => io_implementation = Some(value),
            _ => {}
        }
    }

    CodeOptions {
        opcode: opcode.unwrap(),
        io_implementation,
    }
}
