use std::fmt::{Debug, Formatter};
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::constant_pool::{ConstantPool, ConstantPoolIndex};
use crate::error::Error;
use crate::java_utf8::{FromJavaUtf8Ext, ToJavaUtf8Ext};
use crate::version::JavaVersion;

#[derive(Debug)]
pub struct ReadContext<'a> {
    pub version: &'a JavaVersion,
    pub constant_pool: &'a ConstantPool,
    pub location: Option<&'a AttributeLocation>,
    pub name: Option<ConstantPoolIndex>,
    pub position: Option<u64>,
    pub length: Option<u32>,
    pub wide: Option<bool>,
}

#[derive(Debug)]
pub struct WriteContext {
    pub position: Option<u64>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum AttributeLocation {
    ClassFile,
    Field,
    Method,
    Code,
}

#[derive(Default, Clone, PartialEq)]
pub struct SizedVec<S: ClassFilePartSize, T: ClassFilePart> {
    inner: Vec<T>,
    _size_type: PhantomData<S>,
}

impl<S: ClassFilePartSize, T: ClassFilePart> From<Vec<T>> for SizedVec<S, T> {
    fn from(inner: Vec<T>) -> Self {
        SizedVec {
            inner,
            _size_type: PhantomData,
        }
    }
}

impl<S: ClassFilePartSize, T: ClassFilePart> Eq for SizedVec<S, T> where T: Eq {}

impl<S: ClassFilePartSize, T: ClassFilePart> SizedVec<S, T> {
    pub fn new() -> SizedVec<S, T> {
        SizedVec {
            inner: Vec::new(),
            _size_type: PhantomData,
        }
    }

    pub fn read_without_size<R: Read>(size: S, reader: &mut R, ctx: &ReadContext) -> Result<Self, Error> {
        let size = size.to_usize();
        let mut inner = Vec::with_capacity(size);

        for _ in 0..size {
            inner.push(T::read(reader, ctx)?);
        }

        Ok(SizedVec {
            inner,
            _size_type: PhantomData,
        })
    }

    pub fn write_without_size<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        for v in &self.inner {
            v.write(writer, ctx)?;
        }

        Ok(())
    }

    pub fn inner(self) -> Vec<T> {
        self.inner
    }
}

impl<S: ClassFilePartSize> Write for SizedVec<S, u8> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        for byte in buf {
            self.inner.push(*byte);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

impl<S: ClassFilePartSize, T: ClassFilePart + Debug> Debug for SizedVec<S, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

pub trait ClassFilePart: Clone + PartialEq {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized;

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error>;
}

pub trait ClassFilePartSize: ClassFilePart {
    fn to_usize(&self) -> usize;

    fn from_usize(size: usize) -> Self;
}

impl ClassFilePart for u8 {
    fn read<R: Read>(reader: &mut R, _: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        Ok(reader.read_u8()?)
    }

    fn write<W: Write>(&self, writer: &mut W, _: &WriteContext) -> Result<(), Error> {
        writer.write_u8(*self)?;
        Ok(())
    }
}

impl ClassFilePartSize for u8 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(size: usize) -> Self {
        size as u8
    }
}

impl ClassFilePart for u16 {
    fn read<R: Read>(reader: &mut R, _: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        Ok(reader.read_u16::<BigEndian>()?)
    }

    fn write<W: Write>(&self, writer: &mut W, _: &WriteContext) -> Result<(), Error> {
        writer.write_u16::<BigEndian>(*self)?;
        Ok(())
    }
}

impl ClassFilePartSize for u16 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(size: usize) -> Self {
        size as u16
    }
}

impl ClassFilePart for u32 {
    fn read<R: Read>(reader: &mut R, _: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        Ok(reader.read_u32::<BigEndian>()?)
    }

    fn write<W: Write>(&self, writer: &mut W, _: &WriteContext) -> Result<(), Error> {
        writer.write_u32::<BigEndian>(*self)?;
        Ok(())
    }
}

impl ClassFilePartSize for u32 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(size: usize) -> Self {
        size as u32
    }
}

impl ClassFilePart for i16 {
    fn read<R: Read>(reader: &mut R, _: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        Ok(reader.read_i16::<BigEndian>()?)
    }

    fn write<W: Write>(&self, writer: &mut W, _: &WriteContext) -> Result<(), Error> {
        writer.write_i16::<BigEndian>(*self)?;
        Ok(())
    }
}

impl ClassFilePart for i32 {
    fn read<R: Read>(reader: &mut R, _: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        Ok(reader.read_i32::<BigEndian>()?)
    }

    fn write<W: Write>(&self, writer: &mut W, _: &WriteContext) -> Result<(), Error> {
        writer.write_i32::<BigEndian>(*self)?;
        Ok(())
    }
}

impl ClassFilePartSize for i32 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(size: usize) -> Self {
        size as i32
    }
}

impl ClassFilePart for i64 {
    fn read<R: Read>(reader: &mut R, _: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        Ok(reader.read_i64::<BigEndian>()?)
    }

    fn write<W: Write>(&self, writer: &mut W, _: &WriteContext) -> Result<(), Error> {
        writer.write_i64::<BigEndian>(*self)?;
        Ok(())
    }
}

impl ClassFilePart for f32 {
    fn read<R: Read>(reader: &mut R, _: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        Ok(reader.read_f32::<BigEndian>()?)
    }

    fn write<W: Write>(&self, writer: &mut W, _: &WriteContext) -> Result<(), Error> {
        writer.write_f32::<BigEndian>(*self)?;
        Ok(())
    }
}

impl ClassFilePart for f64 {
    fn read<R: Read>(reader: &mut R, _: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        Ok(reader.read_f64::<BigEndian>()?)
    }

    fn write<W: Write>(&self, writer: &mut W, _: &WriteContext) -> Result<(), Error> {
        writer.write_f64::<BigEndian>(*self)?;
        Ok(())
    }
}

impl<S: ClassFilePartSize, T: ClassFilePart> ClassFilePart for SizedVec<S, T> {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        let length = S::read(reader, ctx)?;
        Self::read_without_size(length, reader, ctx)
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        S::from_usize(self.inner.len()).write(writer, ctx)?;
        self.write_without_size(writer, ctx)
    }
}

impl<S: ClassFilePartSize, T: ClassFilePart> Deref for SizedVec<S, T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S: ClassFilePartSize, T: ClassFilePart> DerefMut for SizedVec<S, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: ClassFilePart> ClassFilePart for Option<T> {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        Ok(Some(ClassFilePart::read(reader, ctx)?))
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        if let Some(v) = self {
            v.write(writer, ctx)?;
        }
        Ok(())
    }
}

impl ClassFilePart for usize {
    fn read<R: Read>(_: &mut R, _: &ReadContext) -> Result<Self, Error>
    where
        Self: Sized,
    {
        unimplemented!("Dummy stub, do not use ClassFilePart for usize as it is platform specific")
    }

    fn write<W: Write>(&self, _: &mut W, _: &WriteContext) -> Result<(), Error> {
        unimplemented!("Dummy stub, do not use ClassFilePart for usize as it is platform specific")
    }
}

impl ClassFilePartSize for usize {
    fn to_usize(&self) -> usize {
        *self
    }

    fn from_usize(size: usize) -> Self {
        size
    }
}

impl<T: ClassFilePart> ClassFilePart for Vec<T> {
    fn read<R: Read>(_reader: &mut R, _ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        unimplemented!("vec is not supposed to be read")
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        for item in self {
            item.write(writer, ctx)?;
        }
        Ok(())
    }
}

impl ClassFilePart for String {
    fn read<R: Read>(reader: &mut R, _: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        let length = reader.read_u16::<BigEndian>()?;

        let bytes = {
            let mut bytes = vec![0u8; length as usize];
            reader.read_exact(&mut bytes)?;
            bytes
        };

        String::from_java_utf8(&bytes)
    }

    fn write<W: Write>(&self, writer: &mut W, _: &WriteContext) -> Result<(), Error> {
        let bytes = self.to_java_utf8()?;
        writer.write_u16::<BigEndian>(bytes.len() as u16)?;
        writer.write_all(&bytes)?;
        Ok(())
    }
}
