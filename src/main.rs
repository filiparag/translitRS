use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::str::FromStr;

use transliterate::Transliterate;

mod transliterate;

#[allow(dead_code)]
fn version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"),);
}

#[allow(dead_code)]
fn help() {
    version();
    println!();
    println!("  {} \t{}", "-v, --version", "show version and quit");
    println!("  {} \t\t{}", "-h, --help", "show usage help and quit");
    println!("  {} \t\t{}", "-i, --input", "read input from file");
    println!("  {} \t\t{}", "-o, --output", "write output to file");
    println!("  {} \t\t{}", "-f, --from", "convert from character set");
    println!("  {} \t{}", "-t, --into", "convert to character set");
}

pub enum Error {
    ArgumentMissing,
    ArgumentUnknown,
    ArgumentInvalid,
    IoError(io::Error),
    ProcessingError(transliterate::Error),
}

impl From<transliterate::Error> for Error {
    fn from(error: transliterate::Error) -> Self {
        Self::ProcessingError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArgumentMissing => writeln!(f, "Missing an argument"),
            Self::ArgumentUnknown => writeln!(f, "Uknown argument"),
            Self::ArgumentInvalid => writeln!(f, "Invalid argument"),
            Self::IoError(e) => writeln!(f, "IO error - {}", e),
            Self::ProcessingError(e) => writeln!(f, "Processing error - {:?}", e),
        }
    }
}

impl std::str::FromStr for transliterate::Charset {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "latin" | "lat" | "l" => Ok(transliterate::Charset::Latin),
            "latin8" | "lat8" | "l8" => Ok(transliterate::Charset::LatinUnicode),
            "latin-unicode" | "lat-u" | "lu" => Ok(transliterate::Charset::LatinUnicode),
            "cyrillic" | "cyr" | "c" => Ok(transliterate::Charset::Cyrillic),
            _ => Err(Error::ArgumentInvalid),
        }
    }
}

fn main() -> Result<(), Error> {
    let mut input: Option<&mut dyn Read> = None;
    let mut output: Option<&mut dyn Write> = None;

    let mut charset_from = transliterate::Charset::Latin;
    let mut charset_into = transliterate::Charset::Cyrillic;

    let mut arguments = env::args();
    let _ = arguments.next();

    while let Some(arg) = arguments.next() {
        match &*arg {
            "-v" | "--version" => {
                version();
                return Ok(());
            }
            "-h" | "--help" => {
                help();
                return Ok(());
            }
            "-t" | "--into" => {
                if let Some(value) = arguments.next() {
                    charset_into = transliterate::Charset::from_str(&value)?
                } else {
                    return Err(Error::ArgumentMissing);
                }
            }
            "-f" | "--from" => {
                if let Some(value) = arguments.next() {
                    charset_from = transliterate::Charset::from_str(&value)?
                } else {
                    return Err(Error::ArgumentMissing);
                }
            }
            "-i" | "--input" => {
                if let Some(path) = arguments.next() {
                    input = Some(Box::leak(Box::from(fs::File::open(path)?)));
                } else {
                    return Err(Error::ArgumentMissing);
                }
            }
            "-o" | "--output" => {
                if let Some(path) = arguments.next() {
                    output = Some(Box::leak(Box::from(fs::File::create(path)?)));
                } else {
                    return Err(Error::ArgumentMissing);
                }
            }
            _ => return Err(Error::ArgumentUnknown),
        }
    }

    if let None = input {
        input = Some(Box::leak(Box::from(io::stdin())));
    }

    if let None = output {
        output = Some(Box::leak(Box::from(io::stdout())));
    }

    let proc = Transliterate::new(charset_from, charset_into);

    if let (Some(input), Some(output)) = (input, output) {
        let mut input_string = String::new();
        input.read_to_string(&mut input_string)?;
        let output_string = proc.process(input_string)?;
        output.write(output_string.as_bytes())?;
    } else {
        unreachable!()
    }
    Ok(())
}
