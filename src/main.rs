use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Read, Write};

mod charmaps;
mod processor;
use processor::{Direction, StreamProcessor};

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
    println!("  {} \t\t{}", "-l, --latin", "convert to Latin");
    println!("  {} \t{}", "-c, --cyrillic", "convert co Cyrillic");
}

enum Error {
    ArgumentMissing,
    ArgumentUnknown,
    IoError(io::Error),
    ProcessingError(processor::Error),
}

impl From<processor::Error> for Error {
    fn from(error: processor::Error) -> Self {
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
            Self::IoError(e) => writeln!(f, "IO error - {}", e),
            Self::ProcessingError(e) => writeln!(f, "Processing error - {:?}", e),
        }
    }
}

fn main() -> Result<(), Error> {
    let mut proc: StreamProcessor = StreamProcessor::new(Direction::LatToCyr);

    let mut input: Option<&mut dyn Read> = None;
    let mut output: Option<&mut dyn Write> = None;

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
            "-l" | "--latin" => proc = StreamProcessor::new(Direction::CyrToLat),
            "-c" | "--cyrillic" => proc = StreamProcessor::new(Direction::LatToCyr),
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

    if let (Some(input), Some(output)) = (input, output) {
        proc.process(input, output)?;
    } else {
        unreachable!()
    }
    Ok(())
}
