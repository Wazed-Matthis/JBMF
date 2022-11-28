use java_rs_base::constant_pool::ConstantPoolIndex;
use java_rs_base::io::SizedVec;
use java_rs_derive::ClassFilePart;

use crate::flags::ModuleDependencyFlags;

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct ModuleRequires {
    requires: ConstantPoolIndex,
    flags: ModuleDependencyFlags,
    version: ConstantPoolIndex,
}

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct ModuleExports {
    export: ConstantPoolIndex,
    flags: ModuleDependencyFlags,
    to: SizedVec<u16, ConstantPoolIndex>,
}

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct ModuleOpens {
    open: ConstantPoolIndex,
    flags: ModuleDependencyFlags,
    to_index: SizedVec<u16, ConstantPoolIndex>,
}

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct ModuleProvides {
    provide: ConstantPoolIndex,
    with_index: SizedVec<u16, ConstantPoolIndex>,
}
