use std::io::{self, Read, Write};
use std::str::{self, Utf8Error};
use std::{char, env, fmt, fs};

const CHARMAP_EMPTY: &'static [char] = &[];

const CHARMAP_SR_CYR: &'static [char] = &[
    'А', 'Б', 'В', 'Г', 'Д', 'Ђ', 'Е', 'Ж', 'З', 'И', 'Ј', 'К', 'Л', 'Љ', 'М', 'Н', 'Њ', 'О', 'П',
    'Р', 'С', 'Т', 'Ћ', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Џ', 'Ш', 'a', 'б', 'в', 'г', 'д', 'ђ', 'e', 'ж',
    'з', 'и', 'j', 'к', 'л', 'љ', 'м', 'н', 'њ', 'o', 'п', 'р', 'с', 'т', 'ћ', 'у', 'ф', 'х', 'ц',
    'ч', 'џ', 'ш',
];

const CHARMAP_SR_LAT: &'static [char] = &[
    'A', 'B', 'V', 'G', 'D', 'Đ', 'E', 'Ž', 'Z', 'I', 'J', 'K', 'L', 'ǈ', 'M', 'N', 'ǋ', 'O', 'P',
    'R', 'S', 'T', 'Ć', 'U', 'F', 'H', 'C', 'Č', 'ǅ', 'Š', 'a', 'b', 'v', 'g', 'd', 'đ', 'e', 'ž',
    'z', 'i', 'j', 'k', 'l', 'ǉ', 'm', 'n', 'ǌ', 'o', 'p', 'r', 's', 't', 'ć', 'u', 'f', 'h', 'c',
    'č', 'ǆ', 'š',
];

const DIGEST_BUFFER: usize = 8;
const IO_BUFFER: usize = 1024;

struct Processor<'s> {
    chars: [u8; DIGEST_BUFFER],
    count: usize,
    table_from: &'s [char],
    table_into: &'s [char],
}

enum Error {
    BufferOverflow,
    MapSizeMismatch,
    IoError(io::Error),
    UTFError(str::Utf8Error),
    ArgumentMissing,
    ArgumentUnknown,
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
            Self::BufferOverflow => writeln!(f, "IO Buffer Overflow"),
            Self::MapSizeMismatch => writeln!(f, "Character map size mismatch"),
            Self::IoError(e) => writeln!(f, "IO error - {}", e),
            Self::UTFError(e) => writeln!(f, "UTF-8 error - {}", e),
            Self::ArgumentMissing => writeln!(f, "Missing argument"),
            Self::ArgumentUnknown => writeln!(f, "Unknown argument"),
        }
    }
}

impl<'s> Default for Processor<'s> {
    #[inline]
    fn default() -> Self {
        Self {
            chars: [0; DIGEST_BUFFER],
            count: 0,
            table_from: CHARMAP_EMPTY,
            table_into: CHARMAP_EMPTY,
        }
    }
}

#[allow(dead_code)]
impl<'s> Processor<'s> {
    fn new() -> Self {
        Self::default()
    }

    fn from(table_from: &'s [char], table_into: &'s [char]) -> Result<Self, Error> {
        let mut new = Self::default();
        if table_from.len() == table_into.len() {
            new.table_from = table_from;
            new.table_into = table_into;
            Ok(new)
        } else {
            Err(Error::MapSizeMismatch)
        }
    }

    fn tables(&mut self, table_from: &'s [char], table_into: &'s [char]) -> Result<(), Error> {
        if table_from.len() == table_into.len() {
            self.table_from = table_from;
            self.table_into = table_into;
            Ok(())
        } else {
            Err(Error::MapSizeMismatch)
        }
    }

    #[inline]
    fn consume(&mut self, c: &u8) -> Result<(), Error> {
        if self.count == DIGEST_BUFFER {
            Err(Error::BufferOverflow)
        } else {
            self.chars[self.count] = *c;
            self.count += 1;
            Ok(())
        }
    }

    fn digest(&mut self, output: &mut String) -> Result<(), Utf8Error> {
        match str::from_utf8(&self.chars[..self.count]) {
            Ok(str) => {
                self.count = 0;
                for c in str.chars() {
                    output.push(self.transliterate(c));
                }
                Ok(())
            }
            Err(e) => {
                if e.valid_up_to() > 0 {
                    self.count = e.valid_up_to();
                    self.digest(output)?;
                    self.count = DIGEST_BUFFER - e.valid_up_to();
                    for i in 0..self.count {
                        self.chars[i] = self.chars[i + e.valid_up_to()]
                    }
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }

    #[inline]
    fn transliterate(&self, input: char) -> char {
        if let Some(i) = self.table_from.iter().position(|e| *e == input) {
            return self.table_into[i];
        } else {
            input
        }
    }

    fn process(&mut self, input: &mut dyn Read, output: &mut dyn Write) -> Result<(), Error> {
        let mut buffer_in = [0u8; IO_BUFFER];
        let mut buffer_out = String::with_capacity(IO_BUFFER);

        loop {
            let n = input.read(&mut buffer_in)?;
            if n == 0 {
                self.digest(&mut buffer_out)?;
                output.write(&buffer_out.as_bytes())?;
                return Ok(());
            }
            for i in 0..n {
                if let Err(_) = self.consume(&buffer_in[i]) {
                    self.digest(&mut buffer_out)?;
                    output.write(&buffer_out.as_bytes())?;
                    buffer_out.clear();
                    if let Err(e) = self.consume(&buffer_in[i]) {
                        return Err(e);
                    }
                }
            }
        }
    }
}

fn version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"),);
}

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

fn main() -> Result<(), Error> {
    let mut proc = Processor::new();

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
            "-l" | "--latin" => proc.tables(CHARMAP_SR_CYR, CHARMAP_SR_LAT)?,
            "-c" | "--cyrillic" => proc.tables(CHARMAP_SR_LAT, CHARMAP_SR_CYR)?,
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
        proc.process(input, output)
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_no_changes_empty() -> Result<(), Error> {
        let mut proc = Processor::new();
        let input = String::from("ã$âÕÝ=¼ÜyÌÈo-P©6ë1M»¡u9-ú{K=/NMd");
        let mut output = Vec::new();
        proc.process(&mut input.as_bytes(), &mut output)?;
        assert_eq!(input, String::from_utf8_lossy(&output));
        Ok(())
    }

    #[test]
    fn process_no_changes_lat() -> Result<(), Error> {
        let mut proc = Processor::new();
        proc.tables(CHARMAP_SR_LAT, CHARMAP_SR_LAT)?;
        let input = String::from("ã$âÕÝ=¼ÜyÌÈo-P©6ë1M»¡u9-ú{K=/NMd");
        let mut output = Vec::new();
        proc.process(&mut input.as_bytes(), &mut output)?;
        assert_eq!(input, String::from_utf8_lossy(&output));
        Ok(())
    }

    #[test]
    fn process_lat_to_cyr() -> Result<(), Error> {
        let mut proc = Processor::new();
        proc.tables(CHARMAP_SR_LAT, CHARMAP_SR_CYR)?;
        let input = String::from("ABVGDĐEŽZIJKLǈMNǋOPRSTĆUFHCČǅŠabvgdđežzijklǉmnǌoprstćufhcčǆš");
        let mut output = Vec::new();
        proc.process(&mut input.as_bytes(), &mut output)?;
        assert_eq!(
            String::from_utf8_lossy(&output),
            "АБВГДЂЕЖЗИЈКЛЉМНЊОПРСТЋУФХЦЧЏШaбвгдђeжзиjклљмнњoпрстћуфхцчџш"
        );
        Ok(())
    }

    #[test]
    fn process_mixed_to_cyr() -> Result<(), Error> {
        let mut proc = Processor::new();
        proc.tables(CHARMAP_SR_LAT, CHARMAP_SR_CYR)?;
        let input = String::from("ABVGDЂЕЖЗИЈКЛЉМNǋOPRСТЋУФHCČǅŠaбвгдђeжзиjклљмnǌoprstćufhcčǆš");
        let mut output = Vec::new();
        proc.process(&mut input.as_bytes(), &mut output)?;
        assert_eq!(
            String::from_utf8_lossy(&output),
            "АБВГДЂЕЖЗИЈКЛЉМНЊОПРСТЋУФХЦЧЏШaбвгдђeжзиjклљмнњoпрстћуфхцчџш"
        );
        Ok(())
    }
}
