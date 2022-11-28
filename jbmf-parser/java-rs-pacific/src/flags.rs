use std::io::{Read, Write};

use java_rs_base::error::Error;
use java_rs_base::io::{ClassFilePart, ReadContext, WriteContext};

macro_rules! implement_class_file_part {
    ($ty:ident) => {
        impl ClassFilePart for $ty {
            fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
            where
                Self: Sized,
            {
                let value = u16::read(reader, ctx)?;

                Ok(unsafe { Self::from_bits_unchecked(value) })
            }

            fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
                self.bits.write(writer, ctx)
            }
        }
    };
}

bitflags::bitflags! {
    pub struct AccessFlags: u16 {
        const NONE = 0x0000;
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const SUPER = 0x0020;
        const INTERFACE = 0x0200;
        const ABSTRACT = 0x0400;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
        const MODULE = 0x8000;
        const MANDATED = Self::MODULE.bits;
    }
}

implement_class_file_part!(AccessFlags);

bitflags::bitflags! {
    pub struct ModuleFlags: u16 {
        const OPEN = 0x0020;
        const SYNTHETIC = 0x1000;
        const MANDATED = 0x8000;
    }
}

implement_class_file_part!(ModuleFlags);

bitflags::bitflags! {
    pub struct ModuleDependencyFlags: u16 {
        const TRANSITIVE = 0x0020;
        const STATIC_PHASE = 0x0040;
        const SYNTHETIC = 0x1000;
        const MANDATED = 0x8000;
    }
}

implement_class_file_part!(ModuleDependencyFlags);
