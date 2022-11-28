use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::struct_handler::ReturnType;
use crate::struct_handler::StructGenerationOptions;

pub(crate) fn generate_class_file_part_impl(
    name: &Ident,
    read_body: TokenStream,
    write_body: TokenStream,
) -> TokenStream {
    quote! {
        impl java_rs_base::io::ClassFilePart for #name {
            fn read<R: std::io::Read>(reader: &mut R, ctx: &java_rs_base::io::ReadContext) -> Result<Self, java_rs_base::error::Error> where Self: std::marker::Sized {
                #read_body
            }
            fn write<W: std::io::Write>(&self, writer: &mut W, ctx: &java_rs_base::io::WriteContext) -> Result<(), java_rs_base::error::Error> {
                #write_body
            }
        }
    }
}

pub(crate) fn generate_struct_write_body(
    operations: Vec<TokenStream>,
    options: &StructGenerationOptions,
) -> TokenStream {
    match options.write_return_type {
        ReturnType::Nothing => quote! {
            #(#operations)*
        },
        ReturnType::OkResult => quote! {
            #(#operations)*
            Ok(self)
        },
        ReturnType::EmptyOkResult => quote! {
            #(#operations)*
            Ok(())
        },
    }
}
