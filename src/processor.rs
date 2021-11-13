use std::fmt::{self};
use std::io::{self, Read, Write};
use std::str::{self, Utf8Error};
use subslice::bmh;

use super::charmaps;

const CONSUMED_BUFFER: usize = 8;
const DIGESTED_BUFFER: usize = 32;
const LASTWORD_BUFFER: usize = 64;

#[allow(dead_code)]
pub struct StreamProcessor {
    buffer_consumed: [u8; CONSUMED_BUFFER],
    buffer_digested: [char; DIGESTED_BUFFER],
    buffer_lastword: [char; LASTWORD_BUFFER],
    consumed: usize,
    digested: usize,
    lastword: usize,
    direction: Direction,
}

#[allow(dead_code)]
pub enum Error {
    EmptyDigest,
    BufferOverflow,
    IoError(io::Error),
    UTFError(str::Utf8Error),
}

#[allow(dead_code)]
pub enum Direction {
    LatToCyr,
    CyrToLat,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(error: str::Utf8Error) -> Self {
        Self::UTFError(error)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDigest => writeln!(f, "Digest is empty"),
            Self::BufferOverflow => writeln!(f, "IO Buffer Overflow"),
            Self::IoError(e) => writeln!(f, "IO error - {}", e),
            Self::UTFError(e) => writeln!(f, "UTF-8 error - {}", e),
        }
    }
}

#[allow(dead_code)]
impl StreamProcessor {
    pub fn new(direction: Direction) -> Self {
        Self {
            buffer_consumed: [0u8; CONSUMED_BUFFER],
            buffer_digested: ['\0'; DIGESTED_BUFFER],
            buffer_lastword: ['\0'; LASTWORD_BUFFER],
            consumed: 0,
            digested: 0,
            lastword: 0,
            direction: direction,
        }
    }

}
}
