use std::io::{Read, Write};

use annotation::Annotation;
pub use bootstrap_methods::*;
pub use code::*;
pub use inner_classes::*;
use java_rs_base::constant_pool::ConstantPoolIndex;
use java_rs_base::error::Error;
use java_rs_base::io::{ClassFilePart, ReadContext, SizedVec, WriteContext};
use java_rs_derive::ClassFilePart;
pub use line_number_table::*;
pub use local_variable_table::*;
pub use local_variable_type_table::*;
pub use method_parameters::*;
pub use module::*;
pub use stack_map::*;
use type_annotation::TypeAnnotation;

use crate::attribute::annotation::ElementValue;
use crate::flags::ModuleFlags;

mod annotation;
mod bootstrap_methods;
mod code;
mod inner_classes;
mod line_number_table;
mod local_variable_table;
mod local_variable_type_table;
mod method_parameters;
mod module;
mod source_debug_extension;
mod stack_map;
mod type_annotation;

#[cfg(test)]
mod constant_value;

#[cfg(test)]
mod enclosing_method;

#[cfg(test)]
mod synthetic;

#[derive(Debug, Eq, PartialEq)]
pub struct EnumDescriptor {
    name_index: ConstantPoolIndex,
    attribute: Attribute,
}

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
#[java_rs(generator = "attribute")]
pub enum Attribute {
    #[java_rs(version = 45.3, location = Field)]
    ConstantValue {
        name: ConstantPoolIndex,
        value: ConstantPoolIndex,
    },
    #[java_rs(io_implementation = code::CodeIO, version = 45.3, location = Method)]
    Code {
        name: ConstantPoolIndex,
        max_stack: Compatibility<u8, u16>,
        max_locals: Compatibility<u8, u16>,
        code: Compatibility<SizedVec<u16, Instruction>, SizedVec<u32, Instruction>>,
        exception_table: SizedVec<u16, ExceptionTable>,
        attributes: SizedVec<u16, Attribute>,
    },
    #[java_rs(version = 50.0, location = Code)]
    StackMapTable {
        name: ConstantPoolIndex,
        entries: SizedVec<u16, StackMapFrame>,
    },
    #[java_rs(version = 45.3, location = Method)]
    Exceptions {
        name: ConstantPoolIndex,
        exception_index_table: SizedVec<u16, ConstantPoolIndex>,
    },
    #[java_rs(version = 45.3, location = ClassFile)]
    InnerClasses {
        name: ConstantPoolIndex,
        classes: SizedVec<u16, InnerClass>,
    },
    #[java_rs(version = 49.0, location = ClassFile)]
    EnclosingMethod {
        name: ConstantPoolIndex,
        class: ConstantPoolIndex,
        method: ConstantPoolIndex,
    },
    #[java_rs(version = 45.3, location = [ClassFile, Field, Method])]
    Synthetic {
        name: ConstantPoolIndex,
    },
    #[java_rs(version = 49.0, location = [ClassFile, Field, Method])]
    Signature {
        name: ConstantPoolIndex,
        signature: ConstantPoolIndex,
    },
    #[java_rs(version = 45.3, location = ClassFile)]
    SourceFile {
        name: ConstantPoolIndex,
        sourcefile: ConstantPoolIndex,
    },
    #[java_rs(io_implementation = source_debug_extension::SourceDebugExtensionIO, version = 49.0, location = ClassFile)]
    SourceDebugExtension {
        name: ConstantPoolIndex,
        debug_extensions: SizedVec<u32, u8>,
    },
    #[java_rs(version = 45.3, location = Code)]
    LineNumberTable {
        name: ConstantPoolIndex,
        line_numbers: SizedVec<u16, LineNumberTable>,
    },
    #[java_rs(version = 45.3, location = Code)]
    LocalVariableTable {
        name: ConstantPoolIndex,
        local_variables: SizedVec<u16, LocalVariableTable>,
    },
    #[java_rs(version = 49.0, location = Code)]
    LocalVariableTypeTable {
        name: ConstantPoolIndex,
        local_variable_type_table: SizedVec<u16, LocalVariableTypeTable>,
    },
    #[java_rs(version = 45.3, location = [ClassFile, Field, Method])]
    Deprecated {
        name: ConstantPoolIndex,
    },
    #[java_rs(version = 49.0, location = [ClassFile, Field, Method])]
    RuntimeVisibleAnnotations {
        name: ConstantPoolIndex,
        annotations: SizedVec<u16, Annotation>,
    },
    #[java_rs(version = 49.0, location = [ClassFile, Field, Method])]
    RuntimeInvisibleAnnotations {
        name: ConstantPoolIndex,
        annotations: SizedVec<u16, Annotation>,
    },
    #[java_rs(version = 49.0, location = Method)]
    RuntimeVisibleParameterAnnotations {
        name: ConstantPoolIndex,
        annotations: SizedVec<u8, SizedVec<u16, Annotation>>,
    },
    #[java_rs(version = 49.0, location = Method)]
    RuntimeInvisibleParameterAnnotations {
        name: ConstantPoolIndex,
        annotations: SizedVec<u8, SizedVec<u16, Annotation>>,
    },
    #[java_rs(version = 52.0, location = [ClassFile, Field, Method, Code])]
    RuntimeVisibleTypeAnnotations {
        name: ConstantPoolIndex,
        annotations: SizedVec<u16, TypeAnnotation>,
    },
    #[java_rs(version = 52.0, location = [ClassFile, Field, Method, Code])]
    RuntimeInvisibleTypeAnnotations {
        name: ConstantPoolIndex,
        annotations: SizedVec<u16, TypeAnnotation>,
    },
    #[java_rs(version = 49.0, location = Method)]
    AnnotationDefault {
        name: ConstantPoolIndex,
        default: ElementValue,
    },
    #[java_rs(version = 51.0, location = ClassFile)]
    BootstrapMethods {
        name: ConstantPoolIndex,
        methods: SizedVec<u16, BootstrapMethod>,
    },
    #[java_rs(version = 52.0, location = Method)]
    MethodParameters {
        name: ConstantPoolIndex,
        parameters: SizedVec<u16, MethodParameter>,
    },
    #[java_rs(version = 53.0, location = ClassFile)]
    Module {
        name: ConstantPoolIndex,
        module_name: ConstantPoolIndex,
        flags: ModuleFlags,
        module_version: ConstantPoolIndex,
        requires: ModuleRequires,
        exports: ModuleExports,
        opens: ModuleOpens,
        uses: SizedVec<u16, ConstantPoolIndex>,
        provides: ModuleProvides,
    },
    #[java_rs(version = 53.0, location = ClassFile)]
    ModulePackages {
        name: ConstantPoolIndex,
        packages: SizedVec<u16, ConstantPoolIndex>,
    },
    #[java_rs(version = 53.0, location = ClassFile)]
    ModuleMainClass {
        name: ConstantPoolIndex,
        main_class: ConstantPoolIndex,
    },
    #[java_rs(version = 55.0, location = ClassFile)]
    NestHost {
        name: ConstantPoolIndex,
        host_class: ConstantPoolIndex,
    },
    #[java_rs(version = 55.0, location = ClassFile)]
    NestMembers {
        name: ConstantPoolIndex,
        classes: SizedVec<u16, ConstantPoolIndex>,
    },
    InvalidUtf8(RawAttribute),
    IllegalNameReference(RawAttribute),
    UnsupportedAndInvalidLocation(Box<Attribute>),
    InvalidLocation(Box<Attribute>),
    Unsupported(Box<Attribute>),
    Unknown(RawAttribute),
    Raw(RawAttribute),
    Custom(CustomAttribute),
}

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct RawAttribute {
    pub name: ConstantPoolIndex,
    pub info: SizedVec<u32, u8>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CustomAttribute {
    pub name: ConstantPoolIndex,
    pub length: u32,
    pub info: Vec<u8>,
}

impl ClassFilePart for CustomAttribute {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        let name = ctx.name.unwrap();
        let length = ctx.length.unwrap();
        let info = &*SizedVec::read_without_size(length, reader, ctx)?;

        Ok(Self {
            name,
            length,
            info: info.to_vec(),
        })
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        self.name.write(writer, ctx)?;
        self.length.write(writer, ctx)?;
        writer.write_all(&self.info)?;
        Ok(())
    }
}

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}
