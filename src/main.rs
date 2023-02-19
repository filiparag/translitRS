use std::str::FromStr;
use std::{env, error, fmt, path};

mod process;
mod transliterate;

#[cfg(feature = "pandoc")]
use process::PandocProcessor;
use process::{FileProcessor, PlaintextProcessor};
use transliterate::{Charset, Transliterator};

fn version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"),);
}

fn help() {
    println!("{}", env!("CARGO_PKG_DESCRIPTION"),);
    println!();
    println!("USAGE:");
    println!("  {} [OPTIONS]", env!("CARGO_PKG_NAME"));
    println!("  pandoc --filter {} [...]", env!("CARGO_PKG_NAME"));
    println!();
    println!("OPTIONS:");
    println!("  -i, --input <path>      read input from file");
    println!("                          default: stdin");
    println!("  -o, --output <path>     write output to file");
    println!("                          default: stdout");
    println!("  -f, --from <charset>    convert from character set");
    println!("                          default: latin");
    println!("  -t, --into <charset>    convert to character set");
    println!("                          default: cyrillic");
    println!("  -d, --skip-digraph      do not check for digraph exceptions");
    println!("  -u, --force-foreign     process words with foreign and mixed characters");
    println!("  -l, --force-links       process hyperlinks, email addresses and units");
    #[cfg(feature = "pandoc")]
    println!("  -p, --pandoc-filter     run in Pandoc JSON pipe filter mode");
    println!("  -v, --version           show version and quit");
    println!("  -h, --help              show usage help and quit");
    println!();
    println!("Character sets:");
    println!("  latin,    lat,  l       Serbian Latin");
    println!("  latin8,   lat8, l8      Serbian Latin (Unicode)");
    println!("  cyrillic, cyr,  c       Serbian Cyrillic");
    println!();
    println!("Pandoc filter environment variables:");
    println!("  CHARS_FROM=<charset>");
    println!("  CHARS_INTO=<charset>");
    println!("  SKIP_DIGRAPH");
    println!("  FORCE_FOREIGN");
    println!("  FORCE_LINKS");
}

#[derive(Debug)]
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArgumentMissing => writeln!(f, "Missing an argument"),
            Self::ArgumentUnknown => writeln!(f, "Uknown argument"),
            Self::ArgumentInvalid => writeln!(f, "Invalid argument"),
            Self::Runtime(e) => writeln!(f, "Runtime error - {}", e),
        }
    }
}

impl error::Error for Error {}

impl std::str::FromStr for Charset {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "latin" | "lat" | "l" => Ok(Charset::Latin),
            "latin8" | "lat8" | "l8" => Ok(Charset::LatinUnicode),
            "cyrillic" | "cyr" | "c" => Ok(Charset::Cyrillic),
            _ => Err(Error::ArgumentInvalid),
        }
    }
}

struct Arguments {
    transliterator: Transliterator,
    input: Option<path::PathBuf>,
    output: Option<path::PathBuf>,
    #[cfg(feature = "pandoc")]
    pandoc_mode: bool,
}

fn parse_args() -> Result<Arguments, Error> {
    let mut input: Option<path::PathBuf> = None;
    let mut output: Option<path::PathBuf> = None;

    let mut charset_from = Charset::Latin;
    let mut charset_into = Charset::Cyrillic;
    let mut skip_digraph = false;
    let mut force_foreign = false;
    let mut force_links = false;
    #[cfg(feature = "pandoc")]
    let mut pandoc_mode = false;

    let mut arguments = env::args();
    let _ = arguments.next();

    while let Some(arg) = arguments.next() {
        match &*arg {
            "-v" | "--version" => {
                version();
                std::process::exit(0);
            }
            "-h" | "--help" => {
                help();
                std::process::exit(0);
            }
            "-t" | "--into" => {
                if let Some(value) = arguments.next() {
                    charset_into = Charset::from_str(&value)?
                } else {
                    return Err(Error::ArgumentMissing);
                }
            }
            "-f" | "--from" => {
                if let Some(value) = arguments.next() {
                    charset_from = Charset::from_str(&value)?
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
            "-d" | "--skip-digraph" => {
                skip_digraph = true;
            }
            "-u" | "--force-foreign" => {
                force_foreign = true;
            }
            "-l" | "--force-links" => {
                force_links = true;
            }
            #[cfg(feature = "pandoc")]
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
    Ok(Arguments {
        transliterator: Transliterator::new(
            charset_from,
            charset_into,
            skip_digraph,
            force_foreign,
            force_links,
        ),
        input,
        output,
        #[cfg(feature = "pandoc")]
        pandoc_mode,
    })
}

fn regular_mode() -> Result<Box<dyn FileProcessor>, Error> {
    let args = parse_args()?;
    #[cfg(not(feature = "pandoc"))]
    return Ok(Box::new(PlaintextProcessor::new(args.input, args.output, args.transliterator)?));
    #[cfg(feature = "pandoc")]
    return Ok(match args.pandoc_mode {
        true => Box::new(PandocProcessor::new(args.transliterator)),
        false => Box::new(PlaintextProcessor::new(args.input, args.output, args.transliterator)?),
    });
}

#[cfg(feature = "pandoc")]
fn pandoc_mode() -> Result<Box<dyn FileProcessor>, Error> {
    fn parse_env_charset(key: &str, default: Charset) -> Result<Charset, Error> {
        if let Ok(value) = env::var(key) {
            if !value.is_empty() {
                return Charset::from_str(&value);
            }
        }
        Ok(default)
    }
    fn parse_env_bool(key: &str, default: bool) -> Result<bool, Error> {
        if let Ok(value) = env::var(key) {
            return Ok(!matches!(value.as_str(), "0" | "false" | "no"));
        }
        Ok(default)
    }
    let transliterator = Transliterator::new(
        parse_env_charset("CHARS_FROM", Charset::Latin)?,
        parse_env_charset("CHARS_INTO", Charset::Cyrillic)?,
        parse_env_bool("SKIP_DIGRAPH", false)?,
        parse_env_bool("FORCE_FOREIGN", false)?,
        parse_env_bool("FORCE_LINKS", false)?,
    );
    Ok(Box::new(PandocProcessor::new(transliterator)))
}

fn main() -> Result<(), Error> {
    #[cfg(not(feature = "pandoc"))]
    regular_mode()?.run()?;
    #[cfg(feature = "pandoc")]
    {
        // Detect if called by Pandoc as a JSON filter
        let mut proc: Box<dyn FileProcessor> = if let Ok(value) = env::var("PANDOC_VERSION") {
            if !value.is_empty() {
                pandoc_mode()
            } else {
                regular_mode()
            }
        } else {
            regular_mode()
        }?;
        proc.run()?;
    }
    Ok(())
}
