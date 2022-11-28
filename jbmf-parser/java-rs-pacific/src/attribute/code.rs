use std::io::{Cursor, Read, Write};
use std::ops::Deref;

use java_rs_base::error::Error;
use java_rs_base::io::{AttributeLocation, ClassFilePart, ClassFilePartSize, ReadContext, SizedVec, WriteContext};
use java_rs_derive::ClassFilePart;

use super::Attribute;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Compatibility<Pre: ClassFilePart + Eq, Current: ClassFilePart + Eq> {
    PreJava1(Pre),
    Current(Current),
}

impl<Pre: ClassFilePart + Eq, Current: ClassFilePart + Eq> ClassFilePart for Compatibility<Pre, Current> {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        if ctx.version.supports(45, 3) {
            Ok(Self::Current(ClassFilePart::read(reader, ctx)?))
        } else {
            Ok(Self::PreJava1(ClassFilePart::read(reader, ctx)?))
        }
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        match self {
            Self::Current(value) => value.write(writer, ctx),
            Self::PreJava1(value) => value.write(writer, ctx),
        }
    }
}

impl<Pre: ClassFilePartSize, Current: ClassFilePartSize, ItemType: ClassFilePart + Eq>
    Compatibility<SizedVec<Pre, ItemType>, SizedVec<Current, ItemType>>
{
    pub fn from(vec: Vec<ItemType>, ctx: &ReadContext) -> Self {
        if ctx.version.supports(45, 3) {
            Self::Current(vec.into())
        } else {
            Self::PreJava1(vec.into())
        }
    }
}

pub struct CodeIO;

impl CodeIO {
    pub fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Attribute, Error>
    where
        Self: Sized,
    {
        let mut ctx = ReadContext {
            location: Some(&AttributeLocation::Code),
            wide: Some(false),
            ..*ctx
        };

        let max_stack = ClassFilePart::read(reader, &ctx)?;
        let max_locals = ClassFilePart::read(reader, &ctx)?;

        let mut code = Vec::new();

        let data = {
            match Compatibility::<SizedVec<u16, u8>, SizedVec<u32, u8>>::read(reader, &ctx)? {
                Compatibility::Current(value) => value.inner(),
                Compatibility::PreJava1(value) => value.inner(),
            }
        };
        let size = data.len();
        let mut code_reader = Cursor::new(data);

        while (code_reader.position() as usize) < size {
            ctx = ReadContext {
                position: Some(code_reader.position()),
                ..ctx
            };

            let instruction = ClassFilePart::read(&mut code_reader, &ctx)?;

            ctx = match instruction {
                Instruction::Wide => {
                    if ctx.wide.is_some() && ctx.wide.unwrap() {
                        ctx
                    } else {
                        ReadContext {
                            wide: Some(true),
                            ..ctx
                        }
                    }
                }
                _ => {
                    if ctx.wide.is_none() || ctx.wide.unwrap() {
                        ReadContext {
                            wide: Some(false),
                            ..ctx
                        }
                    } else {
                        ctx
                    }
                }
            };

            code.push(instruction);
        }

        Ok(Attribute::Code {
            name: ctx.name.unwrap(),
            max_stack,
            max_locals,
            code: Compatibility::from(code, &ctx),
            exception_table: ClassFilePart::read(reader, &ctx)?,
            attributes: ClassFilePart::read(reader, &ctx)?,
        })
    }

    pub fn write<W: Write>(attribute: &Attribute, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        match attribute {
            Attribute::Code {
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes,
                ..
            } => {
                max_stack.write(writer, ctx)?;
                max_locals.write(writer, ctx)?;

                let buffer = {
                    let mut buffer = Vec::new();

                    {
                        let mut writer = Cursor::new(&mut buffer);

                        let instructions = match code {
                            Compatibility::PreJava1(value) => value.deref(),
                            Compatibility::Current(value) => value.deref(),
                        };

                        for instruction in instructions {
                            let pos = writer.position();
                            instruction.write(&mut writer, &WriteContext { position: Some(pos) })?;
                        }
                    }

                    buffer
                };

                if let Compatibility::Current(_) = code {
                    (buffer.len() as u32).write(writer, ctx)?;
                } else {
                    (buffer.len() as u16).write(writer, ctx)?;
                }

                buffer.write(writer, ctx)?;

                exception_table.write(writer, ctx)?;
                attributes.write(writer, ctx)?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SizedIndex {
    Normal(SmallIndex),
    Wide(WideIndex),
}

impl ClassFilePart for SizedIndex {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match ctx.wide.unwrap() {
            false => SmallIndex::read(reader, ctx).map(SizedIndex::Normal),
            true => WideIndex::read(reader, ctx).map(SizedIndex::Wide),
        }
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        match self {
            SizedIndex::Normal(v) => v.write(writer, ctx),
            SizedIndex::Wide(v) => v.write(writer, ctx),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct SmallIndex(pub u8);

impl ClassFilePart for SmallIndex {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self(u8::read(reader, ctx)?))
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        u8::write(&self.0, writer, ctx)
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct WideIndex(pub u16);

impl ClassFilePart for WideIndex {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        Ok(Self(u16::read(reader, ctx)?))
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        u16::write(&self.0, writer, ctx)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AlwaysZero;

impl ClassFilePart for AlwaysZero {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match u8::read(reader, ctx)? {
            0 => Ok(AlwaysZero),
            v => Err(Error::UnexpectedOpCodeValue {
                expected: (0, 0),
                found: v,
            }),
        }
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        0u8.write(writer, ctx)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArrayType {
    Boolean,
    Char,
    Float,
    Double,
    Byte,
    Short,
    Int,
    Long,
}

impl ClassFilePart for ArrayType {
    fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let code = u8::read(reader, ctx)?;

        match code {
            4 => Ok(ArrayType::Boolean),
            5 => Ok(ArrayType::Char),
            6 => Ok(ArrayType::Float),
            7 => Ok(ArrayType::Double),
            8 => Ok(ArrayType::Byte),
            9 => Ok(ArrayType::Short),
            10 => Ok(ArrayType::Int),
            11 => Ok(ArrayType::Long),
            v => Err(Error::UnexpectedOpCodeValue {
                expected: (4, 11),
                found: v,
            }),
        }
    }

    fn write<W: Write>(&self, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        let code: u8 = match self {
            ArrayType::Boolean => 4,
            ArrayType::Char => 5,
            ArrayType::Float => 6,
            ArrayType::Double => 7,
            ArrayType::Byte => 8,
            ArrayType::Short => 9,
            ArrayType::Int => 10,
            ArrayType::Long => 11,
        };

        code.write(writer, ctx)
    }
}

struct TableSwitchIO;

impl TableSwitchIO {
    pub fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Instruction, Error>
    where
        Self: Sized,
    {
        let pos = ctx.position.unwrap() + 1;
        let remainder = pos % 4;
        let align = if remainder == 0 { remainder } else { 4 - remainder };

        let mut _align_buffer = vec![0u8; align as usize];
        reader.read_exact(&mut _align_buffer)?;

        let default = i32::read(reader, ctx)?;
        let low = i32::read(reader, ctx)?;
        let high = i32::read(reader, ctx)?;

        let offsets = SizedVec::<u32, i32>::read_without_size((high - low + 1) as u32, reader, ctx)?;

        Ok(Instruction::TableSwitch {
            default,
            low,
            high,
            offsets,
        })
    }

    pub fn write<W: Write>(instruction: &Instruction, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        match instruction {
            Instruction::TableSwitch {
                default,
                low,
                high,
                offsets,
            } => {
                let pos = ctx.position.unwrap() + 1;
                let remainder = pos % 4;
                let align = if remainder == 0 { remainder } else { 4 - remainder };

                let mut _align_buffer = vec![0u8; align as usize];
                writer.write_all(&_align_buffer)?;

                default.write(writer, ctx)?;
                low.write(writer, ctx)?;
                high.write(writer, ctx)?;

                offsets.write_without_size(writer, ctx)?;

                Ok(())
            }
            _ => panic!("Received unexpected instruction {:?} in TableSwitchIO", instruction),
        }
    }
}

struct LookupSwitchIO;

impl LookupSwitchIO {
    pub fn read<R: Read>(reader: &mut R, ctx: &ReadContext) -> Result<Instruction, Error>
    where
        Self: std::marker::Sized,
    {
        let pos = ctx.position.unwrap() + 1;
        let remainder = pos % 4;
        let align = if remainder == 0 { remainder } else { 4 - remainder };

        let mut _align_buffer = vec![0u8; align as usize];
        reader.read_exact(&mut _align_buffer)?;

        let default = i32::read(reader, ctx)?;
        let pairs = SizedVec::<i32, MatchOffsetPair>::read(reader, ctx)?;

        Ok(Instruction::LookUpSwitch { default, pairs })
    }

    pub fn write<W: Write>(instruction: &Instruction, writer: &mut W, ctx: &WriteContext) -> Result<(), Error> {
        match instruction {
            Instruction::LookUpSwitch { default, pairs } => {
                let pos = ctx.position.unwrap() + 1;
                let remainder = pos % 4;
                let align = if remainder == 0 { remainder } else { 4 - remainder };

                let mut _align_buffer = vec![0u8; align as usize];
                writer.write_all(&_align_buffer)?;
                _align_buffer.len();

                default.write(writer, ctx)?;
                pairs.write(writer, ctx)?;

                Ok(())
            }
            _ => panic!("Received unexpected instruction {:?} in LookupSwitchIO", instruction),
        }
    }
}

#[derive(Debug, ClassFilePart, Copy, Clone, Eq, PartialEq)]
pub struct MatchOffsetPair {
    pub match_value: i32,
    pub offset: i32,
}

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
#[java_rs(generator = "code")]
pub enum Instruction {
    #[java_rs(opcode = 0x32)]
    AALoad,
    #[java_rs(opcode = 0x53)]
    AAStore,
    #[java_rs(opcode = 0x01)]
    AConstNull,
    #[java_rs(opcode = 0x19)]
    ALoad { index: SizedIndex },
    #[java_rs(opcode = 0x2A)]
    ALoad0,
    #[java_rs(opcode = 0x2B)]
    ALoad1,
    #[java_rs(opcode = 0x2C)]
    ALoad2,
    #[java_rs(opcode = 0x2D)]
    ALoad3,
    #[java_rs(opcode = 0xBD)]
    ANewArray { index: WideIndex },
    #[java_rs(opcode = 0xB0)]
    AReturn,
    #[java_rs(opcode = 0xBE)]
    ArrayLength,
    #[java_rs(opcode = 0x3A)]
    AStore { index: SizedIndex },
    #[java_rs(opcode = 0x4B)]
    AStore0,
    #[java_rs(opcode = 0x4C)]
    AStore1,
    #[java_rs(opcode = 0x4D)]
    AStore2,
    #[java_rs(opcode = 0x4E)]
    AStore3,
    #[java_rs(opcode = 0xBF)]
    AThrow,
    #[java_rs(opcode = 0x33)]
    BALoad,
    #[java_rs(opcode = 0x54)]
    BAStore,
    #[java_rs(opcode = 0x10)]
    BIPush { value: u8 },
    #[java_rs(opcode = 0x34)]
    CALoad,
    #[java_rs(opcode = 0x55)]
    CAStore,
    #[java_rs(opcode = 0xC0)]
    CheckCast { index: WideIndex },
    #[java_rs(opcode = 0x90)]
    D2F,
    #[java_rs(opcode = 0x8E)]
    D2I,
    #[java_rs(opcode = 0x8F)]
    D2L,
    #[java_rs(opcode = 0x63)]
    DAdd,
    #[java_rs(opcode = 0x31)]
    DALoad,
    #[java_rs(opcode = 0x52)]
    DAStore,
    #[java_rs(opcode = 0x98)]
    DCmpG,
    #[java_rs(opcode = 0x97)]
    DCmpL,
    #[java_rs(opcode = 0xE)]
    DConst0,
    #[java_rs(opcode = 0xF)]
    DConst1,
    #[java_rs(opcode = 0x6F)]
    DDiv,
    #[java_rs(opcode = 0x18)]
    DLoad { index: SizedIndex },
    #[java_rs(opcode = 0x26)]
    DLoad0,
    #[java_rs(opcode = 0x27)]
    DLoad1,
    #[java_rs(opcode = 0x28)]
    DLoad2,
    #[java_rs(opcode = 0x29)]
    DLoad3,
    #[java_rs(opcode = 0x6B)]
    DMul,
    #[java_rs(opcode = 0x77)]
    DNeg,
    #[java_rs(opcode = 0x73)]
    DRem,
    #[java_rs(opcode = 0xAF)]
    DReturn,
    #[java_rs(opcode = 0x39)]
    DStore { index: SizedIndex },
    #[java_rs(opcode = 0x47)]
    DStore0,
    #[java_rs(opcode = 0x48)]
    DStore1,
    #[java_rs(opcode = 0x49)]
    DStore2,
    #[java_rs(opcode = 0x4A)]
    DStore3,
    #[java_rs(opcode = 0x67)]
    DSub,
    #[java_rs(opcode = 0x59)]
    Dup,
    #[java_rs(opcode = 0x5A)]
    DupX1,
    #[java_rs(opcode = 0x5B)]
    DupX2,
    #[java_rs(opcode = 0x5C)]
    Dup2,
    #[java_rs(opcode = 0x5D)]
    Dup2X1,
    #[java_rs(opcode = 0x5E)]
    Dup2X2,
    #[java_rs(opcode = 0x8D)]
    F2D,
    #[java_rs(opcode = 0x8B)]
    F2I,
    #[java_rs(opcode = 0x8C)]
    F2L,
    #[java_rs(opcode = 0x62)]
    FAdd,
    #[java_rs(opcode = 0x30)]
    FALoad,
    #[java_rs(opcode = 0x51)]
    FAStore,
    #[java_rs(opcode = 0x96)]
    FCmpPG,
    #[java_rs(opcode = 0x95)]
    FCmpPL,
    #[java_rs(opcode = 0xB)]
    FConst0,
    #[java_rs(opcode = 0xC)]
    FConst1,
    #[java_rs(opcode = 0xD)]
    FConst2,
    #[java_rs(opcode = 0x6E)]
    FDiv,
    #[java_rs(opcode = 0x17)]
    FLoad { index: SizedIndex },
    #[java_rs(opcode = 0x22)]
    FLoad0,
    #[java_rs(opcode = 0x23)]
    FLoad1,
    #[java_rs(opcode = 0x24)]
    FLoad2,
    #[java_rs(opcode = 0x25)]
    FLoad3,
    #[java_rs(opcode = 0x6A)]
    FMul,
    #[java_rs(opcode = 0x76)]
    FNeg,
    #[java_rs(opcode = 0x72)]
    FRem,
    #[java_rs(opcode = 0xAE)]
    FReturn,
    #[java_rs(opcode = 0x38)]
    FStore { index: SizedIndex },
    #[java_rs(opcode = 0x43)]
    FStore0,
    #[java_rs(opcode = 0x44)]
    FStore1,
    #[java_rs(opcode = 0x45)]
    FStore2,
    #[java_rs(opcode = 0x46)]
    FStore3,
    #[java_rs(opcode = 0x66)]
    FSub,
    #[java_rs(opcode = 0xB4)]
    GetField { index: WideIndex },
    #[java_rs(opcode = 0xB2)]
    GetStatic { index: WideIndex },
    #[java_rs(opcode = 0xA7)]
    Goto { offset: u16 },
    #[java_rs(opcode = 0xC8)]
    GotoW { offset: u32 },
    #[java_rs(opcode = 0x91)]
    I2B,
    #[java_rs(opcode = 0x92)]
    I2C,
    #[java_rs(opcode = 0x87)]
    I2D,
    #[java_rs(opcode = 0x86)]
    I2F,
    #[java_rs(opcode = 0x85)]
    I2L,
    #[java_rs(opcode = 0x93)]
    I2S,
    #[java_rs(opcode = 0x60)]
    IAdd,
    #[java_rs(opcode = 0x2E)]
    IALoad,
    #[java_rs(opcode = 0x7E)]
    IAnd,
    #[java_rs(opcode = 0x4F)]
    IAStore,
    #[java_rs(opcode = 0x2)]
    IConstM1,
    #[java_rs(opcode = 0x3)]
    IConst0,
    #[java_rs(opcode = 0x4)]
    IConst1,
    #[java_rs(opcode = 0x5)]
    IConst2,
    #[java_rs(opcode = 0x6)]
    IConst3,
    #[java_rs(opcode = 0x7)]
    IConst4,
    #[java_rs(opcode = 0x8)]
    IConst5,
    #[java_rs(opcode = 0x6C)]
    IDiv,
    #[java_rs(opcode = 0xA5)]
    IfACmpEq { offset: u16 },
    #[java_rs(opcode = 0xA6)]
    IfACmpNe { offset: u16 },
    #[java_rs(opcode = 0x9F)]
    IfICmpEq { offset: u16 },
    #[java_rs(opcode = 0xA0)]
    IfICmpNe { offset: u16 },
    #[java_rs(opcode = 0xA1)]
    IfICmpLt { offset: u16 },
    #[java_rs(opcode = 0xA2)]
    IfICmpGe { offset: u16 },
    #[java_rs(opcode = 0xA3)]
    IfICmpGt { offset: u16 },
    #[java_rs(opcode = 0xA4)]
    IfICmpLe { offset: u16 },
    #[java_rs(opcode = 0x99)]
    IfEq { offset: u16 },
    #[java_rs(opcode = 0x9A)]
    IfNe { offset: u16 },
    #[java_rs(opcode = 0x9B)]
    IfLt { offset: u16 },
    #[java_rs(opcode = 0x9C)]
    IfGe { offset: u16 },
    #[java_rs(opcode = 0x9D)]
    IfGt { offset: u16 },
    #[java_rs(opcode = 0x9E)]
    IfLe { offset: u16 },
    #[java_rs(opcode = 0xC7)]
    IfNonNull { offset: u16 },
    #[java_rs(opcode = 0xC6)]
    IfNull { offset: u16 },
    #[java_rs(opcode = 0x84)]
    IInc { index: SizedIndex, value: SizedIndex },
    #[java_rs(opcode = 0x15)]
    ILoad { index: SizedIndex },
    #[java_rs(opcode = 0x1A)]
    ILoad0,
    #[java_rs(opcode = 0x1B)]
    ILoad1,
    #[java_rs(opcode = 0x1C)]
    ILoad2,
    #[java_rs(opcode = 0x1D)]
    ILoad3,
    #[java_rs(opcode = 0x68)]
    IMul,
    #[java_rs(opcode = 0x74)]
    INeg,
    #[java_rs(opcode = 0xC1)]
    InstanceOf { index: WideIndex },
    #[java_rs(opcode = 0xBA)]
    InvokeDynamic {
        index: WideIndex,
        _zero0: AlwaysZero,
        _zero1: AlwaysZero,
    },
    #[java_rs(opcode = 0xB9)]
    InvokeInterface {
        index: WideIndex,
        count: u8,
        _zero: AlwaysZero,
    },
    #[java_rs(opcode = 0xB7)]
    InvokeSpecial { index: WideIndex },
    #[java_rs(opcode = 0xB8)]
    InvokeStatic { index: WideIndex },
    #[java_rs(opcode = 0xB6)]
    InvokeVirtual { index: WideIndex },
    #[java_rs(opcode = 0x80)]
    IOr,
    #[java_rs(opcode = 0x70)]
    IRem,
    #[java_rs(opcode = 0xAC)]
    IReturn,
    #[java_rs(opcode = 0x78)]
    IShl,
    #[java_rs(opcode = 0x7A)]
    IShr,
    #[java_rs(opcode = 0x36)]
    IStore { index: SizedIndex },
    #[java_rs(opcode = 0x3B)]
    IStore0,
    #[java_rs(opcode = 0x3C)]
    IStore1,
    #[java_rs(opcode = 0x3D)]
    IStore2,
    #[java_rs(opcode = 0x3E)]
    IStore3,
    #[java_rs(opcode = 0x64)]
    ISub,
    #[java_rs(opcode = 0x7C)]
    IUShr,
    #[java_rs(opcode = 0x82)]
    IXor,
    #[java_rs(opcode = 0xA8)]
    JSR { offset: u16 },
    #[java_rs(opcode = 0xC9)]
    JSRW { offset: u32 },
    #[java_rs(opcode = 0x8A)]
    L2D,
    #[java_rs(opcode = 0x89)]
    L2F,
    #[java_rs(opcode = 0x88)]
    L2I,
    #[java_rs(opcode = 0x61)]
    LAdd,
    #[java_rs(opcode = 0x2F)]
    LALoad,
    #[java_rs(opcode = 0x7F)]
    LAnd,
    #[java_rs(opcode = 0x50)]
    LAStore,
    #[java_rs(opcode = 0x94)]
    LCmp,
    #[java_rs(opcode = 0x9)]
    LConst0,
    #[java_rs(opcode = 0xA)]
    LConst1,
    #[java_rs(opcode = 0x12)]
    LDC { index: SmallIndex },
    #[java_rs(opcode = 0x13)]
    LDCW { index: WideIndex },
    #[java_rs(opcode = 0x14)]
    LDC2W { index: WideIndex },
    #[java_rs(opcode = 0x6D)]
    LDiv,
    #[java_rs(opcode = 0x16)]
    LLoad { index: SizedIndex },
    #[java_rs(opcode = 0x1E)]
    LLoad0,
    #[java_rs(opcode = 0x1F)]
    LLoad1,
    #[java_rs(opcode = 0x20)]
    LLoad2,
    #[java_rs(opcode = 0x21)]
    LLoad3,
    #[java_rs(opcode = 0x69)]
    LMul,
    #[java_rs(opcode = 0x75)]
    LNeg,
    #[java_rs(opcode = 0xAB, io_implementation = LookupSwitchIO)]
    LookUpSwitch {
        default: i32,
        pairs: SizedVec<i32, MatchOffsetPair>,
    },
    #[java_rs(opcode = 0x81)]
    LOr,
    #[java_rs(opcode = 0x71)]
    LRem,
    #[java_rs(opcode = 0xAD)]
    LReturn,
    #[java_rs(opcode = 0x79)]
    LShl,
    #[java_rs(opcode = 0x7B)]
    LShr,
    #[java_rs(opcode = 0x37)]
    LStore { index: SizedIndex },
    #[java_rs(opcode = 0x3F)]
    LStore0,
    #[java_rs(opcode = 0x40)]
    LStore1,
    #[java_rs(opcode = 0x41)]
    LStore2,
    #[java_rs(opcode = 0x42)]
    LStore3,
    #[java_rs(opcode = 0x65)]
    LSub,
    #[java_rs(opcode = 0x7D)]
    LUShr,
    #[java_rs(opcode = 0x83)]
    LXor,
    #[java_rs(opcode = 0xC2)]
    MonitorEnter,
    #[java_rs(opcode = 0xC3)]
    MonitorExit,
    #[java_rs(opcode = 0xC5)]
    MultiANewArray { index: WideIndex, dimensions: u8 },
    #[java_rs(opcode = 0xBB)]
    New { index: WideIndex },
    #[java_rs(opcode = 0xBC)]
    NewArray { ty: ArrayType },
    #[java_rs(opcode = 0x0)]
    Nop,
    #[java_rs(opcode = 0x57)]
    Pop,
    #[java_rs(opcode = 0x58)]
    Pop2,
    #[java_rs(opcode = 0xB5)]
    PutField { index: WideIndex },
    #[java_rs(opcode = 0xB3)]
    PutStatic { index: WideIndex },
    #[java_rs(opcode = 0xA9)]
    Ret { index: SizedIndex },
    #[java_rs(opcode = 0xB1)]
    Return,
    #[java_rs(opcode = 0x35)]
    SALoad,
    #[java_rs(opcode = 0x56)]
    SAStore,
    #[java_rs(opcode = 0x11)]
    SIPush { value: i16 },
    #[java_rs(opcode = 0x5F)]
    Swap,
    #[java_rs(opcode = 0xAA, io_implementation = TableSwitchIO)]
    TableSwitch {
        default: i32,
        low: i32,
        high: i32,
        offsets: SizedVec<u32, i32>,
    },
    #[java_rs(opcode = 0xC4)]
    Wide,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, BufWriter};

    use crate::{AccessFlags, Constant, ConstantPoolIndex, Error, JavaClass, JavaVersion, MagicNumber, Method, SizedVec};
    use crate::attribute::{Attribute, Compatibility};
    use crate::helper::*;

    #[test]
    fn check_empty_current_code() -> Result<(), Error> {
        let (path, _guard) = init_tmp_dir("CheckEmptyCodeTest.class");

        let reference = JavaClass {
            magic: MagicNumber::Cafebabe,
            version: JavaVersion { major: 45, minor: 3 },
            constant_pool: vec![
                Constant::Class(ConstantPoolIndex(3)),
                Constant::Class(ConstantPoolIndex(4)),
                Constant::Utf8("CheckEmptyCodeTest".into()),
                Constant::Utf8("java/lang/Object".into()),
                Constant::Utf8("m1".into()),
                Constant::Utf8("()V".into()),
                Constant::Utf8("Code".into())
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
                    max_stack: Compatibility::Current(65534),
                    max_locals:Compatibility::Current(65534),
                    code: Compatibility::Current(SizedVec::new()),
                    exception_table: SizedVec::new(),
                    attributes: SizedVec::new()
                }].into()
            }].into(),
            attributes: SizedVec::new(),
        };

        reference.write(&mut BufWriter::new(File::create(&path)?))?;
        assert_eq!(reference, JavaClass::read(&mut BufReader::new(File::open(path)?))?);
        Ok(())
    }

    #[test]
    fn check_empty_pre_java1_code() -> Result<(), Error> {
        let (path, _guard) = init_tmp_dir("CheckEmptyCodeTest.class");

        let reference = JavaClass {
            magic: MagicNumber::Cafebabe,
            version: JavaVersion { major: 45, minor: 2 },
            constant_pool: vec![
                Constant::Unsupported(Box::new(Constant::Class(ConstantPoolIndex(3)))),
                Constant::Unsupported(Box::new(Constant::Class(ConstantPoolIndex(4)))),
                Constant::Unsupported(Box::new(Constant::Utf8("CheckEmptyCodeTest".into()))),
                Constant::Unsupported(Box::new(Constant::Utf8("java/lang/Object".into()))),
                Constant::Unsupported(Box::new(Constant::Utf8("m1".into()))),
                Constant::Unsupported(Box::new(Constant::Utf8("()V".into()))),
                Constant::Unsupported(Box::new(Constant::Utf8("Code".into())))
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
                attributes: vec![Attribute::Unsupported(Box::new(Attribute::Code {
                    name: ConstantPoolIndex(7),
                    max_stack: Compatibility::PreJava1(0),
                    max_locals: Compatibility::PreJava1(0),
                    code: Compatibility::PreJava1(SizedVec::new()),
                    exception_table: SizedVec::new(),
                    attributes: SizedVec::new(),
                }))].into()
            }].into(),
            attributes: SizedVec::new(),
        };

        reference.write(&mut BufWriter::new(File::create(&path)?))?;
        assert_eq!(reference, JavaClass::read(&mut BufReader::new(File::open(path)?))?);
        Ok(())
    }
}
