use java_rs_base::constant_pool::ConstantPoolIndex;
use java_rs_derive::ClassFilePart;

use crate::flags::AccessFlags;

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct InnerClass {
    inner_class: ConstantPoolIndex,
    outer_class: ConstantPoolIndex,
    inner_name: ConstantPoolIndex,
    inner_class_access_flags: AccessFlags,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, BufWriter};

    use crate::{AccessFlags, Constant, ConstantPoolIndex, Error, JavaClass, JavaVersion, MagicNumber, SizedVec};
    use crate::attribute::{Attribute, InnerClass};
    use crate::helper::*;

    #[test]
    fn check_inner_classes() -> Result<(), Error> {
        let (path, _guard) = init_tmp_dir("InnerClassesTest.class");

        let reference = JavaClass {
            magic: MagicNumber::Cafebabe,
            version: JavaVersion { major: 45, minor: 3 },
            constant_pool: vec![
                Constant::Class(ConstantPoolIndex(3)),
                Constant::Class(ConstantPoolIndex(4)),
                Constant::Utf8("InnerClassesTest".into()),
                Constant::Utf8("java/lang/Object".into()),
                Constant::Utf8("InnerClasses".into()),
                Constant::Class(ConstantPoolIndex(7)),
                Constant::Utf8("InnerClass".into()),
            ]
                .into(),
            access_flags: AccessFlags::NONE,
            this_class: ConstantPoolIndex(1),
            super_class: ConstantPoolIndex(2),
            interfaces: SizedVec::new(),
            fields: SizedVec::new(),
            methods: SizedVec::new(),
            attributes: vec![Attribute::InnerClasses {
                name: ConstantPoolIndex(5),
                classes: vec![InnerClass {
                    inner_class: ConstantPoolIndex(6),
                    outer_class: ConstantPoolIndex(1),
                    inner_name: ConstantPoolIndex(8),
                    inner_class_access_flags: AccessFlags::PUBLIC | AccessFlags::STATIC | AccessFlags::FINAL
                }].into()
            }].into(),
        };

        reference.write(&mut BufWriter::new(File::create(&path)?))?;
        assert_eq!(reference, JavaClass::read(&mut BufReader::new(File::open(path)?))?);
        Ok(())
    }
}
