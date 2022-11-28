use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, Fields};

use crate::utils::generation;
pub use options::ReturnType;
pub use options::StructGenerationOptions;

pub mod named;
pub mod unnamed;

mod options;

pub fn generate(input: &DeriveInput, data: &DataStruct) -> TokenStream {
    let name = &input.ident;

    let options = {
        let mut options = StructGenerationOptions::new();
        options.return_type(quote! { #name });
        options
    };

    let (read_body, write_body) = match &data.fields {
        Fields::Named(fields) => named::generate_all(fields, &options),
        Fields::Unnamed(fields) => unnamed::generate_all(fields, &options),
        Fields::Unit => panic!("cannot derive a struct with unit fields"),
    };

    generation::generate_class_file_part_impl(name, read_body, write_body)
}
