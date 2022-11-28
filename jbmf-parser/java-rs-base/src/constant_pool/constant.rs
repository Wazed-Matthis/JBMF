use std::fmt::Debug;
use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::constant_pool::{ConstantPoolIndex, JavaVersion};
use crate::error::Error;
use crate::java_utf8::{FromJavaUtf8Ext, ToJavaUtf8Ext};

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    // Sorted by tag
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(ConstantPoolIndex),
    String(ConstantPoolIndex),
    FieldRef {
        class: ConstantPoolIndex,
        name_and_type: ConstantPoolIndex,
    },
    MethodRef {
        class: ConstantPoolIndex,
        name_and_type: ConstantPoolIndex,
    },
    InterfaceMethodRef {
        class: ConstantPoolIndex,
        name_and_type: ConstantPoolIndex,
    },
    NameAndType {
        name: ConstantPoolIndex,
        descriptor: ConstantPoolIndex,
    },
    MethodHandle {
        reference_kind: u8,
        reference: ConstantPoolIndex,
    },
    MethodType(ConstantPoolIndex),
    // TODO: Create BootstrapMethodAttributeIndex
    Dynamic {
        bootstrap_method_attribute: u16,
        name_and_type: ConstantPoolIndex,
    },
    InvokeDynamic {
        bootstrap_method_attribute: u16,
        name_and_type: ConstantPoolIndex,
    },
    Module(ConstantPoolIndex),
    Package(ConstantPoolIndex),

    // Custom
    Raw {
        // write-only
        tag: u8,
        info: Vec<u8>,
    },
    // Example: Here is a Java 11 constant in a Java 7 class file
    Unsupported(Box<Constant>),
    InvalidUtf8(Vec<u8>),
    Unusable,
}

impl Eq for Constant {}

impl Constant {
    pub fn read<R: Read>(reader: &mut R, version: &JavaVersion) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        let tag = reader.read_u8()?;

        match tag {
            1 => {
                let length = reader.read_u16::<BigEndian>()?;

                let bytes = {
                    let mut bytes = vec![0u8; length as usize];
                    reader.read_exact(&mut bytes)?;
                    bytes
                };

                let constant = match String::from_java_utf8(&bytes) {
                    Err(_) => Constant::InvalidUtf8(bytes),
                    Ok(v) => Constant::Utf8(v),
                };

                if version.supports(45, 3) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            3 => {
                let constant = Constant::Integer(reader.read_i32::<BigEndian>()?);

                if version.supports(45, 3) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            4 => {
                let constant = Constant::Float(reader.read_f32::<BigEndian>()?);

                if version.supports(45, 3) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            5 => {
                let constant = Constant::Long(reader.read_i64::<BigEndian>()?);

                if version.supports(45, 3) {
                    return Ok(vec![constant, Constant::Unusable]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant)), Constant::Unusable])
            }
            6 => {
                let constant = Constant::Double(reader.read_f64::<BigEndian>()?);

                if version.supports(45, 3) {
                    return Ok(vec![constant, Constant::Unusable]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant)), Constant::Unusable])
            }
            7 => {
                let constant = Constant::Class(ConstantPoolIndex(reader.read_u16::<BigEndian>()?));

                if version.supports(45, 3) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            8 => {
                let constant = Constant::String(ConstantPoolIndex(reader.read_u16::<BigEndian>()?));

                if version.supports(45, 3) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            9 => {
                let constant = Constant::FieldRef {
                    class: ConstantPoolIndex(reader.read_u16::<BigEndian>()?),
                    name_and_type: ConstantPoolIndex(reader.read_u16::<BigEndian>()?),
                };

                if version.supports(45, 3) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            10 => {
                let constant = Constant::MethodRef {
                    class: ConstantPoolIndex(reader.read_u16::<BigEndian>()?),
                    name_and_type: ConstantPoolIndex(reader.read_u16::<BigEndian>()?),
                };

                if version.supports(45, 3) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            11 => {
                let constant = Constant::InterfaceMethodRef {
                    class: ConstantPoolIndex(reader.read_u16::<BigEndian>()?),
                    name_and_type: ConstantPoolIndex(reader.read_u16::<BigEndian>()?),
                };

                if version.supports(45, 3) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            12 => {
                let constant = Constant::NameAndType {
                    name: ConstantPoolIndex(reader.read_u16::<BigEndian>()?),
                    descriptor: ConstantPoolIndex(reader.read_u16::<BigEndian>()?),
                };

                if version.supports(45, 3) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            15 => {
                let constant = Constant::MethodHandle {
                    reference_kind: reader.read_u8()?,
                    reference: ConstantPoolIndex(reader.read_u16::<BigEndian>()?),
                };

                if version.supports(51, 0) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            16 => {
                let constant = Constant::MethodType(ConstantPoolIndex(reader.read_u16::<BigEndian>()?));

                if version.supports(51, 0) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            17 => {
                let constant = Constant::Dynamic {
                    bootstrap_method_attribute: reader.read_u16::<BigEndian>()?,
                    name_and_type: ConstantPoolIndex(reader.read_u16::<BigEndian>()?),
                };

                if version.supports(55, 0) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            18 => {
                let constant = Constant::InvokeDynamic {
                    bootstrap_method_attribute: reader.read_u16::<BigEndian>()?,
                    name_and_type: ConstantPoolIndex(reader.read_u16::<BigEndian>()?),
                };

                if version.supports(51, 0) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            19 => {
                let constant = Constant::Module(ConstantPoolIndex(reader.read_u16::<BigEndian>()?));

                if version.supports(53, 0) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            20 => {
                let constant = Constant::Package(ConstantPoolIndex(reader.read_u16::<BigEndian>()?));

                if version.supports(53, 0) {
                    return Ok(vec![constant]);
                }

                Ok(vec![Constant::Unsupported(Box::new(constant))])
            }
            _ => Err(Error::UnknownTag(tag)),
        }
    }
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        match self {
            Self::Utf8(v) => {
                writer.write_u8(1)?;
                let bytes = String::to_java_utf8(v)?;
                writer.write_u16::<BigEndian>(bytes.len() as u16)?;
                writer.write_all(&bytes)?;
            }
            Self::Integer(v) => {
                writer.write_u8(3)?;
                writer.write_i32::<BigEndian>(*v)?;
            }
            Self::Float(v) => {
                writer.write_u8(4)?;
                writer.write_f32::<BigEndian>(*v)?;
            }
            Self::Long(v) => {
                writer.write_u8(5)?;
                writer.write_i64::<BigEndian>(*v)?;
            }
            Self::Double(v) => {
                writer.write_u8(6)?;
                writer.write_f64::<BigEndian>(*v)?;
            }
            Self::Class(v) => {
                writer.write_u8(7)?;
                writer.write_u16::<BigEndian>(v.0)?;
            }
            Self::String(v) => {
                writer.write_u8(8)?;
                writer.write_u16::<BigEndian>(v.0)?;
            }
            Self::FieldRef { class, name_and_type } => {
                writer.write_u8(9)?;
                writer.write_u16::<BigEndian>(class.0)?;
                writer.write_u16::<BigEndian>(name_and_type.0)?;
            }
            Self::MethodRef { class, name_and_type } => {
                writer.write_u8(10)?;
                writer.write_u16::<BigEndian>(class.0)?;
                writer.write_u16::<BigEndian>(name_and_type.0)?;
            }
            Self::InterfaceMethodRef { class, name_and_type } => {
                writer.write_u8(11)?;
                writer.write_u16::<BigEndian>(class.0)?;
                writer.write_u16::<BigEndian>(name_and_type.0)?;
            }
            Self::NameAndType { name, descriptor } => {
                writer.write_u8(12)?;
                writer.write_u16::<BigEndian>(name.0)?;
                writer.write_u16::<BigEndian>(descriptor.0)?;
            }
            Self::MethodHandle {
                reference_kind,
                reference,
            } => {
                writer.write_u8(15)?;
                writer.write_u8(*reference_kind)?;
                writer.write_u16::<BigEndian>(reference.0)?;
            }
            Self::MethodType(v) => {
                writer.write_u8(16)?;
                writer.write_u16::<BigEndian>(v.0)?;
            }
            Self::Dynamic {
                bootstrap_method_attribute,
                name_and_type,
            } => {
                writer.write_u8(17)?;
                writer.write_u16::<BigEndian>(*bootstrap_method_attribute)?;
                writer.write_u16::<BigEndian>(name_and_type.0)?;
            }
            Self::InvokeDynamic {
                bootstrap_method_attribute,
                name_and_type,
            } => {
                writer.write_u8(18)?;
                writer.write_u16::<BigEndian>(*bootstrap_method_attribute)?;
                writer.write_u16::<BigEndian>(name_and_type.0)?;
            }
            Self::Module(v) => {
                writer.write_u8(19)?;
                writer.write_u16::<BigEndian>(v.0)?;
            }
            Self::Package(v) => {
                writer.write_u8(20)?;
                writer.write_u16::<BigEndian>(v.0)?;
            }
            Self::Raw { tag, info } => {
                writer.write_u8(*tag)?;
                writer.write_all(info)?;
            }
            Self::Unsupported(constant) => constant.write(writer)?,
            Self::InvalidUtf8(bytes) => {
                writer.write_u8(1)?;
                writer.write_u16::<BigEndian>(bytes.len() as u16)?;
                writer.write_all(&bytes)?;
            }
            Self::Unusable => {}
        }

        Ok(())
    }
}
