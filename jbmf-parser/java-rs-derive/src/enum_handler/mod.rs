use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{DataEnum, DeriveInput};

use crate::attribute::JavaRsAttribute;
use crate::enum_handler::generators::attribute::AttributeGenerator;
use crate::enum_handler::generators::code::CodeGenerator;
use crate::enum_handler::generators::EnumGenerator;

mod generators;

pub fn generate(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    let attributes: Vec<JavaRsAttribute> = JavaRsAttribute::from_attributes(&input.attrs);

    let generator = {
        let mut generator = None;

        for attribute in attributes {
            if let JavaRsAttribute::Generator { value, span } = attribute {
                generator = Some((value, span))
            }
        }

        generator
    };

    if let Some(generator) = generator {
        match generator.0.as_str() {
            "attribute" => AttributeGenerator::generate(input, data),
            "code" => CodeGenerator::generate(input, data),
            "instruction_index" => {
                proc_macro_error::abort!(input.span(), "instruction index generator is not implemented yet");
            }
            _ => proc_macro_error::abort!(generator.1, "Unknown generator {}", generator.0),
        }
    } else {
        proc_macro_error::abort_call_site!(
        "Missing generator specification!"; help = r#"Try adding #[java_rs(generator = "attribute"]"#);
    }
}
