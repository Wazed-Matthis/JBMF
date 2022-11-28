use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataEnum, DeriveInput, Fields};

use java_rs_base::io::AttributeLocation;

use crate::attribute::JavaRsAttribute;
use crate::enum_handler::generators::EnumGenerator;
use crate::struct_handler;
use crate::struct_handler::{ReturnType, StructGenerationOptions};
use crate::utils::conversion::ConvertTo;
use crate::utils::generation;

pub struct AttributeGenerator;

impl EnumGenerator for AttributeGenerator {
    fn generate(input: &DeriveInput, data: &DataEnum) -> TokenStream {
        let (read_cases, write_cases) = generate_cases(data);
        let (read_body, write_body) = generate_bodies(read_cases, write_cases);
        generation::generate_class_file_part_impl(&input.ident, read_body, write_body)
    }
}

fn generate_cases(data: &DataEnum) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut read_cases = Vec::new();
    let mut write_cases = Vec::new();

    let options = {
        let mut options = StructGenerationOptions::new();
        options
            .exclude(vec!["name"])
            .write_return_type(ReturnType::Nothing)
            .read_return_type(ReturnType::Nothing)
            .read_unwrap(false)
            .use_self(false);
        options
    };

    for variant in &data.variants {
        let ident = &variant.ident;

        let options = StructGenerationOptions {
            return_type: quote! { Self::#ident },
            ..options.clone()
        };

        let attributes: Vec<JavaRsAttribute> = JavaRsAttribute::from_attributes(&variant.attrs);

        let (read_body, write_body, custom_io, locations, version) = {
            let mut read_body = None;
            let mut write_body = None;
            let mut custom_io = false;
            let mut locations: Option<Vec<TokenStream>> = None;
            let mut version = None;

            for attribute in attributes {
                match attribute {
                    JavaRsAttribute::IOImplementation { value, .. } => {
                        read_body = Some(quote! {
                            let result = #value::read(reader, ctx);
                        });
                        write_body = Some(quote! {
                            #value::write(self, writer, ctx)?
                        });
                        custom_io = true;
                    }
                    JavaRsAttribute::Location { value, .. } => {
                        locations = Some(
                            value
                                .iter()
                                .map(|location| match location {
                                    AttributeLocation::ClassFile => {
                                        quote! { java_rs_base::io::AttributeLocation::ClassFile }
                                    }
                                    AttributeLocation::Field => quote! { java_rs_base::io::AttributeLocation::Field },
                                    AttributeLocation::Method => quote! { java_rs_base::io::AttributeLocation::Method },
                                    AttributeLocation::Code => quote! { java_rs_base::io::AttributeLocation::Code },
                                })
                                .collect(),
                        )
                    }
                    JavaRsAttribute::Version { value, .. } => version = Some(value),
                    _ => {}
                }
            }

            if read_body.is_none() && write_body.is_none() {
                let (_read_body, _write_body) = match &variant.fields {
                    Fields::Named(fields) => struct_handler::named::generate_all(fields, &options),
                    Fields::Unnamed(fields) => struct_handler::unnamed::generate_all(fields, &options),
                    Fields::Unit => unimplemented!("unit structs are unsupported"),
                };
                read_body = Some(_read_body);
                write_body = Some(_write_body);
            }

            let mut read_body = read_body.unwrap();

            let parameters: Vec<Ident> = match &variant.fields {
                Fields::Named(fields) => fields.custom_into(),
                Fields::Unnamed(fields) => fields.custom_into(),
                Fields::Unit => unimplemented!("unit structs are unsupported"),
            };

            if custom_io {
                read_body = quote! {
                    #read_body
                    if result.is_err() {
                        return Ok(Self::Raw(RawAttribute {
                            name,
                            info: data
                        }))
                    }
                    let result = result.unwrap();
                };
            } else if parameters.len() > 1 {
                let filtered: Vec<&Ident> = parameters
                    .iter()
                    .filter(|ident| ident.to_string().as_str() != "name")
                    .collect();
                let conditions: Vec<TokenStream> = filtered.iter().map(|ident| quote! { #ident.is_err() }).collect();
                let unwrap: Vec<TokenStream> = filtered
                    .iter()
                    .map(|ident| quote! { let #ident = #ident.unwrap(); })
                    .collect();

                read_body = quote! {
                    #read_body
                    if #(#conditions)||* {
                        return Ok(Self::Raw(RawAttribute {
                            name,
                            info: data
                        }))
                    }
                    #(#unwrap)*
                };
            }

            let define_locations = if let Some(value) = locations {
                Some(quote! { vec![#(#value),*] })
            } else {
                None
            };

            (read_body, write_body.unwrap(), custom_io, define_locations, version)
        };

        let parameters = match &variant.fields {
            Fields::Named(fields) => {
                if custom_io {
                    quote! { { name, .. } }
                } else {
                    let parameters: Vec<Ident> = fields.custom_into();
                    quote! { { #(#parameters),* } }
                }
            }
            Fields::Unnamed(fields) => {
                if custom_io {
                    quote! { { name, .. } }
                } else {
                    let parameters: Vec<Ident> = fields.custom_into();
                    quote! { ( #(#parameters),* ) }
                }
            }
            Fields::Unit => unimplemented!("unit structs are unsupported"),
        };

        let string = ident.to_string();

        if &string == "InvalidUtf8"
            || &string == "IllegalNameReference"
            || &string == "UnsupportedAndInvalidLocation"
            || &string == "InvalidLocation"
            || &string == "Unsupported"
            || &string == "Unknown"
            || &string == "Raw"
            || &string == "Custom"
        {
            write_cases.push(generate_edge_case_write_case(ident, &parameters, write_body));
            continue;
        } else {
            write_cases.push(generate_ordinary_write_case(ident, &parameters, write_body));
        }

        let name = variant.ident.to_string();

        let additional = if let Some(version) = version {
            let major = version.major;
            let minor = version.minor;

            match &variant.fields {
                Fields::Named(_fields) => {
                    if custom_io {
                        quote! {
                            let supported = ctx.version.supports(#major, #minor);
                            let locations = #locations;

                            if supported {
                                if locations.contains(location) {
                                    Ok(result)
                                } else {
                                    Ok(Self::InvalidLocation(Box::new(result)))
                                }
                            } else {
                                if locations.contains(location) {
                                    Ok(Self::Unsupported(Box::new(result)))
                                } else {
                                    Ok(Self::UnsupportedAndInvalidLocation(Box::new(result)))
                                }
                            }
                        }
                    } else if locations.is_some() {
                        let locations = locations.unwrap();

                        quote! {
                            let supported = ctx.version.supports(#major, #minor);
                            let locations = #locations;

                            if supported {
                                if locations.contains(location) {
                                    Ok(Self::#ident #parameters)
                                } else {
                                    Ok(Self::InvalidLocation(Box::new(Self::#ident #parameters)))
                                }
                            } else {
                                if locations.contains(location) {
                                    Ok(Self::Unsupported(Box::new(Self::#ident #parameters)))
                                } else {
                                    Ok(Self::UnsupportedAndInvalidLocation(Box::new(Self::#ident #parameters)))
                                }
                            }
                        }
                    } else {
                        quote! {
                            if ctx.version.supports(#major, #minor) {
                                Ok(Self::#ident #parameters)
                            } else {
                                Ok(Self::Unsupported(Box::new(Self::#ident #parameters)))
                            }
                        }
                    }
                }
                Fields::Unnamed(_fields) => unimplemented!("unnamed structs are unsupported"),
                Fields::Unit => unimplemented!("unit structs are unsupported"),
            }
        } else if custom_io {
            if locations.is_some() {
                quote! {
                    let locations = #locations;

                    if locations.contains(location) {
                        Ok(result)
                    } else {
                        Ok(Self::InvalidLocation(Box::new(result)))
                    }
                }
            } else {
                quote! {
                    Ok(result)
                }
            }
        } else {
            quote! {
                Ok(Self::#ident #parameters)
            }
        };

        read_cases.push(quote! {
            #name => {
                let name = name_index;
                #read_body
                #additional
            }
        });
    }

    read_cases.push(quote! {
        _ => Ok(Self::Unknown(RawAttribute {
            name: name_index,
            info: data,
        }))
    });

    (read_cases, write_cases)
}

fn generate_edge_case_write_case(ident: &Ident, parameters: &TokenStream, body: TokenStream) -> TokenStream {
    quote! {
        Self::#ident #parameters => {
            #body
            Ok(())
        }
    }
}

fn generate_ordinary_write_case(ident: &Ident, parameters: &TokenStream, body: TokenStream) -> TokenStream {
    quote! {
        Self::#ident #parameters => {
            name.write(writer, ctx)?;

            let buffer = {
                let mut buffer: java_rs_base::io::SizedVec<u32, u8> = java_rs_base::io::SizedVec::new();

                {
                    let mut writer = std::io::BufWriter::new(&mut buffer);
                    let writer = &mut writer;
                    #body
                }

                buffer
            };

            buffer.write(writer, ctx)?;
            Ok(())
        }
    }
}

fn generate_bodies(read_cases: Vec<TokenStream>, write_cases: Vec<TokenStream>) -> (TokenStream, TokenStream) {
    let read_body = generate_read_body(read_cases);
    let write_body = generate_write_body(write_cases);
    (read_body, write_body)
}

fn generate_read_body(cases: Vec<TokenStream>) -> TokenStream {
    quote! {
        let name_index: java_rs_base::constant_pool::ConstantPoolIndex = java_rs_base::io::ClassFilePart::read(reader, ctx)?;
        let data: java_rs_base::io::SizedVec<u32, u8> = java_rs_base::io::ClassFilePart::read(reader, ctx)?;

        let predetermined_name = if let Some(constant) = ctx.constant_pool.get(name_index) {
            if let java_rs_base::constant_pool::Constant::Unsupported(value) = constant {
                let value = value.as_ref();

                if let java_rs_base::constant_pool::Constant::Utf8(ref v) = value {
                    Some((*v).clone())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let name = if let Some(constant) = ctx.constant_pool.get(name_index) {
            match constant {
                java_rs_base::constant_pool::Constant::Utf8(value) => (*value).clone(),
                java_rs_base::constant_pool::Constant::Unsupported(_) if predetermined_name.is_some() => predetermined_name.unwrap(),
                java_rs_base::constant_pool::Constant::InvalidUtf8(_) => {
                    return Ok(Self::InvalidUtf8(RawAttribute {
                        name: name_index,
                        info: data,
                    }));
                }
                _ => {
                    return Ok(Self::IllegalNameReference(RawAttribute {
                        name: name_index,
                        info: data,
                    }));
                }
            }
        } else {
            return Ok(Self::IllegalNameReference(RawAttribute {
                name: name_index,
                info: data,
            }));
        };
        let name = name.as_str();

        let ctx = java_rs_base::io::ReadContext {
            name: Some(name_index),
            length: Some(data.len() as u32),
            ..*ctx
        };
        let ctx = &ctx;

        let mut reader = std::io::Cursor::new(std::ops::Deref::deref(&data));
        let reader = &mut reader;

        let location = ctx.location.unwrap();

        match name {
            #(#cases)*
        }
    }
}

fn generate_write_body(cases: Vec<TokenStream>) -> TokenStream {
    quote! {
        match self {
            #(#cases)*
        }
    }
}
