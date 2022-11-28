use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::FieldsNamed;

use crate::struct_handler::options::ReturnType;
use crate::struct_handler::StructGenerationOptions;
use crate::utils::conversion::ConvertTo;
use crate::utils::generation;

pub fn generate_all(fields: &FieldsNamed, options: &StructGenerationOptions) -> (TokenStream, TokenStream) {
    (
        generate_read_body(fields, options),
        generate_write_body(fields, options),
    )
}

pub fn generate_read_body(fields: &FieldsNamed, options: &StructGenerationOptions) -> TokenStream {
    let operations = {
        let mut reads = Vec::new();

        for field in &fields.named {
            let identifier = field.ident.as_ref().expect("named struct field has no name");

            if options.exclude.contains(&&*identifier.to_string()) {
                continue;
            }

            if options.read_unwrap {
                reads.push(quote! {
                    let #identifier = java_rs_base::io::ClassFilePart::read(reader, ctx)?;
                });
            } else {
                reads.push(quote! {
                    let #identifier = java_rs_base::io::ClassFilePart::read(reader, ctx);
                });
            }
        }

        reads
    };

    match options.read_return_type {
        ReturnType::Nothing => quote! { #(#operations)* },
        ReturnType::OkResult => {
            let return_type = &options.return_type;
            let parameters: Vec<Ident> = fields.custom_into();

            quote! {
                #(#operations)*
                Ok(#return_type { #(#parameters),* })
            }
        }
        ReturnType::EmptyOkResult => quote! { #(#operations)* Ok(()) },
    }
}

pub fn generate_write_body(fields: &FieldsNamed, options: &StructGenerationOptions) -> TokenStream {
    let operations = {
        let mut operations = Vec::new();

        for field in &fields.named {
            let identifier = field.ident.as_ref().expect("named struct field has no name");

            if options.exclude.contains(&&*identifier.to_string()) {
                continue;
            }

            let operation = if options.use_self {
                quote! { self.#identifier.write(writer, ctx)?; }
            } else {
                quote! { #identifier.write(writer, ctx)?; }
            };

            operations.push(operation);
        }

        operations
    };

    generation::generate_struct_write_body(operations, options)
}
