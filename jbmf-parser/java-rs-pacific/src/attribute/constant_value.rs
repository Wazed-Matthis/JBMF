mod tests {
    use std::fs::File;
    use std::io::{BufReader, BufWriter};

    use crate::{AccessFlags, Constant, ConstantPoolIndex, Error, Field, JavaClass, JavaVersion, MagicNumber, SizedVec};
    use crate::attribute::Attribute;
    use crate::helper::*;

    #[test]
    fn check_constant_value() -> Result<(), Error> {
        let (path, _guard) = init_tmp_dir("CheckConstantIntegerTest.class");

        let reference = JavaClass {
            magic: MagicNumber::Cafebabe,
            version: JavaVersion { major: 45, minor: 3 },
            constant_pool: vec![
                Constant::Class(ConstantPoolIndex(3)),
                Constant::Class(ConstantPoolIndex(4)),
                Constant::Utf8("CheckConstantIntegerTest".into()),
                Constant::Utf8("java/lang/Object".into()),
                Constant::Utf8("integer".into()),
                Constant::Utf8("I".into()),
                Constant::Utf8("ConstantValue".into()),
                Constant::Integer(123456),
                Constant::Utf8("float".into()),
                Constant::Utf8("F".into()),
                Constant::Utf8("ConstantValue".into()),
                Constant::Float(123.456),
                Constant::Utf8("long".into()),
                Constant::Utf8("J".into()),
                Constant::Utf8("ConstantValue".into()),
                Constant::Long(123456),
                Constant::Unusable,
                Constant::Utf8("double".into()),
                Constant::Utf8("D".into()),
                Constant::Utf8("ConstantValue".into()),
                Constant::Double(123.456),
                Constant::Unusable,
                Constant::Utf8("string".into()),
                Constant::Utf8("Ljava/lang/String;".into()),
                Constant::Utf8("ConstantValue".into()),
                Constant::String(ConstantPoolIndex(25)),
                Constant::Utf8("Hello, world!".into()),
            ]
                .into(),
            access_flags: AccessFlags::NONE,
            this_class: ConstantPoolIndex(1),
            super_class: ConstantPoolIndex(2),
            interfaces: SizedVec::new(),
            fields: vec![
                Field {
                    access_flags: AccessFlags::NONE,
                    name: ConstantPoolIndex(5),
                    descriptor: ConstantPoolIndex(6),
                    attributes: vec![Attribute::ConstantValue {
                        name: ConstantPoolIndex(7),
                        value: ConstantPoolIndex(8)
                    }].into()
                },
                Field {
                    access_flags: AccessFlags::NONE,
                    name: ConstantPoolIndex(9),
                    descriptor: ConstantPoolIndex(10),
                    attributes: vec![Attribute::ConstantValue {
                        name: ConstantPoolIndex(11),
                        value: ConstantPoolIndex(12)
                    }].into()
                },
                Field {
                    access_flags: AccessFlags::NONE,
                    name: ConstantPoolIndex(13),
                    descriptor: ConstantPoolIndex(14),
                    attributes: vec![Attribute::ConstantValue {
                        name: ConstantPoolIndex(15),
                        value: ConstantPoolIndex(16)
                    }].into()
                },
                Field {
                    access_flags: AccessFlags::NONE,
                    name: ConstantPoolIndex(18),
                    descriptor: ConstantPoolIndex(19),
                    attributes: vec![Attribute::ConstantValue {
                        name: ConstantPoolIndex(20),
                        value: ConstantPoolIndex(21)
                    }].into()
                },
                Field {
                    access_flags: AccessFlags::NONE,
                    name: ConstantPoolIndex(23),
                    descriptor: ConstantPoolIndex(24),
                    attributes: vec![Attribute::ConstantValue {
                        name: ConstantPoolIndex(25),
                        value: ConstantPoolIndex(26)
                    }].into()
                }
            ].into(),
            methods: SizedVec::new(),
            attributes: SizedVec::new(),
        };

        reference.write(&mut BufWriter::new(File::create(&path)?))?;
        assert_eq!(reference, JavaClass::read(&mut BufReader::new(File::open(path)?))?);
        Ok(())
    }
}
