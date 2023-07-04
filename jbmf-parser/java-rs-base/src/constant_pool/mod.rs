use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub use constant::Constant;

use crate::error::Error;
use crate::io::{ClassFilePart, ReadContext, WriteContext};
use crate::version::JavaVersion;

mod constant;

// https://docs.oracle.com/javase/specs/jvms/se15/html/jvms-4.html#jvms-4.4
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConstantPool(pub Vec<Constant>);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ConstantPoolIndex(pub u16);

impl ConstantPool {
    pub fn read<R: Read>(reader: &mut R, version: &JavaVersion) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let count = reader.read_u16::<BigEndian>()? - 1;
        let mut constants = Vec::with_capacity(count as usize);
        let mut i = 0;

        while i < count {
            let result = Constant::read(reader, version)?;
            let length = &(result.len() as u16);

            for constant in result {
                constants.push(constant);
            }

            i += length;
        }

        Ok(ConstantPool(constants))
    }

    pub fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        (1 + self.0.len() as u16).write(writer, ctx)?;

        for constant in &self.0 {
            constant.write(writer)?;
        }

        Ok(())
    }
}

impl ConstantPool {
    pub fn get(&self, index: ConstantPoolIndex) -> Option<&Constant> {
        if let Some(value) = self.0.get(index.0 as usize - 1) {
            Some(value)
        } else {
            None
        }
    }
}

impl From<Vec<Constant>> for ConstantPool {
    fn from(vec: Vec<Constant>) -> Self {
        Self(vec)
    }
}

impl ClassFilePart for ConstantPoolIndex {
    fn read<R: Read>(reader: &mut R, _: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        Ok(ConstantPoolIndex(reader.read_u16::<BigEndian>()?))
    }

    fn write<W: Write>(&self, writer: &mut W, _: &WriteContext) -> Result<(), Error> {
        writer.write_u16::<BigEndian>(self.0)?;
        Ok(())
    }
}
