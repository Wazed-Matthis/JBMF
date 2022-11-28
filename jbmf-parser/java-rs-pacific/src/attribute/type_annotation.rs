use std::io::{Read, Write};

use java_rs_base::error::Error;
use java_rs_base::io::{ClassFilePart, ReadContext, SizedVec, WriteContext};
use java_rs_derive::ClassFilePart;

use crate::attribute::annotation::ElementValuePair;
use crate::ConstantPoolIndex;

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct TypeAnnotation {
    target: Target,
    target_path: TypePath,
    ty: ConstantPoolIndex,
    element_value_pairs: SizedVec<u16, ElementValuePair>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Target {
    TypeParameter {
        target_type: u8,
    },
    Supertype {
        supertype: u16,
    },
    TypeParameterBound {
        target_type: u8,
        bound: u8,
    },
    Empty {
        target_type: u8,
    },
    FormalParameter {
        formal_parameter: u8,
    },
    Throws {
        throws_type: u16,
    },
    Localvar {
        target_type: u8,
        table: SizedVec<u16, LocalvarTable>,
    },
    Catch {
        exception_table: u16,
    },
    Offset {
        target_type: u8,
        offset: u16,
    },
    TypeArgument {
        target_type: u8,
        type_argument: u8,
    },
    Raw {
        target_type: u8,
        data: Vec<u8>,
    },
}

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct LocalvarTable {
    start_pc: u16,
    length: u16,
    index: u16,
}

impl ClassFilePart for Target {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let target_type = u8::read(reader, ctx)?;

        match target_type {
            0x00 | 0x01 => Ok(Self::TypeParameter { target_type }),
            0x10 => Ok(Self::Supertype {
                supertype: ClassFilePart::read(reader, ctx)?,
            }),
            0x11 | 0x12 => Ok(Self::TypeParameterBound {
                target_type,
                bound: ClassFilePart::read(reader, ctx)?,
            }),
            0x13 | 0x14 | 0x15 => Ok(Self::Empty { target_type }),
            0x16 => Ok(Self::FormalParameter {
                formal_parameter: ClassFilePart::read(reader, ctx)?,
            }),
            0x17 => Ok(Self::Throws {
                throws_type: ClassFilePart::read(reader, ctx)?,
            }),
            0x40 | 0x41 => Ok(Self::Localvar {
                target_type,
                table: ClassFilePart::read(reader, ctx)?,
            }),
            0x42 => Ok(Self::Catch {
                exception_table: ClassFilePart::read(reader, ctx)?,
            }),
            0x43 | 0x44 | 0x45 | 0x46 => Ok(Self::Offset {
                target_type,
                offset: ClassFilePart::read(reader, ctx)?,
            }),
            0x47 | 0x48 | 0x49 | 0x4A | 0x4B => Ok(Self::TypeArgument {
                target_type,
                type_argument: ClassFilePart::read(reader, ctx)?,
            }),
            _ => Err(Error::UnknownTargetType(target_type)),
        }
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        match self {
            Self::TypeParameter { target_type } => target_type.write(writer, ctx)?,
            Self::Supertype { supertype } => {
                0x10.write(writer, ctx)?;
                supertype.write(writer, ctx)?;
            }
            Self::TypeParameterBound { target_type, bound } => {
                target_type.write(writer, ctx)?;
                bound.write(writer, ctx)?;
            }
            Self::Empty { target_type } => target_type.write(writer, ctx)?,
            Self::FormalParameter { formal_parameter } => {
                0x16.write(writer, ctx)?;
                formal_parameter.write(writer, ctx)?;
            }
            Self::Throws { throws_type } => {
                0x17.write(writer, ctx)?;
                throws_type.write(writer, ctx)?;
            }
            Self::Localvar { target_type, table } => {
                target_type.write(writer, ctx)?;
                table.write(writer, ctx)?;
            }
            Self::Catch { exception_table } => {
                0x42.write(writer, ctx)?;
                exception_table.write(writer, ctx)?;
            }
            Self::Offset { target_type, offset } => {
                target_type.write(writer, ctx)?;
                offset.write(writer, ctx)?;
            }
            Self::TypeArgument {
                target_type,
                type_argument,
            } => {
                target_type.write(writer, ctx)?;
                type_argument.write(writer, ctx)?;
            }
            Self::Raw { target_type, data } => {
                target_type.write(writer, ctx)?;
                writer.write_all(data)?;
                data.len();
            }
        }

        Ok(())
    }
}

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct TypePath(SizedVec<u8, Path>);

#[derive(Debug, ClassFilePart, Copy, Clone, Eq, PartialEq)]
pub struct Path {
    type_path_kind: u8,
    type_argument_index: u8,
}
