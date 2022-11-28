use std::fmt::Debug;
use std::io::{Read, Write};

use bitflags::_core::fmt::Formatter;
use byteorder::{BigEndian, ReadBytesExt};

use attribute::Attribute;
pub use field::Field;
pub use flags::*;
pub use java_rs_base::constant_pool::*;
pub use java_rs_base::error::Error;
pub use java_rs_base::io::SizedVec;
use java_rs_base::io::{AttributeLocation, ClassFilePart, ReadContext, WriteContext};
pub use java_rs_base::java_utf8::{FromJavaUtf8Ext, ToJavaUtf8Ext};
pub use java_rs_base::version::JavaVersion;
pub use method::Method;

#[allow(dead_code, unused_variables)]
pub mod attribute;
mod field;
mod flags;
#[cfg(test)]
mod helper;
mod method;

#[derive(Eq, PartialEq)]
pub enum MagicNumber {
    Cafebabe,
    Unknown(u32),
}

impl Debug for MagicNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cafebabe => f.write_str("Cafebabe"),
            Self::Unknown(number) => f.write_str(&std::format!("Unknown(0x{:X?})", number)),
        }
    }
}

// https://docs.oracle.com/javase/specs/jvms/se15/html/jvms-4.html#jvms-4.1
#[derive(Debug, Eq, PartialEq)]
pub struct JavaClass {
    pub magic: MagicNumber,
    pub version: JavaVersion,
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    pub this_class: ConstantPoolIndex,
    pub super_class: ConstantPoolIndex,
    pub interfaces: SizedVec<u16, ConstantPoolIndex>,
    pub fields: SizedVec<u16, Field>,
    pub methods: SizedVec<u16, Method>,
    pub attributes: SizedVec<u16, Attribute>,
}

impl JavaClass {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let magic = match reader.read_u32::<BigEndian>()? {
            v if v == 0xCAFEBABE => MagicNumber::Cafebabe,
            v => MagicNumber::Unknown(v),
        };

        let version = {
            let minor = reader.read_u16::<BigEndian>()?;
            let major = reader.read_u16::<BigEndian>()?;
            JavaVersion { minor, major }
        };

        let constant_pool = ConstantPool::read(reader, &version)?;

        let ctx = ReadContext {
            version: &version,
            constant_pool: &constant_pool,
            location: None,
            name: None,
            position: None,
            length: None,
            wide: None,
        };

        let access_flags = AccessFlags::read(reader, &ctx)?;
        let this_class = ConstantPoolIndex::read(reader, &ctx)?;
        let super_class = ConstantPoolIndex::read(reader, &ctx)?;
        let interfaces = SizedVec::<u16, ConstantPoolIndex>::read(reader, &ctx)?;
        let fields = SizedVec::<u16, Field>::read(
            reader,
            &ReadContext {
                location: Some(&AttributeLocation::Field),
                ..ctx
            },
        )?;
        let methods = SizedVec::<u16, Method>::read(
            reader,
            &ReadContext {
                location: Some(&AttributeLocation::Method),
                ..ctx
            },
        )?;
        let attributes = SizedVec::<u16, Attribute>::read(
            reader,
            &ReadContext {
                location: Some(&AttributeLocation::ClassFile),
                ..ctx
            },
        )?;

        Ok(JavaClass {
            magic,
            version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        let ctx = WriteContext { position: None };
        let ctx = &ctx;

        match self.magic {
            MagicNumber::Cafebabe => (0xCAFEBABE as u32).write(writer, ctx)?,
            MagicNumber::Unknown(number) => number.write(writer, ctx)?,
        };

        self.version.minor.write(writer, ctx)?;
        self.version.major.write(writer, ctx)?;
        self.constant_pool.write(writer, ctx)?;
        self.access_flags.write(writer, ctx)?;
        self.this_class.write(writer, ctx)?;
        self.super_class.write(writer, ctx)?;
        self.interfaces.write(writer, ctx)?;
        self.fields.write(writer, ctx)?;
        self.methods.write(writer, ctx)?;
        self.attributes.write(writer, ctx)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{File, OpenOptions};
    use std::io::{BufReader, BufWriter};

    use crate::{
        AccessFlags, Constant, ConstantPoolIndex, Error, JavaClass, JavaVersion, MagicNumber,
        SizedVec,
    };

    use super::helper::*;

    #[test]
    fn check_sanity() -> Result<(), Error> {
        let (path, _guard) = init_tmp_dir("CheckSanityTest.class");

        let reference = JavaClass {
            magic: MagicNumber::Cafebabe,
            version: JavaVersion {
                major: 45,
                minor: 3,
            },
            constant_pool: vec![
                Constant::Class(ConstantPoolIndex(3)),
                Constant::Class(ConstantPoolIndex(4)),
                Constant::Utf8("CheckSanityTest".into()),
                Constant::Utf8("java/lang/Object".into()),
            ]
            .into(),
            access_flags: AccessFlags::NONE,
            this_class: ConstantPoolIndex(1),
            super_class: ConstantPoolIndex(2),
            interfaces: SizedVec::new(),
            fields: SizedVec::new(),
            methods: SizedVec::new(),
            attributes: SizedVec::new(),
        };

        reference.write(&mut BufWriter::new(File::create(&path)?))?;
        assert_eq!(
            reference,
            JavaClass::read(&mut BufReader::new(File::open(path)?))?
        );
        Ok(())
    }

    #[test]
    pub fn run_cfr_tests() {
        let c = JavaClass::read(&mut BufReader::new(
            OpenOptions::new()
                .read(true)
                .open("G:/cfr-tests/cfr_tests-master/output/java_6/org/benf/cfr/tests/AnnotationTest1.class")
                .unwrap(),
        ))
        .unwrap();

        for ele in c.methods.iter() {
            for ele in ele.attributes.iter() {
                println!("Attr: {:#?}", ele);
            }
        }
    }
}
