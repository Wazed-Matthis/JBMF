use std::fmt::Debug;

use proc_macro2::Span;
use quote::ToTokens;
use std::fmt::Formatter;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Attribute, Path};

use java_rs_base::version::JavaVersion;

use crate::symbol;
use java_rs_base::io::AttributeLocation;

#[derive(Clone)]
pub enum JavaRsAttribute {
    Generator { span: Span, value: String },
    IOImplementation { span: Span, value: Path },
    Location { span: Span, value: Vec<AttributeLocation> },
    Version { span: Span, value: JavaVersion },
    OpCode { span: Span, value: u8 },
}

struct JavaRsAttributeVec(pub Vec<JavaRsAttribute>);

impl Parse for JavaRsAttributeVec {
    fn parse(stream: ParseStream) -> Result<JavaRsAttributeVec, syn::parse::Error> {
        let mut attrs = Vec::new();

        loop {
            let ty = stream.parse::<syn::Ident>()?;

            let attr = match ty.to_string().as_ref() {
                "generator" => JavaRsAttribute::parse_generator(stream)?,
                "io_implementation" => JavaRsAttribute::parse_io_implementation(stream)?,
                "location" => JavaRsAttribute::parse_location(stream)?,
                "version" => JavaRsAttribute::parse_version(stream)?,
                "opcode" => JavaRsAttribute::parse_opcode(stream)?,
                _ => {
                    return Err(syn::parse::Error::new(
                        ty.span(),
                        "Expected version, generator or io_implementation",
                    ));
                }
            };

            attrs.push(attr);

            if stream.parse::<syn::Token![,]>().is_err() {
                break;
            }
        }

        Ok(JavaRsAttributeVec(attrs))
    }
}

impl JavaRsAttribute {
    pub fn from_attributes(attrs: &[Attribute]) -> Vec<JavaRsAttribute> {
        attrs
            .iter()
            .filter(|attr| attr.path == symbol::JAVA_RS)
            .filter_map(|attr| match attr.parse_args::<JavaRsAttributeVec>() {
                Ok(v) => Some(v.0),
                Err(err) => {
                    proc_macro_error::emit_error!(err);
                    None
                }
            })
            .flatten()
            .collect()
    }

    fn parse_generator(stream: ParseStream) -> Result<JavaRsAttribute, syn::parse::Error> {
        stream.parse::<syn::Token![=]>()?;
        let name = stream.parse::<syn::LitStr>()?;
        Ok(JavaRsAttribute::Generator {
            span: name.span(),
            value: name.value(),
        })
    }

    fn parse_io_implementation(stream: ParseStream) -> Result<JavaRsAttribute, syn::parse::Error> {
        stream.parse::<syn::Token![=]>()?;
        let ty = stream.parse::<syn::Type>()?;

        let type_path = if let syn::Type::Path(path) = ty {
            path
        } else {
            return Err(syn::parse::Error::new(ty.span(), "Expected a type path"));
        };

        Ok(JavaRsAttribute::IOImplementation {
            span: type_path.span(),
            value: type_path.path,
        })
    }

    fn parse_location(stream: ParseStream) -> Result<JavaRsAttribute, syn::parse::Error> {
        stream.parse::<syn::Token![=]>()?;
        let (span, locations) = if stream.peek(syn::token::Bracket) {
            let content;
            let bracket = syn::bracketed!(content in stream);

            let mut locations = Vec::new();

            loop {
                let ident = content.parse::<syn::Ident>()?;
                if locations.contains(&ident) {
                    proc_macro_error::emit_error!(ident.span(), "Duplicated location {}", ident);
                } else {
                    locations.push(ident);
                }

                if content.parse::<syn::Token![,]>().is_err() {
                    break;
                }
            }

            (bracket.span, locations)
        } else {
            let ident = stream.parse::<syn::Ident>()?;

            (ident.span(), vec![ident])
        };

        let locations = locations
            .iter()
            .filter_map(|ident| match ident.to_string().as_str() {
                "Code" => Some(AttributeLocation::Code),
                "Method" => Some(AttributeLocation::Method),
                "Field" => Some(AttributeLocation::Field),
                "ClassFile" => Some(AttributeLocation::ClassFile),
                v => {
                    proc_macro_error::emit_error!(ident.span(),
                "Unknown location type {}", v; help = "Only Code, Method, Field or Class are currently available");
                    None
                }
            })
            .collect();

        Ok(JavaRsAttribute::Location { span, value: locations })
    }

    fn parse_version(stream: ParseStream) -> Result<JavaRsAttribute, syn::parse::Error> {
        stream.parse::<syn::Token![=]>()?;
        let float_literal = stream.parse::<syn::LitFloat>()?;
        let digits = float_literal.base10_digits().split('.').collect::<Vec<&str>>();
        assert_eq!(digits.len(), 2);

        let major = digits[0].parse().unwrap();
        let minor = digits[1].parse().unwrap();

        Ok(JavaRsAttribute::Version {
            span: float_literal.span(),
            value: JavaVersion { major, minor },
        })
    }

    fn parse_opcode(stream: ParseStream) -> Result<JavaRsAttribute, syn::parse::Error> {
        stream.parse::<syn::Token![=]>()?;
        let literal = stream.parse::<syn::LitInt>()?;

        Ok(JavaRsAttribute::OpCode {
            span: literal.span(),
            value: literal.base10_parse()?,
        })
    }
}

impl Debug for JavaRsAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JavaRsAttribute::Version { span, value } => f
                .debug_struct("JavaRsAttribute::Version")
                .field("span", span)
                .field("value", value)
                .finish(),
            JavaRsAttribute::Generator { span, value } => f
                .debug_struct("JavaRsAttribute::Generator")
                .field("span", span)
                .field("value", value)
                .finish(),
            JavaRsAttribute::IOImplementation { span, value } => f
                .debug_struct("JavaRsAttribute::IOImplementation")
                .field("span", span)
                .field("value", &value.to_token_stream())
                .finish(),
            JavaRsAttribute::Location { span, value } => f
                .debug_struct("JavaRsAttribute::Location")
                .field("span", span)
                .field("value", value)
                .finish(),
            JavaRsAttribute::OpCode { span, value } => f
                .debug_struct("JavaRsAttribute::OpCode")
                .field("span", span)
                .field("value", value)
                .finish(),
        }
    }
}
