use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An IO error occurred")]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    UtfConversionError(#[from] UtfConversionError),

    #[error("Unknown Java tag type {0:X}")]
    UnknownTag(u8),

    #[error("Unknown Java target type {0:X}")]
    UnknownTargetType(u8),

    #[error("Unknown Java verification type {0:X}")]
    UnknownVerificationType(u8),

    #[error("Unknown Java stack map frame type {0:X}")]
    UnknownStackMapFrameType(u8),

    #[error("Invalid element value tag {0}")]
    InvalidElementValueTag(char),

    #[error("Invalid Java access flags {0:X}")]
    InvalidAccessFlags(u16),

    #[error("Expected value {} to {}, but got {found}", expected.0, expected.1)]
    UnexpectedOpCodeValue { expected: (u8, u8), found: u8 },
}

#[derive(Debug, Error)]
pub enum UtfConversionError {
    #[error("Preliminary data end")]
    UnexpectedEndOfData,

    #[error("Unexpected continuation byte 0x{0:X}")]
    UnexpectedContinuation(u8),

    #[error("CESU8 String contained a null byte")]
    NullByteFound,

    #[error("Invalid Java UTF8: {0:?}")]
    InvalidJavaUtf8(Vec<u8>),
}
