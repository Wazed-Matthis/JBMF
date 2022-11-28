#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, BufWriter};

    use crate::{AccessFlags, Constant, ConstantPoolIndex, Error, Field, JavaClass, JavaVersion, MagicNumber, Method, SizedVec};
    use crate::attribute::{Attribute, Compatibility};
    use crate::helper::*;

    #[test]
    fn check_synthetic_class_file() -> Result<(), Error> {
        let (path, _guard) = init_tmp_dir("SyntheticClassFileTest.class");

        let reference = JavaClass {
            magic: MagicNumber::Cafebabe,
            version: JavaVersion { major: 45, minor: 3 },
            constant_pool: vec![
                Constant::Class(ConstantPoolIndex(3)),
                Constant::Class(ConstantPoolIndex(4)),
                Constant::Utf8("SyntheticClassFileTest".into()),
                Constant::Utf8("java/lang/Object".into()),
                Constant::Utf8("Synthetic".into()),
            ]
                .into(),
            access_flags: AccessFlags::NONE,
            this_class: ConstantPoolIndex(1),
            super_class: ConstantPoolIndex(2),
            interfaces: SizedVec::new(),
            fields: SizedVec::new(),
            methods: SizedVec::new(),
            attributes: vec![Attribute::Synthetic {
                name: ConstantPoolIndex(5)
            }].into(),
        };

        reference.write(&mut BufWriter::new(File::create(&path)?))?;
        assert_eq!(reference, JavaClass::read(&mut BufReader::new(File::open(path)?))?);
        Ok(())
    }

    #[test]
    fn check_synthetic_field() -> Result<(), Error> {
        let (path, _guard) = init_tmp_dir("SyntheticFieldTest.class");

        let reference = JavaClass {
            magic: MagicNumber::Cafebabe,
            version: JavaVersion { major: 45, minor: 3 },
            constant_pool: vec![
                Constant::Class(ConstantPoolIndex(3)),
                Constant::Class(ConstantPoolIndex(4)),
                Constant::Utf8("SyntheticFieldTest".into()),
                Constant::Utf8("java/lang/Object".into()),
                Constant::Utf8("f1".into()),
                Constant::Utf8("Z".into()),
                Constant::Utf8("Synthetic".into()),
            ]
                .into(),
            access_flags: AccessFlags::NONE,
            this_class: ConstantPoolIndex(1),
            super_class: ConstantPoolIndex(2),
            interfaces: SizedVec::new(),
            fields: vec![Field {
                access_flags: AccessFlags::NONE,
                name: ConstantPoolIndex(5),
                descriptor: ConstantPoolIndex(6),
                attributes: vec![Attribute::Synthetic {
                    name: ConstantPoolIndex(7)
                }].into()
            }].into(),
            methods: SizedVec::new(),
            attributes: SizedVec::new(),
        };

        reference.write(&mut BufWriter::new(File::create(&path)?))?;
        assert_eq!(reference, JavaClass::read(&mut BufReader::new(File::open(path)?))?);
        Ok(())
    }

    #[test]
    fn check_synthetic_method() -> Result<(), Error> {
        let (path, _guard) = init_tmp_dir("SyntheticMethodTest.class");

        let reference = JavaClass {
            magic: MagicNumber::Cafebabe,
            version: JavaVersion { major: 45, minor: 3 },
            constant_pool: vec![
                Constant::Class(ConstantPoolIndex(3)),
                Constant::Class(ConstantPoolIndex(4)),
                Constant::Utf8("SyntheticMethodTest".into()),
                Constant::Utf8("java/lang/Object".into()),
                Constant::Utf8("m1".into()),
                Constant::Utf8("()V".into()),
                Constant::Utf8("Synthetic".into()),
            ]
                .into(),
            access_flags: AccessFlags::NONE,
            this_class: ConstantPoolIndex(1),
            super_class: ConstantPoolIndex(2),
            interfaces: SizedVec::new(),
            fields: SizedVec::new(),
            methods: vec![Method {
                access_flags: AccessFlags::NONE,
                name: ConstantPoolIndex(5),
                descriptor: ConstantPoolIndex(6),
                attributes: vec![Attribute::Synthetic {
                    name: ConstantPoolIndex(7)
                }].into()
            }].into(),
            attributes: SizedVec::new(),
        };

        reference.write(&mut BufWriter::new(File::create(&path)?))?;
        assert_eq!(reference, JavaClass::read(&mut BufReader::new(File::open(path)?))?);
        Ok(())
    }

    #[test]
    fn check_synthetic_code() -> Result<(), Error> {
        let (path, _guard) = init_tmp_dir("SyntheticCodeTest.class");

        let mut reference = JavaClass {
            magic: MagicNumber::Cafebabe,
            version: JavaVersion { major: 45, minor: 3 },
            constant_pool: vec![
                Constant::Class(ConstantPoolIndex(3)),
                Constant::Class(ConstantPoolIndex(4)),
                Constant::Utf8("SyntheticCodeTest".into()),
                Constant::Utf8("java/lang/Object".into()),
                Constant::Utf8("m1".into()),
                Constant::Utf8("()V".into()),
                Constant::Utf8("Code".into()),
                Constant::Utf8("Synthetic".into()),
            ]
                .into(),
            access_flags: AccessFlags::NONE,
            this_class: ConstantPoolIndex(1),
            super_class: ConstantPoolIndex(2),
            interfaces: SizedVec::new(),
            fields: SizedVec::new(),
            methods: vec![Method {
                access_flags: AccessFlags::NONE,
                name: ConstantPoolIndex(5),
                descriptor: ConstantPoolIndex(6),
                attributes: vec![Attribute::Code {
                    name: ConstantPoolIndex(7),
                    max_stack: Compatibility::Current(0),
                    max_locals: Compatibility::Current(0),
                    code: Compatibility::Current(SizedVec::new()),
                    exception_table: SizedVec::new(),
                    attributes: vec![Attribute::Synthetic {
                        name: ConstantPoolIndex(8)
                    }].into()
                }].into()
            }].into(),
            attributes: SizedVec::new(),
        };

        reference.write(&mut BufWriter::new(File::create(&path)?))?;

        reference.methods[0].attributes[0] = Attribute::Code {
            name: ConstantPoolIndex(7),
            max_stack: Compatibility::Current(0),
            max_locals: Compatibility::Current(0),
            code: Compatibility::Current(SizedVec::new()),
            exception_table: SizedVec::new(),
            attributes: vec![Attribute::InvalidLocation(Box::new(Attribute::Synthetic {
                name: ConstantPoolIndex(8)
            }))].into()
        };

        assert_ne!(reference, JavaClass::read(&mut BufReader::new(File::open(path)?))?);
        Ok(())
    }
}
