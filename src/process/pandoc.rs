use std::io::{self, Read, Write};

use pandoc_ast::{self, Inline, MutVisitor};

use super::{Error, Processor};
use crate::transliterate::Transliterator;

pub struct PandocProcessor {
    input: io::Stdin,
    output: io::Stdout,
    processor: Transliterator,
}

impl MutVisitor for PandocProcessor {
    fn visit_inline(&mut self, inline: &mut Inline) {
        if let Inline::Str(ref mut s) = *inline {
            if let Ok(result) = self.processor.process(&s) {
                *s = result;
            }
        }
        self.walk_inline(inline);
    }
}

impl PandocProcessor {
    pub fn new(processor: Transliterator) -> Self {
        Self {
            input: io::stdin(),
            output: io::stdout(),
            processor,
        }
    }
}

impl Processor for PandocProcessor {
    fn run(&mut self) -> Result<(), Error> {
        let mut input_string = String::new();
        self.input.read_to_string(&mut input_string)?;
        let output_string = pandoc_ast::filter(input_string, |mut pandoc| {
            self.walk_pandoc(&mut pandoc);
            pandoc
        });
        self.output.write_all(output_string.as_bytes())?;
        Ok(())
    }
}
