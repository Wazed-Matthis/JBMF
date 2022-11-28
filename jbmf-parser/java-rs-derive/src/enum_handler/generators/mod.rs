use proc_macro2::TokenStream;
use syn::{DataEnum, DeriveInput};

pub(crate) trait EnumGenerator {
    fn generate(input: &DeriveInput, data: &DataEnum) -> TokenStream;
}

pub(crate) mod attribute;
pub(crate) mod code;
