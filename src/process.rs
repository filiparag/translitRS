use std::{fmt, io};

use crate::transliterate;

mod pandoc;
mod plaintext;

pub use self::pandoc::PandocProcessor;
pub use plaintext::PlaintextProcessor;

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

pub trait Processor {
    fn run(&mut self) -> Result<(), Error>;
}
