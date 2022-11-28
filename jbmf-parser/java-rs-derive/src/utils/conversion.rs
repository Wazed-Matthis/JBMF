use proc_macro2::Ident;
use syn::{FieldsNamed, FieldsUnnamed, Lit};

pub trait ConvertTo<T> {
    fn custom_into(&self) -> T;
}

impl ConvertTo<String> for Lit {
    fn custom_into(&self) -> String {
        match self {
            Lit::Str(str) => str.value(),
            Lit::Float(float) => float.to_string(),
            _ => unimplemented!(),
        }
    }
}

impl ConvertTo<u16> for Lit {
    fn custom_into(&self) -> u16 {
        match self {
            Lit::Int(int) => int.base10_parse().expect("unable to parse lit float"),
            _ => unimplemented!(),
        }
    }
}

impl ConvertTo<f32> for Lit {
    fn custom_into(&self) -> f32 {
        match self {
            Lit::Float(float) => float.base10_parse().expect("unable to parse lit float"),
            _ => unimplemented!(),
        }
    }
}

impl ConvertTo<Vec<Ident>> for FieldsNamed {
    fn custom_into(&self) -> Vec<Ident> {
        let mut parameters = Vec::new();

        for field in &self.named {
            parameters.push(field.ident.as_ref().expect("named struct field has no name").clone());
        }

        parameters
    }
}

impl ConvertTo<Vec<Ident>> for FieldsUnnamed {
    fn custom_into(&self) -> Vec<Ident> {
        let mut parameters = Vec::new();

        for i in 1..self.unnamed.len() + 1 {
            parameters.push(quote::format_ident!("f{}", i));
        }

        parameters
    }
}
