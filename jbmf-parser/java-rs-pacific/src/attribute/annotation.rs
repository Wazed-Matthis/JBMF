use std::io::{Read, Write};

use byteorder::ReadBytesExt;

use java_rs_base::constant_pool::ConstantPoolIndex;
use java_rs_base::error::Error;
use java_rs_base::io::{ClassFilePart, ReadContext, SizedVec, WriteContext};
use java_rs_derive::ClassFilePart;

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct Annotation {
    ty: ConstantPoolIndex,
    element_value_pairs: SizedVec<u16, ElementValuePair>,
}

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct ElementValuePair {
    element_name: ConstantPoolIndex,
    element_value: ElementValue,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ElementValue {
    ConstValue {
        tag: char,
        index: ConstantPoolIndex,
    },
    EnumConstValue {
        type_name: ConstantPoolIndex,
        const_name: ConstantPoolIndex,
    },
    Class(ConstantPoolIndex),
    AnnotationValue(Annotation),
    ArrayValue(SizedVec<u16, ElementValue>),
}

impl ClassFilePart for ElementValue {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        let tag = reader.read_u8()?;

        match tag {
            b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => Ok(Self::ConstValue {
                tag: tag as char,
                index: ClassFilePart::read(reader, ctx)?,
            }),
            b'e' => Ok(Self::EnumConstValue {
                type_name: ClassFilePart::read(reader, ctx)?,
                const_name: ClassFilePart::read(reader, ctx)?,
            }),
            b'c' => Ok(Self::Class(ClassFilePart::read(reader, ctx)?)),
            b'@' => Ok(Self::AnnotationValue(ClassFilePart::read(reader, ctx)?)),
            b'[' => Ok(Self::ArrayValue(ClassFilePart::read(reader, ctx)?)),
            _ => Err(Error::InvalidElementValueTag(tag as char)),
        }
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        match self {
            Self::ConstValue { tag, index } => {
                (*tag as u8).write(writer, ctx)?;
                index.write(writer, ctx)?;
            }
            Self::EnumConstValue { type_name, const_name } => {
                b'e'.write(writer, ctx)?;
                type_name.write(writer, ctx)?;
                const_name.write(writer, ctx)?;
            }
            Self::Class(index) => {
                b'c'.write(writer, ctx)?;
                index.write(writer, ctx)?;
            }
            Self::AnnotationValue(annotation) => {
                b'@'.write(writer, ctx)?;
                annotation.write(writer, ctx)?;
            }
            Self::ArrayValue(array) => {
                b'['.write(writer, ctx)?;
                array.write(writer, ctx)?;
            }
        }

        Ok(())
    }
}
