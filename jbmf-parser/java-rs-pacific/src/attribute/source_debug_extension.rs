use std::io::{Read, Write};

use java_rs_base::error::Error;
use java_rs_base::io::{ReadContext, SizedVec, WriteContext};

use crate::Attribute;

pub(crate) struct SourceDebugExtensionIO;

impl SourceDebugExtensionIO {
    pub(crate) fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Attribute, Error>
    where
        Self: std::marker::Sized,
    {
        let items = SizedVec::<u32, u8>::read_without_size(ctx.length.unwrap(), reader, ctx)?;

        Ok(Attribute::SourceDebugExtension {
            name: ctx.name.unwrap(),
            debug_extensions: items,
        })
    }

    pub(crate) fn write<W: Write>(attribute: &Attribute, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        match attribute {
            Attribute::SourceDebugExtension { debug_extensions, .. } => {
                debug_extensions.write_without_size(writer, ctx)?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}
