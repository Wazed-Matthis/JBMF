use crate::struct_handler::options::ReturnType;
use crate::struct_handler::StructGenerationOptions;
use crate::utils::conversion::ConvertTo;
use crate::utils::generation;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{FieldsUnnamed, Index, Member};

pub(crate) fn generate_all(fields: &FieldsUnnamed, options: &StructGenerationOptions) -> (TokenStream, TokenStream) {
    (
        generate_read_body(fields, options),
        generate_write_body(fields, options),
    )
}

pub(crate) fn generate_read_body(fields: &FieldsUnnamed, options: &StructGenerationOptions) -> TokenStream {
    let mut operations = Vec::new();

    for i in 1..fields.unnamed.len() + 1 {
        let identifier = quote::format_ident!("f{}", i);

        operations.push(quote! {
            let #identifier = java_rs_base::io::ClassFilePart::read(reader, ctx)?;
        });
    }

    match options.read_return_type {
        ReturnType::Nothing => quote! { #(#operations)* },
        ReturnType::OkResult => {
            let return_type = &options.return_type;
            let parameters: Vec<Ident> = fields.custom_into();

            quote! {
                #(#operations)*
                Ok(#return_type ( #(#parameters),* ))
            }
        }
        ReturnType::EmptyOkResult => quote! { #(#operations)* Ok(()) },
    }
}

pub(crate) fn generate_write_body(fields: &FieldsUnnamed, options: &StructGenerationOptions) -> TokenStream {
    let mut operations = Vec::new();

    for i in 0..fields.unnamed.len() {
        if options.use_self {
            let identifier = Member::Unnamed(Index {
                index: i as u32,
                span: Span::call_site(),
            });

            operations.push(quote! {
                self.#identifier.write(writer, ctx)?;
            });
        } else {
            let identifier = quote::format_ident!("f{}", i + 1);

            operations.push(quote! {
                #identifier.write(writer, ctx)?;
            });
        }
    }

    generation::generate_struct_write_body(operations, options)
}
