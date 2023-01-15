use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use super::{Error, FileProcessor};
use crate::transliterate::Transliterator;

pub struct PlaintextProcessor {
    input: Box<dyn Read>,
    output: Box<dyn Write>,
    processor: Transliterator,
}

impl PlaintextProcessor {
    pub fn new(
        input: Option<PathBuf>,
        output: Option<PathBuf>,
        processor: Transliterator,
    ) -> Result<Self, Error> {
        Ok(Self {
            input: if let Some(p) = input {
                Box::from(File::open(p)?)
            } else {
                Box::from(io::stdin())
            },
            output: if let Some(p) = output {
                // TODO: Open file later for in-place edits
                Box::from(File::create(p)?)
            } else {
                Box::from(io::stdout())
            },
            processor,
        })
    }
}

impl FileProcessor for PlaintextProcessor {
    fn run(&mut self) -> Result<(), Error> {
        let mut input_string = String::new();
        self.input.read_to_string(&mut input_string)?;
        let output_string = self.processor.process(input_string)?;
        self.output.write_all(output_string.as_bytes())?;
        Ok(())
    }
}
