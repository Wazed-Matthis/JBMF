use proc_macro::TokenStream;

use syn::{parse_macro_input, Data, DeriveInput};

mod attribute;
mod enum_handler;
mod struct_handler;
mod symbol;
mod utils;

#[proc_macro_derive(ClassFilePart, attributes(java_rs))]
#[proc_macro_error::proc_macro_error]
pub fn derive(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    TokenStream::from(match &input.data {
        Data::Struct(data) => struct_handler::generate(&input, data),
        Data::Enum(data) => enum_handler::generate(&input, data),
        Data::Union(_data) => unimplemented!("union types are unsupported"),
    })
}
