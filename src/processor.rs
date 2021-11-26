use std::fmt;
use std::io::{self, Read, Write};
use std::str;
use subslice::bmh;

use super::charmaps;

const CONSUMED_BUFFER: usize = 8;
const DIGESTED_BUFFER: usize = 32;
const LASTWORD_BUFFER: usize = 128;

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

    fn consume<'a>(&mut self, raw: &'a [u8]) -> Result<usize, usize> {
        let (chunk, overflow) = if self.consumed + raw.len() >= CONSUMED_BUFFER {
            (CONSUMED_BUFFER - self.consumed, true)
        } else {
            (raw.len(), false)
        };
        self.buffer_consumed[self.consumed..self.consumed + chunk].copy_from_slice(&raw[..chunk]);
        self.consumed += chunk;
        if overflow {
            Err(chunk)
        } else {
            Ok(chunk)
        }
    }

    fn digest(&mut self) -> Result<(), Error> {
        match str::from_utf8(&self.buffer_consumed[..self.consumed]) {
            Ok(str) => {
                let chars = str.chars();
                let count = chars.count();
                if self.digested + count >= DIGESTED_BUFFER {
                    return Err(Error::BufferOverflow);
                }
                for (i, c) in str.chars().enumerate() {
                    self.buffer_digested[self.digested + i] = c;
                }
                self.digested += count;
                self.consumed = 0;
                Ok(())
            }
            Err(e) => {
                if e.valid_up_to() > 0 {
                    let remaining = self.consumed - e.valid_up_to();
                    self.consumed = e.valid_up_to();
                    self.digest()?;
                    self.consumed = remaining;
                    for i in 0..remaining {
                        self.buffer_consumed[i] = self.buffer_consumed[i + e.valid_up_to()];
                    }
                    Ok(())
                } else {
                    Err(Error::UTFError(e))
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    const EXAMPLES: &'static [(&str, &str)] = &[
        ("", ""),
        ("1234567890", "1234567890"),
        (
            "ABVGDĐEŽZIJKLLjMNNjOPRSTĆUFHCČDžŠabvgdđežzijklljmnnjoprstćufhcčdžš",
            "АБВГДЂЕЖЗИЈКЛЉМНЊОПРСТЋУФХЦЧЏШабвгдђежзијклљмнњопрстћуфхцчџш",
        ),
        (
            "Stala mala Mara na kraj stara hana sama.",
            "Стала мала Мара на крај стара хана сама.",
        ),
        (
            "Nevesele snene žene plele teške mreže",
            "Невеселе снене жене плеле тешке мреже",
        ),
        (
            "Javorov jaram, javorova ralica, ralo drvo javorovo.",
            "Јаворов јарам, јаворова ралица, рало дрво јаворово.",
        ),
        (
            "Jesi li to ti to tu? Jesi li to tu ti? Jesi li to ti tu? Jesi li tu ti to?",
            "Јеси ли то ти то ту? Јеси ли то ту ти? Јеси ли то ти ту? Јеси ли ту ти то?",
        ),
        (
            "Adjektivisati|ZABLUDJE|odžvać|PredŽivot|kenjon|konjug|TANJug",
            "Адјективисати|ЗАБЛУДЈЕ|оджваћ|ПредЖивот|кенјон|конјуг|ТАНЈуг",
        ),
        (
            "ABVGDĐÐDJDjEZŽŽIJKLLJǇLjǈMNNJǊNjǋOPRSTĆĆUFHCČČDŽǄDŽDžǅDžŠŠ",
            "АБВГДЂЂЂЂЕЗЖЖИЈКЛЉЉЉЉМНЊЊЊЊОПРСТЋЋУФХЦЧЧЏЏЏЏЏЏШШ",
        ),
        (
            "aæbvgdđdjezžžiĳjklljǉmnnjǌoœprsﬆtććufﬁﬂhcččdžǆdžšš",
            "ааебвгдђђезжжиијјклљљмнњњооепрссттћћуффифлхцччџџџшш",
        ),
        (
            "ABVGDĐÐDJDjezžžiĳjklljǉMNNJǊNjǋOPRsﬆtććufﬁﬂhcččdžǄDŽDžǅDžŠŠ",
            "АБВГДЂЂЂЂезжжиијјклљљМНЊЊЊЊОПРссттћћуффифлхцччџЏЏЏЏЏШШ",
        ),
    ];

    #[test]
    fn test_consume() -> Result<(), usize> {
        for e in 0..CONSUMED_BUFFER {
            let mut proc = StreamProcessor::new(Direction::LatToCyr);
            proc.buffer_consumed[..e].copy_from_slice(&EXAMPLES[e].0.as_bytes()[..e]);
            let mut consumed: usize = 0;
            while consumed < EXAMPLES[e].0.as_bytes().len() {
                match proc.consume(&EXAMPLES[e].0.as_bytes()[consumed..]) {
                    Ok(_) => {
                        break;
                    }
                    Err(v) => {
                        consumed += v;
                        // Fake digest
                        proc.digested = 0;
                        proc.consumed = 0;
                        // Clear buffer
                        proc.buffer_consumed = [0u8; CONSUMED_BUFFER];
                    }
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test_digest() -> Result<(), Error> {
        for (_, e) in EXAMPLES {
            let mut proc = StreamProcessor::new(Direction::LatToCyr);
            let mut chars: Vec<char> = vec![];
            e.chars().for_each(|c| chars.push(c));
            if chars.len() < DIGESTED_BUFFER / 3 {
                continue;
            }
            // Fill up a part of digested buffer
            proc.buffer_digested[..DIGESTED_BUFFER / 3]
                .copy_from_slice(&chars[..DIGESTED_BUFFER / 3]);
            proc.digested = DIGESTED_BUFFER / 3;
            let mut remaining_char: Vec<u8> = vec![];
            for c in &chars[DIGESTED_BUFFER / 3..] {
                let mut b = c.to_string().as_bytes().to_vec();
                if b.len() > 1 && proc.consumed >= CONSUMED_BUFFER - 5 {
                    // Intentionally break last character
                    proc.buffer_consumed[proc.consumed] = b[0];
                    proc.consumed += 1;
                    b.remove(0);
                    remaining_char.append(&mut b);
                    break;
                }
                proc.buffer_consumed[proc.consumed..proc.consumed + b.len()].copy_from_slice(&b);
                proc.consumed += b.len();
                if proc.consumed >= CONSUMED_BUFFER {
                    break;
                }
            }
            proc.digest()?;
            let mut d = proc.digested;
            assert_eq!(
                proc.buffer_digested[..proc.digested],
                chars[..proc.digested]
            );
            if remaining_char.is_empty() {
                continue;
            }
            // Check if last caracter gets stitched
            proc.buffer_consumed[proc.consumed..proc.consumed + remaining_char.len()]
                .copy_from_slice(&remaining_char);
            proc.consumed += remaining_char.len();
            proc.digest()?;
            assert!(proc.digested == d + 1);
            assert_eq!(
                proc.buffer_digested[..proc.digested],
                chars[..proc.digested]
            );
        }
        Ok(())
    }

}
