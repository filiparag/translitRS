use std::str::FromStr;
use std::{env, fmt, path};

mod process;
mod transliterate;

use process::{PandocProcessor, PlaintextProcessor, Processor};
use transliterate::Transliterator;

fn version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"),);
}

fn help() {
    version();
    println!();
    println!("Usage:");
    println!("  -v, --version           show version and quit");
    println!("  -h, --help              show usage help and quit");
    println!("  -i, --input <path>      read input from file");
    println!("  -o, --output <path>     write output to file");
    println!("  -f, --from <charset>    convert from character set");
    println!("  -t, --into <charset>    convert to character set");
    println!("  -s, --skip-foreign      skip words with foreign characters");
    println!("  -p, --pandoc-filter     run in Pandoc JSON filter mode");
    println!();
    println!("Character sets:");
    println!("  latin,    lat,  l       Serbian Latin");
    println!("  latin8,   lat8, l8      Serbian Latin (Unicode)");
    println!("  cyrillic, cyr,  c       Serbian Cyrillic");
}

pub enum Error {
    ArgumentMissing,
    ArgumentUnknown,
    ArgumentInvalid,
    Runtime(process::Error),
}

impl From<process::Error> for Error {
    fn from(error: process::Error) -> Self {
        Self::Runtime(error)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArgumentMissing => writeln!(f, "Missing an argument"),
            Self::ArgumentUnknown => writeln!(f, "Uknown argument"),
            Self::ArgumentInvalid => writeln!(f, "Invalid argument"),
            Self::Runtime(e) => writeln!(f, "Runtime error - {}", e),
        }
    }
}

impl std::str::FromStr for transliterate::Charset {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "latin" | "lat" | "l" => Ok(transliterate::Charset::Latin),
            "latin8" | "lat8" | "l8" => Ok(transliterate::Charset::LatinUnicode),
            "cyrillic" | "cyr" | "c" => Ok(transliterate::Charset::Cyrillic),
            _ => Err(Error::ArgumentInvalid),
        }
    }
}

fn main() -> Result<(), Error> {
    let mut input: Option<path::PathBuf> = None;
    let mut output: Option<path::PathBuf> = None;

    let mut charset_from = transliterate::Charset::Latin;
    let mut charset_into = transliterate::Charset::Cyrillic;
    let mut skip_foreing = false;
    let mut pandoc_mode = false;

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
                    input = Some(path::PathBuf::from(path));
                } else {
                    return Err(Error::ArgumentMissing);
                }
            }
            "-s" | "--skip-foreign" => {
                skip_foreing = true;
            }
            "-p" | "--pandoc-filter" => {
                pandoc_mode = true;
            }
            "-o" | "--output" => {
                if let Some(path) = arguments.next() {
                    output = Some(path::PathBuf::from(path));
                } else {
                    return Err(Error::ArgumentMissing);
                }
            }
            _ => return Err(Error::ArgumentUnknown),
        }
    }

    let t = Transliterator::new(charset_from, charset_into, skip_foreing);
    let mut p: Box<dyn Processor> = match pandoc_mode {
        true => Box::new(PandocProcessor::new(t)),
        false => Box::new(PlaintextProcessor::new(input, output, t)?),
    };
    p.run()?;

    Ok(())
}
