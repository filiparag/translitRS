use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp, fmt, str, string};
use subslice::bmh;

mod charmaps;

use charmaps::{Case as LetterCase, Character};

#[derive(Clone)]
pub enum Charset {
    Latin,
    LatinUnicode,
    Cyrillic,
}

pub struct Transliterator {
    charset_from: &'static [Character<'static>],
    charset_into: &'static [Character<'static>],
    exceptions: bool,
    skip_digraph: bool,
    force_foreign: bool,
    force_links: bool,
}

#[derive(Debug)]
pub enum Error {
    BufferOverflow,
    Utf8(str::Utf8Error),
    FromUtf8(string::FromUtf8Error),
}

impl From<str::Utf8Error> for Error {
    fn from(error: str::Utf8Error) -> Self {
        Self::Utf8(error)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(error: string::FromUtf8Error) -> Self {
        Self::FromUtf8(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BufferOverflow => writeln!(f, "Buffer Overflow"),
            Self::Utf8(e) => writeln!(f, "UTF-8 error - {}", e),
            Self::FromUtf8(e) => writeln!(f, "From UTF-8 error - {}", e),
        }
    }
}

impl Default for Transliterator {
    fn default() -> Self {
        Self {
            charset_from: charmaps::LATIN_DIRTY,
            charset_into: charmaps::CYRILLIC_DIRTY,
            exceptions: true,
            skip_digraph: false,
            force_foreign: false,
            force_links: false,
        }
    }
}

impl Transliterator {
    pub fn new(
        from: Charset,
        into: Charset,
        skip_digraph: bool,
        force_foreign: bool,
        force_links: bool,
    ) -> Self {
        let (f, i, e) = match (from, into) {
            (Charset::Latin, Charset::Latin) => (charmaps::EMPTY, charmaps::EMPTY, false),
            (Charset::LatinUnicode, Charset::LatinUnicode) => {
                (charmaps::EMPTY, charmaps::EMPTY, false)
            }
            (Charset::Cyrillic, Charset::Cyrillic) => (charmaps::EMPTY, charmaps::EMPTY, false),
            //
            (Charset::Latin, Charset::LatinUnicode) => {
                (charmaps::LATIN_DIRTY, charmaps::LATIN_DIRTY_UNICODE, true)
            }
            (Charset::LatinUnicode, Charset::Latin) => {
                (charmaps::LATIN_CLEAN_UNICODE, charmaps::LATIN_CLEAN, false)
            }
            //
            (Charset::Latin, Charset::Cyrillic) => {
                (charmaps::LATIN_DIRTY, charmaps::CYRILLIC_DIRTY, true)
            }
            (Charset::LatinUnicode, Charset::Cyrillic) => {
                (charmaps::LATIN_CLEAN_UNICODE, charmaps::CYRILLIC_CLEAN, true)
            }
            //
            (Charset::Cyrillic, Charset::Latin) => {
                (charmaps::CYRILLIC_CLEAN, charmaps::LATIN_CLEAN, false)
            }
            (Charset::Cyrillic, Charset::LatinUnicode) => {
                (charmaps::CYRILLIC_CLEAN, charmaps::LATIN_CLEAN_UNICODE, false)
            }
        };
        assert_eq!(f.len(), i.len());
        Self {
            charset_from: f,
            charset_into: i,
            exceptions: e,
            skip_digraph,
            force_foreign,
            force_links,
        }
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
        latinize: bool,
    ) -> Result<Option<&'a Character<'a>>, Error> {
        for exception in charmaps::DIGRAPH_EXCEPTIONS {
            for i in 0..exception.latin.len() {
                if exception.latin[i].value == character {
                    let mut lowercase: Vec<u8> = vec![0; word.len() * 4];
                    let mut cursor: usize = 0;
                    for letter in word {
                        for c in letter.to_lowercase() {
                            cursor += Self::chars_to_utf8(&[c], &mut lowercase[cursor..])?;
                        }
                    }
                    for e in exception.exceptions {
                        if bmh::find(&lowercase, e.as_bytes()).is_some() {
                            if latinize {
                                return Ok(Some(&exception.latinized[i]));
                            } else {
                                return Ok(Some(&exception.cyrillic[i]));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn foreign_pattern_exception(word: &str) -> bool {
        lazy_static! {
            // Borrowed from https://stackoverflow.com/a/26093611
            static ref RE_DOMAIN: Regex = Regex::new(
                r"[a-zA-Z0-9][a-zA-Z0-9-]{1,61}[a-zA-Z0-9](?:\.[a-zA-Z]{2,})+"
            ).unwrap();
            // Borrowed from https://www.emailregex.com/
            static ref RE_EMAIL: Regex =
                Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,6}\b").unwrap();
            // Borrowed from http://urlregex.com/
            static ref RE_URL: Regex = Regex::new(
                r"((http[s]?://)|(./)|(/))(?:[a-zA-Z]|[0-9]|[$-_@.&+]|[!*\(\),]|(?:%[0-9a-fA-F][0-9a-fA-F]))+"
            ).unwrap();
            // Borrowed from https://github.com/turanjanin/cirilizator
            static ref RE_MEASUREMENT: Regex = Regex::new(
                r"(\d+([\.,]\d)*)((K|??[FC]|[kKMGTPEY](ib|b|iB|B|Hz)|[pn??mcdhk]m[????]?|m[????]|[mcdkh][lg])|([zafpn??mcdhKMGTPEY]?([BVWJFSHC??ATNhlmg]|m[????]|s[??]|cd|Pa|Wb|Hz|deg|rad)))"
            ).unwrap();
        }
        RE_DOMAIN.is_match(word)
            || RE_EMAIL.is_match(word)
            || RE_URL.is_match(word)
            || RE_MEASUREMENT.is_match(word)
    }

    pub fn process_word(&self, word: &str) -> Result<String, Error> {
        let mut out: Vec<u8> = vec![0; word.len() * 4];
        let chars = word.chars().into_iter().collect::<Vec<char>>();
        let mut cursor_in: usize = 0;
        let mut cursor_out: usize = 0;
        let mut force_process: bool = false;
        if Self::foreign_pattern_exception(word) {
            if self.force_links {
                force_process = true;
            } else {
                return Ok(word.to_string());
            }
        }
        'outer: while cursor_in < chars.len() {
            'inner: for (i, Character { value: c, case: lc }) in
                self.charset_from.iter().enumerate().rev()
            {
                if chars[cursor_in..].starts_with(c) {
                    if !self.skip_digraph && self.exceptions {
                        // If transliterating to latin8, transliterate exception too
                        let latinize = self.charset_into == charmaps::LATIN_CLEAN_UNICODE
                            || self.charset_into == charmaps::LATIN_DIRTY_UNICODE;
                        // Start from bottom to catch digraphs first
                        if let Some(exception) = Self::digraph_exception(&chars, c, latinize)? {
                            cursor_out +=
                                Self::chars_to_utf8(exception.value, &mut out[cursor_out..])?;
                            cursor_in += exception.value.len();
                            continue 'outer;
                        }
                    }
                    // Check if digraph is preceded or followed by same case
                    if lc == &LetterCase::Mixed {
                        let prev_lower = if cursor_in > 0 {
                            Some(chars[cursor_in - 1].is_lowercase())
                        } else {
                            None
                        };
                        let next_lower = if cursor_in < chars.len() - 1 {
                            Some(chars[cursor_in + 1].is_lowercase())
                        } else {
                            None
                        };
                        if matches!((prev_lower, next_lower), (_, Some(false))) {
                            continue 'inner;
                        }
                    }
                    // Exception is not found, proceed to transliterate
                    cursor_out +=
                        Self::chars_to_utf8(self.charset_into[i].value, &mut out[cursor_out..])?;
                    cursor_in += c.len();
                    continue 'outer;
                }
            }
            if !force_process && !self.force_foreign && chars[cursor_in].is_alphabetic() {
                // Foreign character is found, return original
                return Ok(word.to_string());
            } else {
                // Add found non-alphabetic or foreign character
                cursor_out += Self::chars_to_utf8(&[chars[cursor_in]], &mut out[cursor_out..])?;
                cursor_in += 1;
            }
        }
        out.resize(cursor_out, 0);
        let out = String::from_utf8(out)?;
        Ok(out)
    }

    pub fn process<S: AsRef<str>>(&self, input: S) -> Result<String, Error> {
        let input = input.as_ref();
        let mut output = String::with_capacity(input.len());
        let mut cursor_left = 0;
        fn next_occurence(input: &str, left: usize, match_text: bool) -> usize {
            let criterion = match match_text {
                true => |c: char| c.is_whitespace(),
                false => |c: char| !c.is_whitespace(),
            };
            if let Some(res) = input[left..].find(criterion) {
                cmp::min(left + res, input.len())
            } else {
                input.len()
            }
        }
        let mut match_text: bool = false;
        while cursor_left < input.len() {
            let cursor_right = next_occurence(input, cursor_left, match_text);
            if match_text {
                let res = self.process_word(&input[cursor_left..cursor_right])?;
                output.push_str(&res);
            } else {
                // Skip processing space characters
                output.push_str(&input[cursor_left..cursor_right]);
            }
            cursor_left = cursor_right;
            // Toggle between processing whitespace and other characters
            match_text = !match_text;
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: &'static [(&str, &str, bool)] = &[
        ("", "", true),
        ("1234567890", "1234567890", true),
        (
            "A B V G D ?? E ?? Z I J K L Lj M N Nj O P R S T ?? U F H C ?? D?? ?? a b v g d ?? e ?? z i j k l lj m n nj o p r s t ?? u f h c ?? d?? ??",
            "?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ??",
            true,
        ),
        (
            "Stala mala Mara na kraj stara hana sama.",
            "?????????? ???????? ???????? ???? ???????? ?????????? ???????? ????????.",
            true,
        ),
        (
            "Nevesele snene ??ene plele te??ke mre??e",
            "???????????????? ?????????? ???????? ?????????? ?????????? ??????????",
            true,
        ),
        (
            "Javorov jaram, javorova ralica, ralo drvo javorovo.",
            "?????????????? ??????????, ???????????????? ????????????, ???????? ???????? ????????????????.",
            true,
        ),
        (
            "Jesi li to ti to tu? Jesi li to tu ti? Jesi li to ti tu? Jesi li tu ti to?",
            "???????? ???? ???? ???? ???? ????? ???????? ???? ???? ???? ????? ???????? ???? ???? ???? ????? ???????? ???? ???? ???? ?????",
            true,
        ),
        (
            "Adjektivisati|ZABLUDJE|od??va??|Pred??ivot|kenjon|konjug|TANJug",
            "??????????????????????????|????????????????|????????????|??????????????????|????????????|????????????|????????????",
            true,
        ),
        (
            "A B V G D ?? ?? DJ Dj E Z ?? ?? I J K L LJ ?? Lj ?? M N NJ ?? Nj ?? O P R S T ?? ?? U F H C ?? ?? D?? ?? D?? D?? ?? D?? ?? ??",
            "?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ??",
            false,
        ),
        (
            "a ?? b v g d ?? dj e z ?? ?? i ?? j k l lj ?? m n nj ?? o ?? p r s ??? t ?? ?? u f ??? ??? h c ?? ?? d?? ?? d?? ?? ??",
            "?? ???? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ???? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ???? ?? ?? ?? ???? ?? ?? ?? ?? ?? ???? ???? ?? ?? ?? ?? ?? ?? ?? ?? ??",
            false,
        ),
        (
            "A B V G D ?? ?? DJ Dj e z ?? ?? i ?? j k l lj ?? M N NJ ?? Nj ?? O P R s ??? t ?? ?? u f ??? ??? h c ?? ?? d?? ?? D?? D?? ?? D?? ?? ??",
            "?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ???? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ???? ?? ?? ?? ?? ?? ???? ???? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ??",
            false,
        ),
    ];

    #[test]
    fn test_charsets() -> Result<(), Error> {
        let charsets = vec![Charset::Latin, Charset::LatinUnicode, Charset::Cyrillic];
        for f in charsets.clone() {
            for i in charsets.clone() {
                let _ = Transliterator::new(f.clone(), i.clone(), false, false, false);
            }
        }
        Ok(())
    }

    #[test]
    fn test_chars_to_utf8() -> Result<(), Error> {
        let mut output: Vec<u8> = vec![0; 100];
        let len = Transliterator::chars_to_utf8(
            &['??', '??', '??', ' ', '??', 'a', 'r', 'a', 'd', '??'],
            &mut output,
        )?;
        assert_eq!(String::from_utf8_lossy(&output[..len]), "?????? ??arad??");
        Ok(())
    }

    #[test]
    fn test_digraph_exception() -> Result<(), Error> {
        assert_eq!(
            Transliterator::digraph_exception(
                &['a', 'D', 'r', 'u', 'g', 'd', 'j', 'e', 'd'],
                &['??'],
                false
            )?
            .unwrap()
            .value,
            &['??', '??']
        );
        assert_eq!(
            Transliterator::digraph_exception(
                &['a', 'D', 'r', 'u', 'g', 'd', 'j', 'e', 'd'],
                &['??'],
                true
            )?
            .unwrap()
            .value,
            &['d', 'j']
        );
        assert_eq!(
            Transliterator::digraph_exception(
                &['n', 'a', 'D', '??', 'i', 'v', 'e', 't', 'i'],
                &['D', '??'],
                false
            )?
            .unwrap()
            .value,
            &['??', '??']
        );
        assert_eq!(
            Transliterator::digraph_exception(
                &['n', 'a', 'D', '??', 'i', 'v', 'e', 't', 'i'],
                &['D', '??'],
                true
            )?
            .unwrap()
            .value,
            &['D', '??']
        );
        assert_eq!(
            Transliterator::digraph_exception(
                &['d', 'a', 'N', 'J', 'o', 'n', 'i'],
                &['N', 'J'],
                false
            )?
            .unwrap()
            .value,
            &['??', '??']
        );
        assert_eq!(
            Transliterator::digraph_exception(
                &['d', 'a', 'N', 'J', 'o', 'n', 'i'],
                &['N', 'J'],
                true
            )?
            .unwrap()
            .value,
            &['N', 'J']
        );
        Ok(())
    }

    #[test]
    fn test_transliterate_lat_cyr() -> Result<(), Error> {
        for (lat, cyr, _) in EXAMPLES {
            let t = Transliterator::new(Charset::Latin, Charset::Cyrillic, false, false, false);
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
            let t = Transliterator::new(Charset::Cyrillic, Charset::Latin, false, false, false);
            let res = t.process(cyr)?;
            assert_eq!(&&res, lat);
        }
        Ok(())
    }

    #[test]
    fn test_skip_foreign() -> Result<(), Error> {
        let t = Transliterator::new(Charset::Latin, Charset::Cyrillic, false, false, false);
        for text in vec!["example", "??????", "????????", "p??lda"] {
            assert_eq!(text, t.process_word(text)?);
        }
        let t = Transliterator::new(Charset::Latin, Charset::Cyrillic, false, true, false);
        for (text, expected) in vec![
            ("example", "??x??????????"),
            ("??????", "??????"),
            ("????????", "????????"),
            ("p??lda", "??????????"),
        ] {
            assert_eq!(expected, t.process_word(text)?);
        }
        Ok(())
    }

    #[test]
    fn test_digraph_capitalization() -> Result<(), Error> {
        let t_lat_cyr = Transliterator::new(Charset::Latin, Charset::Cyrillic, false, false, false);
        let t_lat8_cyr =
            Transliterator::new(Charset::LatinUnicode, Charset::Cyrillic, false, false, false);
        //
        let t_lat_lat8 =
            Transliterator::new(Charset::Latin, Charset::LatinUnicode, false, false, false);
        let t_lat8_lat =
            Transliterator::new(Charset::LatinUnicode, Charset::Latin, false, false, false);
        //
        let t_cyr_lat = Transliterator::new(Charset::Cyrillic, Charset::Latin, false, false, false);
        let t_cyr_lat8 =
            Transliterator::new(Charset::Cyrillic, Charset::LatinUnicode, false, false, false);
        for (latin, latin8, cyrillic) in vec![
            // ??
            ("Ljubi??a", "??ubi??a", "????????????"),
            ("ljubi??a", "??ubi??a", "????????????"),
            ("LJUBI??A", "??UBI??A", "????????????"),
            ("Naljutiti", "Na??utiti", "????????????????"),
            ("naljutiti", "na??utiti", "????????????????"),
            ("NALJUTITI", "NA??UTITI", "????????????????"),
            // ??
            ("Njiva", "??iva", "????????"),
            ("njiva", "??iva", "????????"),
            ("NJIVA", "??IVA", "????????"),
            ("Anja", "A??a", "??????"),
            ("anja", "a??a", "??????"),
            ("ANJA", "A??A", "??????"),
            // ??
            ("D??onatan", "??onatan", "??????????????"),
            ("d??onatan", "??onatan", "??????????????"),
            ("D??ONATAN", "??ONATAN", "??????????????"),
            ("Mid??or", "Mi??or", "??????????"),
            ("mid??or", "mi??or", "??????????"),
            ("MID??OR", "MI??OR", "??????????"),
        ] {
            assert_eq!(cyrillic, t_lat_cyr.process_word(latin)?);
            assert_eq!(cyrillic, t_lat8_cyr.process_word(latin8)?);
            //
            assert_eq!(latin8, t_lat_lat8.process_word(latin)?);
            assert_eq!(latin, t_lat8_lat.process_word(latin8)?);
            //
            assert_eq!(latin, t_cyr_lat.process_word(cyrillic)?);
            assert_eq!(latin8, t_cyr_lat8.process_word(cyrillic)?);
        }
        Ok(())
    }
}
