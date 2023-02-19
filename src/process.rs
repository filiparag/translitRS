use std::{error, fmt, io};

use crate::transliterate;

#[cfg(feature = "pandoc")]
mod pandoc;
mod plaintext;

#[cfg(feature = "pandoc")]
pub use self::pandoc::PandocProcessor;
pub use plaintext::PlaintextProcessor;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Processing(transliterate::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => writeln!(f, "IO error - {}", e),
            Self::Processing(e) => writeln!(f, "Processing error - {}", e),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<transliterate::Error> for Error {
    fn from(error: transliterate::Error) -> Self {
        Self::Processing(error)
    }
}

pub trait FileProcessor {
    fn run(&mut self) -> Result<(), Error>;
}
