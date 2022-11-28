use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse class file: {0}")]
    ParserError(#[from] std::io::Error),

    #[error("Failed to lift {0} because of {1}")]
    LifterError(String, String),
}
