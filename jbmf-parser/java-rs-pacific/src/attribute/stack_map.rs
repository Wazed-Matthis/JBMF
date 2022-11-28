use std::io::{Read, Write};

use java_rs_base::constant_pool::ConstantPoolIndex;
use java_rs_base::error::Error;
use java_rs_base::io::{ClassFilePart, ReadContext, SizedVec, WriteContext};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object { index: ConstantPoolIndex },
    Uninitialized { offset: u16 },
}

impl ClassFilePart for VerificationTypeInfo {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let ty = u8::read(reader, ctx)?;

        match ty {
            0 => Ok(Self::Top),
            1 => Ok(Self::Integer),
            2 => Ok(Self::Float),
            3 => Ok(Self::Double),
            4 => Ok(Self::Long),
            5 => Ok(Self::Null),
            6 => Ok(Self::UninitializedThis),
            7 => {
                let index = ConstantPoolIndex::read(reader, ctx)?;
                Ok(Self::Object { index })
            }
            8 => {
                let offset = u16::read(reader, ctx)?;
                Ok(Self::Uninitialized { offset })
            }
            v => Err(Error::UnknownVerificationType(v)),
        }
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        match self {
            Self::Top => 0u8.write(writer, ctx)?,
            Self::Integer => 1u8.write(writer, ctx)?,
            Self::Float => 2u8.write(writer, ctx)?,
            Self::Double => 3u8.write(writer, ctx)?,
            Self::Long => 4u8.write(writer, ctx)?,
            Self::Null => 5u8.write(writer, ctx)?,
            Self::UninitializedThis => 6u8.write(writer, ctx)?,
            Self::Object { index } => {
                7u8.write(writer, ctx)?;
                index.write(writer, ctx)?;
            }
            Self::Uninitialized { offset } => {
                8u8.write(writer, ctx)?;
                offset.write(writer, ctx)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StackMapFrame {
    Same {
        frame_type: u8,
    },
    SameLocals1StackItem {
        frame_type: u8,
        stack: VerificationTypeInfo,
    },
    SameLocals1StackItemExtended {
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    Chop {
        frame_type: u8,
        offset_delta: u16,
    },
    SameExtended {
        offset_delta: u16,
    },
    Append {
        frame_type: u8,
        offset_delta: u16,
        locals: SizedVec<u8, VerificationTypeInfo>,
    },
    Full {
        offset_delta: u16,
        locals: SizedVec<u16, VerificationTypeInfo>,
        stack: SizedVec<u16, VerificationTypeInfo>,
    },
}

impl ClassFilePart for StackMapFrame {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let frame_type = u8::read(reader, ctx)?;

        match frame_type {
            0..=63 => Ok(StackMapFrame::Same { frame_type }),
            64..=127 => {
                let stack = VerificationTypeInfo::read(reader, ctx)?;
                Ok(StackMapFrame::SameLocals1StackItem { frame_type, stack })
            }
            247 => {
                let offset_delta = u16::read(reader, ctx)?;
                let stack = VerificationTypeInfo::read(reader, ctx)?;

                Ok(StackMapFrame::SameLocals1StackItemExtended { offset_delta, stack })
            }
            248..=250 => {
                let offset_delta = u16::read(reader, ctx)?;
                Ok(StackMapFrame::Chop {
                    frame_type,
                    offset_delta,
                })
            }
            251 => {
                let offset_delta = u16::read(reader, ctx)?;
                Ok(StackMapFrame::SameExtended { offset_delta })
            }
            252..=254 => {
                let offset_delta = u16::read(reader, ctx)?;

                let size = frame_type - 251;
                let locals = SizedVec::<u8, VerificationTypeInfo>::read_without_size(size, reader, ctx)?;

                Ok(StackMapFrame::Append {
                    frame_type,
                    offset_delta,
                    locals,
                })
            }
            255 => {
                let offset_delta = u16::read(reader, ctx)?;
                let locals = SizedVec::<u16, VerificationTypeInfo>::read(reader, ctx)?;
                let stack = SizedVec::<u16, VerificationTypeInfo>::read(reader, ctx)?;

                Ok(StackMapFrame::Full {
                    offset_delta,
                    locals,
                    stack,
                })
            }
            v => Err(Error::UnknownStackMapFrameType(v)),
        }
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        match self {
            Self::Same { frame_type } => frame_type.write(writer, ctx)?,
            Self::SameLocals1StackItem { frame_type, stack } => {
                frame_type.write(writer, ctx)?;
                stack.write(writer, ctx)?;
            }
            Self::SameLocals1StackItemExtended { offset_delta, stack } => {
                247u8.write(writer, ctx)?;
                offset_delta.write(writer, ctx)?;
                stack.write(writer, ctx)?;
            }
            Self::Chop {
                frame_type,
                offset_delta,
            } => {
                frame_type.write(writer, ctx)?;
                offset_delta.write(writer, ctx)?;
            }
            Self::SameExtended { offset_delta } => {
                251u8.write(writer, ctx)?;
                offset_delta.write(writer, ctx)?;
            }
            Self::Append {
                frame_type,
                offset_delta,
                locals,
            } => {
                frame_type.write(writer, ctx)?;
                offset_delta.write(writer, ctx)?;
                locals.write_without_size(writer, ctx)?;
            }
            Self::Full {
                offset_delta,
                locals,
                stack,
            } => {
                255u8.write(writer, ctx)?;
                offset_delta.write(writer, ctx)?;
                locals.write(writer, ctx)?;
                stack.write(writer, ctx)?;
            }
        }

        Ok(())
    }
}

impl StackMapFrame {
    pub fn offset_delta(&self) -> u16 {
        match self {
            Self::Same { frame_type } => *frame_type as u16,
            Self::SameLocals1StackItem { frame_type, .. } => (*frame_type as u16) - 64,
            Self::SameLocals1StackItemExtended { offset_delta, .. } => *offset_delta,
            Self::Chop { offset_delta, .. } => *offset_delta,
            Self::SameExtended { offset_delta } => *offset_delta,
            Self::Append { offset_delta, .. } => *offset_delta,
            Self::Full { offset_delta, .. } => *offset_delta,
        }
    }
}
