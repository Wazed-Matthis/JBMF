mod tests {
    use std::fs::File;
    use std::io::{BufReader, BufWriter};

    use crate::{AccessFlags, Constant, ConstantPoolIndex, Error, JavaClass, JavaVersion, MagicNumber, SizedVec};
    use crate::attribute::Attribute;
    use crate::helper::*;

    #[test]
    fn check_enclosing_method() -> Result<(), Error> {
        let (path, _guard) = init_tmp_dir("EnclosingMethodTest.class");

        let reference = JavaClass {
            magic: MagicNumber::Cafebabe,
            version: JavaVersion { major: 49, minor: 0 },
            constant_pool: vec![
                Constant::Class(ConstantPoolIndex(3)),
                Constant::Class(ConstantPoolIndex(4)),
                Constant::Utf8("EnclosingMethodTest".into()),
                Constant::Utf8("java/lang/Object".into()),
                Constant::Utf8("EnclosingMethod".into()),
                Constant::Class(ConstantPoolIndex(7)),
                Constant::Utf8("EnclosingClass".into()),
            ]
                .into(),
            access_flags: AccessFlags::NONE,
            this_class: ConstantPoolIndex(1),
            super_class: ConstantPoolIndex(2),
            interfaces: SizedVec::new(),
            fields: SizedVec::new(),
            methods: SizedVec::new(),
            attributes: vec![Attribute::EnclosingMethod {
                name: ConstantPoolIndex(5),
                class: ConstantPoolIndex(6),
                method: ConstantPoolIndex(0)
            }].into(),
        };

        reference.write(&mut BufWriter::new(File::create(&path)?))?;
        assert_eq!(reference, JavaClass::read(&mut BufReader::new(File::open(path)?))?);
        Ok(())
    }

    #[test]
    fn check_enclosing_method_within_method() -> Result<(), Error> {
        let (path, _guard) = init_tmp_dir("EnclosingMethodWithinMethodTest.class");

        let reference = JavaClass {
            magic: MagicNumber::Cafebabe,
            version: JavaVersion { major: 49, minor: 0 },
            constant_pool: vec![
                Constant::Class(ConstantPoolIndex(3)),
                Constant::Class(ConstantPoolIndex(4)),
                Constant::Utf8("EnclosingMethodWithinMethodTest".into()),
                Constant::Utf8("java/lang/Object".into()),
                Constant::Utf8("EnclosingMethod".into()),
                Constant::Class(ConstantPoolIndex(7)),
                Constant::Utf8("EnclosingClass".into()),
                Constant::NameAndType {
                    name: ConstantPoolIndex(10),
                    descriptor: ConstantPoolIndex(11)
                },
                Constant::Utf8("enclosingMethod".into()),
                Constant::Utf8("()V".into()),
            ]
                .into(),
            access_flags: AccessFlags::NONE,
            this_class: ConstantPoolIndex(1),
            super_class: ConstantPoolIndex(2),
            interfaces: SizedVec::new(),
            fields: SizedVec::new(),
            methods: SizedVec::new(),
            attributes: vec![Attribute::EnclosingMethod {
                name: ConstantPoolIndex(5),
                class: ConstantPoolIndex(6),
                method: ConstantPoolIndex(9)
            }].into(),
        };

        reference.write(&mut BufWriter::new(File::create(&path)?))?;
        assert_eq!(reference, JavaClass::read(&mut BufReader::new(File::open(path)?))?);
        Ok(())
    }
}
