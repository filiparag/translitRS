use std::{fmt, io, str, string};
use subslice::bmh;

mod charmaps;

pub enum Direction {
    LatinToCyrillic,
    CyrillicToLatin,
}

pub struct Transliterate {
    direction: Direction,
}

pub enum Error {
    EmptyDigest,
    BufferOverflow,
    IoError(io::Error),
    UTFError(str::Utf8Error),
    FromUTFError(string::FromUtf8Error),
}

impl From<str::Utf8Error> for Error {
    fn from(error: str::Utf8Error) -> Self {
        Self::UTFError(error)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(error: string::FromUtf8Error) -> Self {
        Self::FromUTFError(error)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDigest => writeln!(f, "Digest is empty"),
            Self::BufferOverflow => writeln!(f, "Buffer Overflow"),
            Self::IoError(e) => writeln!(f, "IO error - {}", e),
            Self::UTFError(e) => writeln!(f, "UTF-8 error - {}", e),
            Self::FromUTFError(e) => writeln!(f, "From UTF-8 error - {}", e),
        }
    }
}

impl Default for Transliterate {
    fn default() -> Self {
        Self {
            direction: Direction::LatinToCyrillic,
        }
    }
}

impl Transliterate {
    pub fn new(direction: Direction) -> Self {
        Self { direction }
    }

    fn chars_to_utf8(input: &[char], output: &mut [u8]) -> Result<usize, Error> {
        let mut cursor: usize = 0;
        for c in input {
            let length = c.len_utf8();
            if cursor + length > output.len() {
                return Err(Error::BufferOverflow);
            }
            c.encode_utf8(&mut output[cursor..cursor + length]);
            cursor += length;
        }
        Ok(cursor)
    }

    fn digraph_exception<'a>(
        word: &[char],
        character: &'a [char],
    ) -> Result<Option<&'a [char]>, Error> {
        for exception in charmaps::DIGRAPH_EXCEPTIONS {
            for i in 0..exception.latin.len() {
                if exception.latin[i] == character {
                    let mut lowercase: Vec<u8> = Vec::with_capacity(word.len() * 4);
                    lowercase.resize(word.len() * 4, 0);
                    let mut cursor: usize = 0;
                    for letter in word {
                        for c in letter.to_lowercase() {
                            cursor += Self::chars_to_utf8(&[c], &mut lowercase[cursor..])?;
                        }
                    }
                    for e in exception.exceptions {
                        if let Some(_) = bmh::find(&lowercase, e.as_bytes()) {
                            return Ok(Some(exception.cyrillic[i]));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn process_word(&self, word: &str) -> Result<String, Error> {
        let mut out: Vec<u8> = Vec::with_capacity(word.len() * 4);
        out.resize(word.len() * 4, 0);
        let chars = word.chars().into_iter().collect::<Vec<char>>();

        let mut cursor_in: usize = 0;
        let mut cursor_out: usize = 0;

        'outer: while cursor_in < chars.len() {
            for (i, c) in match self.direction {
                Direction::LatinToCyrillic => charmaps::LATIN_DIRTY,
                Direction::CyrillicToLatin => charmaps::CYRILLIC_CLEAN,
            }
            .iter()
            .enumerate()
            .rev()
            {
                if chars[cursor_in..].starts_with(c) {
                    if let Direction::LatinToCyrillic = self.direction {
                        // Start from bottom to catch digraphs first
                        if let Some(exception) = Self::digraph_exception(&chars, c)? {
                            cursor_out += Self::chars_to_utf8(exception, &mut out[cursor_out..])?;
                            cursor_in += exception.len();
                            continue 'outer;
                        }
                    }
                    // Exception is not found, proceed to transliterate
                    cursor_out += Self::chars_to_utf8(
                        match self.direction {
                            Direction::LatinToCyrillic => charmaps::CYRILLIC_DIRTY,
                            Direction::CyrillicToLatin => charmaps::LATIN_CLEAN,
                        }[i],
                        &mut out[cursor_out..],
                    )?;
                    cursor_in += c.len();
                    continue 'outer;
                }
            }
            cursor_out += Self::chars_to_utf8(&[chars[cursor_in]], &mut out[cursor_out..])?;
            cursor_in += 1;
        }
        out.resize(cursor_out, 0);
        let out = String::from_utf8(out)?;
        Ok(out)
    }

    pub fn process<S: AsRef<str>>(&self, input: S) -> Result<String, Error> {
        let mut output = String::with_capacity(input.as_ref().len());
        let words = input.as_ref().split_whitespace();
        for w in words {
            let res = self.process_word(w)?;
            // eprintln!("Processing {}", w);
            // eprintln!("         = {}", res);
            output.push_str(&res);
            output.push(' ')
        }
        output.pop();
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    const EXAMPLES: &'static [(&str, &str, bool)] = &[
        ("", "", true),
        ("1234567890", "1234567890", true),
        (
            "ABVGDĐEŽZIJKLLjMNNjOPRSTĆUFHCČDžŠabvgdđežzijklljmnnjoprstćufhcčdžš",
            "АБВГДЂЕЖЗИЈКЛЉМНЊОПРСТЋУФХЦЧЏШабвгдђежзијклљмнњопрстћуфхцчџш",
            true,
        ),
        (
            "Stala mala Mara na kraj stara hana sama.",
            "Стала мала Мара на крај стара хана сама.",
            true,
        ),
        (
            "Nevesele snene žene plele teške mreže",
            "Невеселе снене жене плеле тешке мреже",
            true,
        ),
        (
            "Javorov jaram, javorova ralica, ralo drvo javorovo.",
            "Јаворов јарам, јаворова ралица, рало дрво јаворово.",
            true,
        ),
        (
            "Jesi li to ti to tu? Jesi li to tu ti? Jesi li to ti tu? Jesi li tu ti to?",
            "Јеси ли то ти то ту? Јеси ли то ту ти? Јеси ли то ти ту? Јеси ли ту ти то?",
            true,
        ),
        (
            "Adjektivisati|ZABLUDJE|odžvać|PredŽivot|kenjon|konjug|TANJug",
            "Адјективисати|ЗАБЛУДЈЕ|оджваћ|ПредЖивот|кенјон|конјуг|ТАНЈуг",
            true,
        ),
        (
            "ABVGDĐÐDJDjEZŽŽIJKLLJǇLjǈMNNJǊNjǋOPRSTĆĆUFHCČČDŽǄDŽDžǅDžŠŠ",
            "АБВГДЂЂЂЂЕЗЖЖИЈКЛЉЉЉЉМНЊЊЊЊОПРСТЋЋУФХЦЧЧЏЏЏЏЏЏШШ",
            false,
        ),
        (
            "aæbvgdđdjezžžiĳjklljǉmnnjǌoœprsﬆtććufﬁﬂhcččdžǆdžšš",
            "ааебвгдђђезжжиијјклљљмнњњооепрссттћћуффифлхцччџџџшш",
            false,
        ),
        (
            "ABVGDĐÐDJDjezžžiĳjklljǉMNNJǊNjǋOPRsﬆtććufﬁﬂhcččdžǄDŽDžǅDžŠŠ",
            "АБВГДЂЂЂЂезжжиијјклљљМНЊЊЊЊОПРссттћћуффифлхцччџЏЏЏЏЏШШ",
            false,
        ),
    ];

    // #[test]
    // fn test() -> Result<(), Error> {
    //     let t = Transliterate::new(Direction::LatinToCyrillic);
    //     t.process("abc\u{00A0}\u{2005}\u{2003}def\u{2008}ghi\u{3000}jkl\u{202F}\u{2006}mno")?;
    //     Ok(())
    // }

    // #[test]
    // fn test_chars_to_utf8() -> Result<(), Error> {
    //     let mut output: Vec<u8> = vec![0; 100];
    //     let len = Transliterate::chars_to_utf8(
    //         &['В', 'у', 'к', ' ', 'К', 'a', 'r', 'a', 'd', 'ž'],
    //         &mut output,
    //     )?;
    //     eprintln!(
    //         ">>>>> {} {} \n{:?}",
    //         len,
    //         String::from_utf8_lossy(&output),
    //         &output
    //     );
    //     Ok(())
    // }

    // #[test]
    // fn test_digraph_exception() -> Result<(), Error> {
    //     let mut output: Vec<u8> = vec![0; 64];
    //     eprintln!(
    //         ">> {:?}",
    //         Transliterate::digraph_exception(
    //             &['a', 'D', 'r', 'u', 'g', 'd', 'j', 'e', 'd'],
    //             &['đ'],
    //         )?
    //     );
    //     Ok(())
    // }

    #[test]
    fn test_transliterate_lat_cyr() -> Result<(), Error> {
        for (lat, cyr, clean) in EXAMPLES {
            let t = Transliterate::new(Direction::LatinToCyrillic);
            let res = t.process(lat)?;
            assert_eq!(&&res, cyr);
        }
        Ok(())
    }

    #[test]
    fn test_transliterate_cyr_lat() -> Result<(), Error> {
        for (lat, cyr, clean) in EXAMPLES {
            if !clean {
                continue;
            }
            let t = Transliterate::new(Direction::CyrillicToLatin);
            let res = t.process(cyr)?;
            assert_eq!(&&res, lat);
        }
        Ok(())
    }
}
