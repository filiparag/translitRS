[![Crates.io](https://img.shields.io/crates/v/translitRS)](https://crates.io/crates/translitRS)
[![Build status](https://img.shields.io/github/workflow/status/filiparag/translitRS/Code%20testing)](https://github.com/filiparag/translitRS/actions/workflows/rust.yml)
[![License](https://img.shields.io/crates/l/translitRS)](https://github.com/filiparag/translitRS/blob/master/LICENSE)

## translitRS — Transliterator for Serbian Language

TranslitRS is a command-line utility for transliteration between Cyrillic and Latin scripts of the Serbian language. It can work on plain text files directly, or as a filter for [Pandoc](https://pandoc.org/) document processor (*Markdown*, *HTML*, *LaTeX*, *Microsoft Word*...).

## Usage

### Arguments
  
- `-i, --input <path>` \
  Read input from file \
  Default: *standard input*
- `-o, --output <path>` \
  Write output to file \
  Default: *standard output*
- `-f, --from <charset>` \
  Convert from [character set](#character-sets) \
  Default: *latin*
- `-t, --into <charset>` \
  Convert to [character set](#character-sets) \
  Default: *cyrillic*
- `-d, --skip-digraph` \
  Do not check for digraph exceptions
- `-u, --force-foreign` \
  Process words with foreign and mixed characters
- `-l, --force-links` \
  Process hyperlinks, email addresses and units
- `-p, --pandoc-filter` \
  Run in Pandoc JSON pipe [filter mode](#pandoc-filter-mode)
- `-v, --version` \
  Show version and quit
- `-h, --help` \
  Show usage help and quit

### Character sets

Listed below are available character sets and their shorthand codes:

- Serbian Latin \
  `latin, lat, l`
- Serbian Latin (Unicode) \
  `latin8, lat8, l8`
- Serbian Cyrillic \
  `cyrillic, cyr, c`

### Pandoc filter mode

When running as a Pandoc filter, the arguments listed above can't be passed directly. Instead, use the following arguments variables:

- `CHARS_FROM=<charset>` \
Convert from character set
- `CHARS_INTO=<charset>` \
Convert to character set
- `SKIP_DIGRAPH=1` \
Do not check for digraph exceptions
- `FORCE_FOREIGN=1` \
Process words with foreign and mixed characters
- `FORCE_LINKS=1` \
Process hyperlinks, email addresses and units

### Examples
```sh
# Transliterate plaintext file from Latin (Unicode) to Cyrillic
translitrs -f lat8 -t cyr -i source.txt -o destination.txt

# Transliterate Microsoft Word document from Cyrillic to Latin
CHARS_FROM=c CHARS_INTO=l pandoc essay.docx --filter translitrs -o essay.docx
```
