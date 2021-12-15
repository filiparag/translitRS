#[allow(dead_code)]
pub const SEPARATORS: &[char] = &[
    '\u{0020}', '\u{00A0}', '\u{1680}', '\u{180E}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}',
    '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}', '\u{200A}', '\u{200B}',
    '\u{202F}', '\u{205F}', '\u{3000}', '\u{FEFF}',
];

#[derive(PartialEq)]
pub enum Case {
    Upper,
    Mixed,
    Lower,
}

#[derive(PartialEq)]
pub struct Character<'a> {
    pub value: &'a [char],
    pub case: Case,
}

pub const EMPTY: &[Character] = &[];

// Borrowed from Jovan Turanjanin's Ćirilizator
// https://github.com/turanjanin/cirilizator

pub const LATIN_CLEAN: &[Character] = &[
    Character {
        value: &['A'],
        case: Case::Upper,
    },
    Character {
        value: &['B'],
        case: Case::Upper,
    },
    Character {
        value: &['V'],
        case: Case::Upper,
    },
    Character {
        value: &['G'],
        case: Case::Upper,
    },
    Character {
        value: &['D'],
        case: Case::Upper,
    },
    Character {
        value: &['Đ'],
        case: Case::Upper,
    },
    Character {
        value: &['E'],
        case: Case::Upper,
    },
    Character {
        value: &['Ž'],
        case: Case::Upper,
    },
    Character {
        value: &['Z'],
        case: Case::Upper,
    },
    Character {
        value: &['I'],
        case: Case::Upper,
    },
    Character {
        value: &['J'],
        case: Case::Upper,
    },
    Character {
        value: &['K'],
        case: Case::Upper,
    },
    Character {
        value: &['L'],
        case: Case::Upper,
    },
    Character {
        value: &['L', 'J'],
        case: Case::Upper,
    }, //
    Character {
        value: &['L', 'j'],
        case: Case::Mixed,
    }, //
    Character {
        value: &['M'],
        case: Case::Upper,
    },
    Character {
        value: &['N'],
        case: Case::Upper,
    },
    Character {
        value: &['N', 'J'],
        case: Case::Upper,
    }, //
    Character {
        value: &['N', 'j'],
        case: Case::Mixed,
    }, //
    Character {
        value: &['O'],
        case: Case::Upper,
    },
    Character {
        value: &['P'],
        case: Case::Upper,
    },
    Character {
        value: &['R'],
        case: Case::Upper,
    },
    Character {
        value: &['S'],
        case: Case::Upper,
    },
    Character {
        value: &['T'],
        case: Case::Upper,
    },
    Character {
        value: &['Ć'],
        case: Case::Upper,
    },
    Character {
        value: &['U'],
        case: Case::Upper,
    },
    Character {
        value: &['F'],
        case: Case::Upper,
    },
    Character {
        value: &['H'],
        case: Case::Upper,
    },
    Character {
        value: &['C'],
        case: Case::Upper,
    },
    Character {
        value: &['Č'],
        case: Case::Upper,
    },
    Character {
        value: &['D', 'Ž'],
        case: Case::Upper,
    }, //
    Character {
        value: &['D', 'ž'],
        case: Case::Mixed,
    }, //
    Character {
        value: &['Š'],
        case: Case::Upper,
    },
    Character {
        value: &['a'],
        case: Case::Lower,
    },
    Character {
        value: &['b'],
        case: Case::Lower,
    },
    Character {
        value: &['v'],
        case: Case::Lower,
    },
    Character {
        value: &['g'],
        case: Case::Lower,
    },
    Character {
        value: &['d'],
        case: Case::Lower,
    },
    Character {
        value: &['đ'],
        case: Case::Lower,
    },
    Character {
        value: &['e'],
        case: Case::Lower,
    },
    Character {
        value: &['ž'],
        case: Case::Lower,
    },
    Character {
        value: &['z'],
        case: Case::Lower,
    },
    Character {
        value: &['i'],
        case: Case::Lower,
    },
    Character {
        value: &['j'],
        case: Case::Lower,
    },
    Character {
        value: &['k'],
        case: Case::Lower,
    },
    Character {
        value: &['l'],
        case: Case::Lower,
    },
    Character {
        value: &['l', 'j'],
        case: Case::Lower,
    },
    Character {
        value: &['m'],
        case: Case::Lower,
    },
    Character {
        value: &['n'],
        case: Case::Lower,
    },
    Character {
        value: &['n', 'j'],
        case: Case::Lower,
    },
    Character {
        value: &['o'],
        case: Case::Lower,
    },
    Character {
        value: &['p'],
        case: Case::Lower,
    },
    Character {
        value: &['r'],
        case: Case::Lower,
    },
    Character {
        value: &['s'],
        case: Case::Lower,
    },
    Character {
        value: &['t'],
        case: Case::Lower,
    },
    Character {
        value: &['ć'],
        case: Case::Lower,
    },
    Character {
        value: &['u'],
        case: Case::Lower,
    },
    Character {
        value: &['f'],
        case: Case::Lower,
    },
    Character {
        value: &['h'],
        case: Case::Lower,
    },
    Character {
        value: &['c'],
        case: Case::Lower,
    },
    Character {
        value: &['č'],
        case: Case::Lower,
    },
    Character {
        value: &['d', 'ž'],
        case: Case::Lower,
    },
    Character {
        value: &['š'],
        case: Case::Lower,
    },
];

pub const LATIN_CLEAN_UNICODE: &[Character] = &[
    Character {
        value: &['A'],
        case: Case::Upper,
    },
    Character {
        value: &['B'],
        case: Case::Upper,
    },
    Character {
        value: &['V'],
        case: Case::Upper,
    },
    Character {
        value: &['G'],
        case: Case::Upper,
    },
    Character {
        value: &['D'],
        case: Case::Upper,
    },
    Character {
        value: &['Đ'],
        case: Case::Upper,
    },
    Character {
        value: &['E'],
        case: Case::Upper,
    },
    Character {
        value: &['Ž'],
        case: Case::Upper,
    },
    Character {
        value: &['Z'],
        case: Case::Upper,
    },
    Character {
        value: &['I'],
        case: Case::Upper,
    },
    Character {
        value: &['J'],
        case: Case::Upper,
    },
    Character {
        value: &['K'],
        case: Case::Upper,
    },
    Character {
        value: &['L'],
        case: Case::Upper,
    },
    Character {
        value: &['Ǉ'],
        case: Case::Upper,
    }, //
    Character {
        value: &['ǈ'],
        case: Case::Mixed,
    }, //
    Character {
        value: &['M'],
        case: Case::Upper,
    },
    Character {
        value: &['N'],
        case: Case::Upper,
    },
    Character {
        value: &['Ǌ'],
        case: Case::Upper,
    }, //
    Character {
        value: &['ǋ'],
        case: Case::Mixed,
    }, //
    Character {
        value: &['O'],
        case: Case::Upper,
    },
    Character {
        value: &['P'],
        case: Case::Upper,
    },
    Character {
        value: &['R'],
        case: Case::Upper,
    },
    Character {
        value: &['S'],
        case: Case::Upper,
    },
    Character {
        value: &['T'],
        case: Case::Upper,
    },
    Character {
        value: &['Ć'],
        case: Case::Upper,
    },
    Character {
        value: &['U'],
        case: Case::Upper,
    },
    Character {
        value: &['F'],
        case: Case::Upper,
    },
    Character {
        value: &['H'],
        case: Case::Upper,
    },
    Character {
        value: &['C'],
        case: Case::Upper,
    },
    Character {
        value: &['Č'],
        case: Case::Upper,
    },
    Character {
        value: &['Ǆ'],
        case: Case::Upper,
    }, //
    Character {
        value: &['ǅ'],
        case: Case::Mixed,
    }, //
    Character {
        value: &['Š'],
        case: Case::Upper,
    },
    Character {
        value: &['a'],
        case: Case::Lower,
    },
    Character {
        value: &['b'],
        case: Case::Lower,
    },
    Character {
        value: &['v'],
        case: Case::Lower,
    },
    Character {
        value: &['g'],
        case: Case::Lower,
    },
    Character {
        value: &['d'],
        case: Case::Lower,
    },
    Character {
        value: &['đ'],
        case: Case::Lower,
    },
    Character {
        value: &['e'],
        case: Case::Lower,
    },
    Character {
        value: &['ž'],
        case: Case::Lower,
    },
    Character {
        value: &['z'],
        case: Case::Lower,
    },
    Character {
        value: &['i'],
        case: Case::Lower,
    },
    Character {
        value: &['j'],
        case: Case::Lower,
    },
    Character {
        value: &['k'],
        case: Case::Lower,
    },
    Character {
        value: &['l'],
        case: Case::Lower,
    },
    Character {
        value: &['ǉ'],
        case: Case::Lower,
    },
    Character {
        value: &['m'],
        case: Case::Lower,
    },
    Character {
        value: &['n'],
        case: Case::Lower,
    },
    Character {
        value: &['ǌ'],
        case: Case::Lower,
    },
    Character {
        value: &['o'],
        case: Case::Lower,
    },
    Character {
        value: &['p'],
        case: Case::Lower,
    },
    Character {
        value: &['r'],
        case: Case::Lower,
    },
    Character {
        value: &['s'],
        case: Case::Lower,
    },
    Character {
        value: &['t'],
        case: Case::Lower,
    },
    Character {
        value: &['ć'],
        case: Case::Lower,
    },
    Character {
        value: &['u'],
        case: Case::Lower,
    },
    Character {
        value: &['f'],
        case: Case::Lower,
    },
    Character {
        value: &['h'],
        case: Case::Lower,
    },
    Character {
        value: &['c'],
        case: Case::Lower,
    },
    Character {
        value: &['č'],
        case: Case::Lower,
    },
    Character {
        value: &['ǆ'],
        case: Case::Lower,
    },
    Character {
        value: &['š'],
        case: Case::Lower,
    },
];

pub const CYRILLIC_CLEAN: &[Character] = &[
    Character {
        value: &['А'],
        case: Case::Upper,
    },
    Character {
        value: &['Б'],
        case: Case::Upper,
    },
    Character {
        value: &['В'],
        case: Case::Upper,
    },
    Character {
        value: &['Г'],
        case: Case::Upper,
    },
    Character {
        value: &['Д'],
        case: Case::Upper,
    },
    Character {
        value: &['Ђ'],
        case: Case::Upper,
    },
    Character {
        value: &['Е'],
        case: Case::Upper,
    },
    Character {
        value: &['Ж'],
        case: Case::Upper,
    },
    Character {
        value: &['З'],
        case: Case::Upper,
    },
    Character {
        value: &['И'],
        case: Case::Upper,
    },
    Character {
        value: &['Ј'],
        case: Case::Upper,
    },
    Character {
        value: &['К'],
        case: Case::Upper,
    },
    Character {
        value: &['Л'],
        case: Case::Upper,
    },
    Character {
        value: &['Љ'],
        case: Case::Upper,
    }, //
    Character {
        value: &['Љ'],
        case: Case::Mixed,
    }, //
    Character {
        value: &['М'],
        case: Case::Upper,
    },
    Character {
        value: &['Н'],
        case: Case::Upper,
    },
    Character {
        value: &['Њ'],
        case: Case::Upper,
    }, //
    Character {
        value: &['Њ'],
        case: Case::Mixed,
    }, //
    Character {
        value: &['О'],
        case: Case::Upper,
    },
    Character {
        value: &['П'],
        case: Case::Upper,
    },
    Character {
        value: &['Р'],
        case: Case::Upper,
    },
    Character {
        value: &['С'],
        case: Case::Upper,
    },
    Character {
        value: &['Т'],
        case: Case::Upper,
    },
    Character {
        value: &['Ћ'],
        case: Case::Upper,
    },
    Character {
        value: &['У'],
        case: Case::Upper,
    },
    Character {
        value: &['Ф'],
        case: Case::Upper,
    },
    Character {
        value: &['Х'],
        case: Case::Upper,
    },
    Character {
        value: &['Ц'],
        case: Case::Upper,
    },
    Character {
        value: &['Ч'],
        case: Case::Upper,
    },
    Character {
        value: &['Џ'],
        case: Case::Upper,
    }, //
    Character {
        value: &['Џ'],
        case: Case::Mixed,
    }, //
    Character {
        value: &['Ш'],
        case: Case::Upper,
    },
    Character {
        value: &['а'],
        case: Case::Lower,
    },
    Character {
        value: &['б'],
        case: Case::Lower,
    },
    Character {
        value: &['в'],
        case: Case::Lower,
    },
    Character {
        value: &['г'],
        case: Case::Lower,
    },
    Character {
        value: &['д'],
        case: Case::Lower,
    },
    Character {
        value: &['ђ'],
        case: Case::Lower,
    },
    Character {
        value: &['е'],
        case: Case::Lower,
    },
    Character {
        value: &['ж'],
        case: Case::Lower,
    },
    Character {
        value: &['з'],
        case: Case::Lower,
    },
    Character {
        value: &['и'],
        case: Case::Lower,
    },
    Character {
        value: &['ј'],
        case: Case::Lower,
    },
    Character {
        value: &['к'],
        case: Case::Lower,
    },
    Character {
        value: &['л'],
        case: Case::Lower,
    },
    Character {
        value: &['љ'],
        case: Case::Lower,
    },
    Character {
        value: &['м'],
        case: Case::Lower,
    },
    Character {
        value: &['н'],
        case: Case::Lower,
    },
    Character {
        value: &['њ'],
        case: Case::Lower,
    },
    Character {
        value: &['о'],
        case: Case::Lower,
    },
    Character {
        value: &['п'],
        case: Case::Lower,
    },
    Character {
        value: &['р'],
        case: Case::Lower,
    },
    Character {
        value: &['с'],
        case: Case::Lower,
    },
    Character {
        value: &['т'],
        case: Case::Lower,
    },
    Character {
        value: &['ћ'],
        case: Case::Lower,
    },
    Character {
        value: &['у'],
        case: Case::Lower,
    },
    Character {
        value: &['ф'],
        case: Case::Lower,
    },
    Character {
        value: &['х'],
        case: Case::Lower,
    },
    Character {
        value: &['ц'],
        case: Case::Lower,
    },
    Character {
        value: &['ч'],
        case: Case::Lower,
    },
    Character {
        value: &['џ'],
        case: Case::Lower,
    },
    Character {
        value: &['ш'],
        case: Case::Lower,
    },
];

pub const LATIN_DIRTY: &[Character] = &[
    Character {
        value: &['A'],
        case: Case::Upper,
    },
    Character {
        value: &['B'],
        case: Case::Upper,
    },
    Character {
        value: &['V'],
        case: Case::Upper,
    },
    Character {
        value: &['G'],
        case: Case::Upper,
    },
    Character {
        value: &['D'],
        case: Case::Upper,
    },
    Character {
        value: &['Đ'],
        case: Case::Upper,
    },
    Character {
        value: &['Ð'],
        case: Case::Upper,
    },
    Character {
        value: &['D', 'J'],
        case: Case::Upper,
    },
    Character {
        value: &['D', 'j'],
        case: Case::Mixed,
    },
    Character {
        value: &['E'],
        case: Case::Upper,
    },
    Character {
        value: &['Z'],
        case: Case::Upper,
    },
    Character {
        value: &['Ž'],
        case: Case::Upper,
    },
    Character {
        value: &['\u{17d}'],
        case: Case::Upper,
    },
    Character {
        value: &['I'],
        case: Case::Upper,
    },
    Character {
        value: &['J'],
        case: Case::Upper,
    },
    Character {
        value: &['K'],
        case: Case::Upper,
    },
    Character {
        value: &['L'],
        case: Case::Upper,
    },
    Character {
        value: &['L', 'J'],
        case: Case::Upper,
    },
    Character {
        value: &['Ǉ'],
        case: Case::Upper,
    },
    Character {
        value: &['L', 'j'],
        case: Case::Mixed,
    },
    Character {
        value: &['ǈ'],
        case: Case::Mixed,
    },
    Character {
        value: &['M'],
        case: Case::Upper,
    },
    Character {
        value: &['N'],
        case: Case::Upper,
    },
    Character {
        value: &['N', 'J'],
        case: Case::Upper,
    },
    Character {
        value: &['Ǌ'],
        case: Case::Upper,
    },
    Character {
        value: &['N', 'j'],
        case: Case::Mixed,
    },
    Character {
        value: &['ǋ'],
        case: Case::Mixed,
    },
    Character {
        value: &['O'],
        case: Case::Upper,
    },
    Character {
        value: &['P'],
        case: Case::Upper,
    },
    Character {
        value: &['R'],
        case: Case::Upper,
    },
    Character {
        value: &['S'],
        case: Case::Upper,
    },
    Character {
        value: &['T'],
        case: Case::Upper,
    },
    Character {
        value: &['Ć'],
        case: Case::Upper,
    },
    Character {
        value: &['\u{106}'],
        case: Case::Upper,
    },
    Character {
        value: &['U'],
        case: Case::Upper,
    },
    Character {
        value: &['F'],
        case: Case::Upper,
    },
    Character {
        value: &['H'],
        case: Case::Upper,
    },
    Character {
        value: &['C'],
        case: Case::Upper,
    },
    Character {
        value: &['Č'],
        case: Case::Upper,
    },
    Character {
        value: &['\u{10c}'],
        case: Case::Upper,
    },
    Character {
        value: &['D', 'Ž'],
        case: Case::Upper,
    },
    Character {
        value: &['Ǆ'],
        case: Case::Upper,
    },
    Character {
        value: &['D', '\u{17d}'],
        case: Case::Upper,
    },
    Character {
        value: &['D', 'ž'],
        case: Case::Mixed,
    },
    Character {
        value: &['ǅ'],
        case: Case::Mixed,
    },
    Character {
        value: &['D', '\u{17e}'],
        case: Case::Mixed,
    },
    Character {
        value: &['Š'],
        case: Case::Upper,
    },
    Character {
        value: &['\u{160}'],
        case: Case::Upper,
    },
    Character {
        value: &['a'],
        case: Case::Lower,
    },
    Character {
        value: &['æ'],
        case: Case::Lower,
    },
    Character {
        value: &['b'],
        case: Case::Lower,
    },
    Character {
        value: &['v'],
        case: Case::Lower,
    },
    Character {
        value: &['g'],
        case: Case::Lower,
    },
    Character {
        value: &['d'],
        case: Case::Lower,
    },
    Character {
        value: &['đ'],
        case: Case::Lower,
    },
    Character {
        value: &['d', 'j'],
        case: Case::Lower,
    },
    Character {
        value: &['e'],
        case: Case::Lower,
    },
    Character {
        value: &['z'],
        case: Case::Lower,
    },
    Character {
        value: &['ž'],
        case: Case::Lower,
    },
    Character {
        value: &['\u{17e}'],
        case: Case::Lower,
    },
    Character {
        value: &['i'],
        case: Case::Lower,
    },
    Character {
        value: &['ĳ'],
        case: Case::Lower,
    },
    Character {
        value: &['j'],
        case: Case::Lower,
    },
    Character {
        value: &['k'],
        case: Case::Lower,
    },
    Character {
        value: &['l'],
        case: Case::Lower,
    },
    Character {
        value: &['l', 'j'],
        case: Case::Lower,
    },
    Character {
        value: &['ǉ'],
        case: Case::Lower,
    },
    Character {
        value: &['m'],
        case: Case::Lower,
    },
    Character {
        value: &['n'],
        case: Case::Lower,
    },
    Character {
        value: &['n', 'j'],
        case: Case::Lower,
    },
    Character {
        value: &['ǌ'],
        case: Case::Lower,
    },
    Character {
        value: &['o'],
        case: Case::Lower,
    },
    Character {
        value: &['œ'],
        case: Case::Lower,
    },
    Character {
        value: &['p'],
        case: Case::Lower,
    },
    Character {
        value: &['r'],
        case: Case::Lower,
    },
    Character {
        value: &['s'],
        case: Case::Lower,
    },
    Character {
        value: &['ﬆ'],
        case: Case::Lower,
    },
    Character {
        value: &['t'],
        case: Case::Lower,
    },
    Character {
        value: &['ć'],
        case: Case::Lower,
    },
    Character {
        value: &['\u{107}'],
        case: Case::Lower,
    },
    Character {
        value: &['u'],
        case: Case::Lower,
    },
    Character {
        value: &['f'],
        case: Case::Lower,
    },
    Character {
        value: &['ﬁ'],
        case: Case::Lower,
    },
    Character {
        value: &['ﬂ'],
        case: Case::Lower,
    },
    Character {
        value: &['h'],
        case: Case::Lower,
    },
    Character {
        value: &['c'],
        case: Case::Lower,
    },
    Character {
        value: &['č'],
        case: Case::Lower,
    },
    Character {
        value: &['\u{10d}'],
        case: Case::Lower,
    },
    Character {
        value: &['d', 'ž'],
        case: Case::Lower,
    },
    Character {
        value: &['ǆ'],
        case: Case::Lower,
    },
    Character {
        value: &['d', '\u{17e}'],
        case: Case::Lower,
    },
    Character {
        value: &['š'],
        case: Case::Lower,
    },
    Character {
        value: &['\u{161}'],
        case: Case::Lower,
    },
];

pub const LATIN_DIRTY_UNICODE: &[Character] = &[
    Character {
        value: &['A'],
        case: Case::Upper,
    },
    Character {
        value: &['B'],
        case: Case::Upper,
    },
    Character {
        value: &['V'],
        case: Case::Upper,
    },
    Character {
        value: &['G'],
        case: Case::Upper,
    },
    Character {
        value: &['D'],
        case: Case::Upper,
    },
    Character {
        value: &['Đ'],
        case: Case::Upper,
    },
    Character {
        value: &['Đ'],
        case: Case::Upper,
    },
    Character {
        value: &['Đ'],
        case: Case::Upper,
    },
    Character {
        value: &['Đ'],
        case: Case::Upper,
    },
    Character {
        value: &['E'],
        case: Case::Upper,
    },
    Character {
        value: &['Z'],
        case: Case::Upper,
    },
    Character {
        value: &['Ž'],
        case: Case::Upper,
    },
    Character {
        value: &['Ž'],
        case: Case::Upper,
    },
    Character {
        value: &['I'],
        case: Case::Upper,
    },
    Character {
        value: &['J'],
        case: Case::Upper,
    },
    Character {
        value: &['K'],
        case: Case::Upper,
    },
    Character {
        value: &['L'],
        case: Case::Upper,
    },
    Character {
        value: &['Ǉ'],
        case: Case::Upper,
    },
    Character {
        value: &['Ǉ'],
        case: Case::Upper,
    },
    Character {
        value: &['ǈ'],
        case: Case::Mixed,
    },
    Character {
        value: &['ǈ'],
        case: Case::Mixed,
    },
    Character {
        value: &['M'],
        case: Case::Upper,
    },
    Character {
        value: &['N'],
        case: Case::Upper,
    },
    Character {
        value: &['Ǌ'],
        case: Case::Upper,
    },
    Character {
        value: &['Ǌ'],
        case: Case::Upper,
    },
    Character {
        value: &['ǋ'],
        case: Case::Mixed,
    },
    Character {
        value: &['ǋ'],
        case: Case::Mixed,
    },
    Character {
        value: &['O'],
        case: Case::Upper,
    },
    Character {
        value: &['P'],
        case: Case::Upper,
    },
    Character {
        value: &['R'],
        case: Case::Upper,
    },
    Character {
        value: &['S'],
        case: Case::Upper,
    },
    Character {
        value: &['T'],
        case: Case::Upper,
    },
    Character {
        value: &['Ć'],
        case: Case::Upper,
    },
    Character {
        value: &['Ć'],
        case: Case::Upper,
    },
    Character {
        value: &['U'],
        case: Case::Upper,
    },
    Character {
        value: &['F'],
        case: Case::Upper,
    },
    Character {
        value: &['H'],
        case: Case::Upper,
    },
    Character {
        value: &['C'],
        case: Case::Upper,
    },
    Character {
        value: &['Č'],
        case: Case::Upper,
    },
    Character {
        value: &['Č'],
        case: Case::Upper,
    },
    Character {
        value: &['Ǆ'],
        case: Case::Upper,
    },
    Character {
        value: &['Ǆ'],
        case: Case::Upper,
    },
    Character {
        value: &['Ǆ'],
        case: Case::Upper,
    },
    Character {
        value: &['ǅ'],
        case: Case::Mixed,
    },
    Character {
        value: &['ǅ'],
        case: Case::Mixed,
    },
    Character {
        value: &['ǅ'],
        case: Case::Mixed,
    },
    Character {
        value: &['Š'],
        case: Case::Upper,
    },
    Character {
        value: &['Š'],
        case: Case::Upper,
    },
    Character {
        value: &['a'],
        case: Case::Lower,
    },
    Character {
        value: &['a', 'e'],
        case: Case::Lower,
    },
    Character {
        value: &['b'],
        case: Case::Lower,
    },
    Character {
        value: &['v'],
        case: Case::Lower,
    },
    Character {
        value: &['g'],
        case: Case::Lower,
    },
    Character {
        value: &['d'],
        case: Case::Lower,
    },
    Character {
        value: &['đ'],
        case: Case::Lower,
    },
    Character {
        value: &['đ'],
        case: Case::Lower,
    },
    Character {
        value: &['e'],
        case: Case::Lower,
    },
    Character {
        value: &['z'],
        case: Case::Lower,
    },
    Character {
        value: &['ž'],
        case: Case::Lower,
    },
    Character {
        value: &['ž'],
        case: Case::Lower,
    },
    Character {
        value: &['i'],
        case: Case::Lower,
    },
    Character {
        value: &['i', 'j'],
        case: Case::Lower,
    },
    Character {
        value: &['j'],
        case: Case::Lower,
    },
    Character {
        value: &['k'],
        case: Case::Lower,
    },
    Character {
        value: &['l'],
        case: Case::Lower,
    },
    Character {
        value: &['ǉ'],
        case: Case::Lower,
    },
    Character {
        value: &['ǉ'],
        case: Case::Lower,
    },
    Character {
        value: &['m'],
        case: Case::Lower,
    },
    Character {
        value: &['n'],
        case: Case::Lower,
    },
    Character {
        value: &['ǌ'],
        case: Case::Lower,
    },
    Character {
        value: &['ǌ'],
        case: Case::Lower,
    },
    Character {
        value: &['o'],
        case: Case::Lower,
    },
    Character {
        value: &['o', 'e'],
        case: Case::Lower,
    },
    Character {
        value: &['p'],
        case: Case::Lower,
    },
    Character {
        value: &['r'],
        case: Case::Lower,
    },
    Character {
        value: &['s'],
        case: Case::Lower,
    },
    Character {
        value: &['š', 't'],
        case: Case::Lower,
    },
    Character {
        value: &['t'],
        case: Case::Lower,
    },
    Character {
        value: &['ć'],
        case: Case::Lower,
    },
    Character {
        value: &['ć'],
        case: Case::Lower,
    },
    Character {
        value: &['u'],
        case: Case::Lower,
    },
    Character {
        value: &['f'],
        case: Case::Lower,
    },
    Character {
        value: &['f', 'i'],
        case: Case::Lower,
    },
    Character {
        value: &['f', 'l'],
        case: Case::Lower,
    },
    Character {
        value: &['h'],
        case: Case::Lower,
    },
    Character {
        value: &['c'],
        case: Case::Lower,
    },
    Character {
        value: &['č'],
        case: Case::Lower,
    },
    Character {
        value: &['č'],
        case: Case::Lower,
    },
    Character {
        value: &['ǆ'],
        case: Case::Lower,
    },
    Character {
        value: &['ǆ'],
        case: Case::Lower,
    },
    Character {
        value: &['ǆ'],
        case: Case::Lower,
    },
    Character {
        value: &['š'],
        case: Case::Lower,
    },
    Character {
        value: &['š'],
        case: Case::Lower,
    },
];

pub const CYRILLIC_DIRTY: &[Character] = &[
    Character {
        value: &['А'],
        case: Case::Upper,
    },
    Character {
        value: &['Б'],
        case: Case::Upper,
    },
    Character {
        value: &['В'],
        case: Case::Upper,
    },
    Character {
        value: &['Г'],
        case: Case::Upper,
    },
    Character {
        value: &['Д'],
        case: Case::Upper,
    },
    Character {
        value: &['Ђ'],
        case: Case::Upper,
    },
    Character {
        value: &['Ђ'],
        case: Case::Upper,
    },
    Character {
        value: &['Ђ'],
        case: Case::Upper,
    },
    Character {
        value: &['Ђ'],
        case: Case::Upper,
    },
    Character {
        value: &['Е'],
        case: Case::Upper,
    },
    Character {
        value: &['З'],
        case: Case::Upper,
    },
    Character {
        value: &['Ж'],
        case: Case::Upper,
    },
    Character {
        value: &['Ж'],
        case: Case::Upper,
    },
    Character {
        value: &['И'],
        case: Case::Upper,
    },
    Character {
        value: &['Ј'],
        case: Case::Upper,
    },
    Character {
        value: &['К'],
        case: Case::Upper,
    },
    Character {
        value: &['Л'],
        case: Case::Upper,
    },
    Character {
        value: &['Љ'],
        case: Case::Upper,
    },
    Character {
        value: &['Љ'],
        case: Case::Upper,
    },
    Character {
        value: &['Љ'],
        case: Case::Mixed,
    },
    Character {
        value: &['Љ'],
        case: Case::Mixed,
    },
    Character {
        value: &['М'],
        case: Case::Upper,
    },
    Character {
        value: &['Н'],
        case: Case::Upper,
    },
    Character {
        value: &['Њ'],
        case: Case::Upper,
    },
    Character {
        value: &['Њ'],
        case: Case::Upper,
    },
    Character {
        value: &['Њ'],
        case: Case::Mixed,
    },
    Character {
        value: &['Њ'],
        case: Case::Mixed,
    },
    Character {
        value: &['О'],
        case: Case::Upper,
    },
    Character {
        value: &['П'],
        case: Case::Upper,
    },
    Character {
        value: &['Р'],
        case: Case::Upper,
    },
    Character {
        value: &['С'],
        case: Case::Upper,
    },
    Character {
        value: &['Т'],
        case: Case::Upper,
    },
    Character {
        value: &['Ћ'],
        case: Case::Upper,
    },
    Character {
        value: &['Ћ'],
        case: Case::Upper,
    },
    Character {
        value: &['У'],
        case: Case::Upper,
    },
    Character {
        value: &['Ф'],
        case: Case::Upper,
    },
    Character {
        value: &['Х'],
        case: Case::Upper,
    },
    Character {
        value: &['Ц'],
        case: Case::Upper,
    },
    Character {
        value: &['Ч'],
        case: Case::Upper,
    },
    Character {
        value: &['Ч'],
        case: Case::Upper,
    },
    Character {
        value: &['Џ'],
        case: Case::Upper,
    },
    Character {
        value: &['Џ'],
        case: Case::Upper,
    },
    Character {
        value: &['Џ'],
        case: Case::Upper,
    },
    Character {
        value: &['Џ'],
        case: Case::Mixed,
    },
    Character {
        value: &['Џ'],
        case: Case::Mixed,
    },
    Character {
        value: &['Џ'],
        case: Case::Mixed,
    },
    Character {
        value: &['Ш'],
        case: Case::Upper,
    },
    Character {
        value: &['Ш'],
        case: Case::Upper,
    },
    Character {
        value: &['а'],
        case: Case::Lower,
    },
    Character {
        value: &['а', 'е'],
        case: Case::Lower,
    },
    Character {
        value: &['б'],
        case: Case::Lower,
    },
    Character {
        value: &['в'],
        case: Case::Lower,
    },
    Character {
        value: &['г'],
        case: Case::Lower,
    },
    Character {
        value: &['д'],
        case: Case::Lower,
    },
    Character {
        value: &['ђ'],
        case: Case::Lower,
    },
    Character {
        value: &['ђ'],
        case: Case::Lower,
    },
    Character {
        value: &['е'],
        case: Case::Lower,
    },
    Character {
        value: &['з'],
        case: Case::Lower,
    },
    Character {
        value: &['ж'],
        case: Case::Lower,
    },
    Character {
        value: &['ж'],
        case: Case::Lower,
    },
    Character {
        value: &['и'],
        case: Case::Lower,
    },
    Character {
        value: &['и', 'ј'],
        case: Case::Lower,
    },
    Character {
        value: &['ј'],
        case: Case::Lower,
    },
    Character {
        value: &['к'],
        case: Case::Lower,
    },
    Character {
        value: &['л'],
        case: Case::Lower,
    },
    Character {
        value: &['љ'],
        case: Case::Lower,
    },
    Character {
        value: &['љ'],
        case: Case::Lower,
    },
    Character {
        value: &['м'],
        case: Case::Lower,
    },
    Character {
        value: &['н'],
        case: Case::Lower,
    },
    Character {
        value: &['њ'],
        case: Case::Lower,
    },
    Character {
        value: &['њ'],
        case: Case::Lower,
    },
    Character {
        value: &['о'],
        case: Case::Lower,
    },
    Character {
        value: &['о', 'е'],
        case: Case::Lower,
    },
    Character {
        value: &['п'],
        case: Case::Lower,
    },
    Character {
        value: &['р'],
        case: Case::Lower,
    },
    Character {
        value: &['с'],
        case: Case::Lower,
    },
    Character {
        value: &['с', 'т'],
        case: Case::Lower,
    },
    Character {
        value: &['т'],
        case: Case::Lower,
    },
    Character {
        value: &['ћ'],
        case: Case::Lower,
    },
    Character {
        value: &['ћ'],
        case: Case::Lower,
    },
    Character {
        value: &['у'],
        case: Case::Lower,
    },
    Character {
        value: &['ф'],
        case: Case::Lower,
    },
    Character {
        value: &['ф', 'и'],
        case: Case::Lower,
    },
    Character {
        value: &['ф', 'л'],
        case: Case::Lower,
    },
    Character {
        value: &['х'],
        case: Case::Lower,
    },
    Character {
        value: &['ц'],
        case: Case::Lower,
    },
    Character {
        value: &['ч'],
        case: Case::Lower,
    },
    Character {
        value: &['ч'],
        case: Case::Lower,
    },
    Character {
        value: &['џ'],
        case: Case::Lower,
    },
    Character {
        value: &['џ'],
        case: Case::Lower,
    },
    Character {
        value: &['џ'],
        case: Case::Lower,
    },
    Character {
        value: &['ш'],
        case: Case::Lower,
    },
    Character {
        value: &['ш'],
        case: Case::Lower,
    },
];

pub struct DigraphException<'a> {
    pub latin: &'a [Character<'a>],
    pub latinized: &'a [Character<'a>],
    pub cyrillic: &'a [Character<'a>],
    pub exceptions: &'a [&'a str],
}

pub const DIGRAPH_EXCEPTIONS: &[DigraphException<'static>] = &[
    DigraphException {
        latin: &[
            Character {
                value: &['Đ'],
                case: Case::Upper,
            },
            Character {
                value: &['Ð'],
                case: Case::Upper,
            },
            Character {
                value: &['D', 'J'],
                case: Case::Upper,
            },
            Character {
                value: &['D', 'j'],
                case: Case::Mixed,
            },
            Character {
                value: &['đ'],
                case: Case::Lower,
            },
            Character {
                value: &['d', 'j'],
                case: Case::Lower,
            },
        ],
        latinized: &[
            Character {
                value: &['D', 'J'],
                case: Case::Upper,
            },
            Character {
                value: &['D', 'J'],
                case: Case::Upper,
            },
            Character {
                value: &['D', 'J'],
                case: Case::Upper,
            },
            Character {
                value: &['D', 'j'],
                case: Case::Mixed,
            },
            Character {
                value: &['d', 'j'],
                case: Case::Lower,
            },
            Character {
                value: &['d', 'j'],
                case: Case::Lower,
            },
        ],
        cyrillic: &[
            Character {
                value: &['Д', 'J'],
                case: Case::Upper,
            },
            Character {
                value: &['Д', 'J'],
                case: Case::Upper,
            },
            Character {
                value: &['Д', 'Ј'],
                case: Case::Upper,
            },
            Character {
                value: &['Д', 'ј'],
                case: Case::Mixed,
            },
            Character {
                value: &['д', 'ј'],
                case: Case::Lower,
            },
            Character {
                value: &['д', 'ј'],
                case: Case::Lower,
            },
        ],
        exceptions: DIGRAPH_EXCEPTIONS_DJ,
    },
    DigraphException {
        latin: &[
            Character {
                value: &['D', 'Ž'],
                case: Case::Upper,
            },
            Character {
                value: &['Ǆ'],
                case: Case::Upper,
            },
            Character {
                value: &['D', '\u{17d}'],
                case: Case::Upper,
            },
            Character {
                value: &['D', 'ž'],
                case: Case::Mixed,
            },
            Character {
                value: &['ǅ'],
                case: Case::Mixed,
            },
            Character {
                value: &['D', '\u{17e}'],
                case: Case::Mixed,
            },
            Character {
                value: &['d', 'ž'],
                case: Case::Lower,
            },
            Character {
                value: &['ǆ'],
                case: Case::Lower,
            },
        ],
        latinized: &[
            Character {
                value: &['D', 'Ž'],
                case: Case::Upper,
            },
            Character {
                value: &['D', 'Ž'],
                case: Case::Upper,
            },
            Character {
                value: &['D', 'Ž'],
                case: Case::Upper,
            },
            Character {
                value: &['D', 'ž'],
                case: Case::Mixed,
            },
            Character {
                value: &['D', 'ž'],
                case: Case::Mixed,
            },
            Character {
                value: &['D', 'ž'],
                case: Case::Mixed,
            },
            Character {
                value: &['d', 'ž'],
                case: Case::Lower,
            },
            Character {
                value: &['d', 'ž'],
                case: Case::Lower,
            },
        ],
        cyrillic: &[
            Character {
                value: &['Д', 'Ж'],
                case: Case::Upper,
            },
            Character {
                value: &['Д', 'Ж'],
                case: Case::Upper,
            },
            Character {
                value: &['Д', 'Ж'],
                case: Case::Upper,
            },
            Character {
                value: &['Д', 'ж'],
                case: Case::Mixed,
            },
            Character {
                value: &['Д', 'ж'],
                case: Case::Mixed,
            },
            Character {
                value: &['Д', 'ж'],
                case: Case::Mixed,
            },
            Character {
                value: &['д', 'ж'],
                case: Case::Lower,
            },
            Character {
                value: &['д', 'ж'],
                case: Case::Lower,
            },
        ],
        exceptions: DIGRAPH_EXCEPTIONS_DZ,
    },
    DigraphException {
        latin: &[
            Character {
                value: &['N', 'J'],
                case: Case::Upper,
            },
            Character {
                value: &['Ǌ'],
                case: Case::Upper,
            },
            Character {
                value: &['N', 'j'],
                case: Case::Mixed,
            },
            Character {
                value: &['ǋ'],
                case: Case::Mixed,
            },
            Character {
                value: &['n', 'j'],
                case: Case::Lower,
            },
            Character {
                value: &['ǌ'],
                case: Case::Lower,
            },
        ],
        latinized: &[
            Character {
                value: &['N', 'J'],
                case: Case::Upper,
            },
            Character {
                value: &['N', 'J'],
                case: Case::Upper,
            },
            Character {
                value: &['N', 'j'],
                case: Case::Mixed,
            },
            Character {
                value: &['N', 'j'],
                case: Case::Mixed,
            },
            Character {
                value: &['n', 'j'],
                case: Case::Lower,
            },
            Character {
                value: &['n', 'j'],
                case: Case::Lower,
            },
        ],
        cyrillic: &[
            Character {
                value: &['Н', 'Ј'],
                case: Case::Upper,
            },
            Character {
                value: &['Н', 'Ј'],
                case: Case::Upper,
            },
            Character {
                value: &['Н', 'ј'],
                case: Case::Mixed,
            },
            Character {
                value: &['Н', 'ј'],
                case: Case::Mixed,
            },
            Character {
                value: &['н', 'ј'],
                case: Case::Lower,
            },
            Character {
                value: &['н', 'ј'],
                case: Case::Lower,
            },
        ],
        exceptions: DIGRAPH_EXCEPTIONS_NJ,
    },
];

pub const DIGRAPH_EXCEPTIONS_DJ: &[&str] = &[
    "adjektiv",
    "adjunkt",
    "bazdje",
    "bdje",
    "bezdje",
    "blijedje",
    "bludje",
    "bridjе",
    "vidjel",
    "vidjet",
    "vindjakn",
    "višenedje",
    "vrijedje",
    "gdje",
    "gudje",
    "gdjir",
    "daždje",
    "dvonedje",
    "devetonedje",
    "desetonedje",
    "djb",
    "djeva",
    "djevi",
    "djevo",
    "djed",
    "djejstv",
    "djel",
    "djenem",
    "djeneš",
    "djenu",
    "djet",
    "djec",
    "dječ",
    "djuar",
    "djubison",
    "djubouz",
    "djuer",
    "djui",
    "djuks",
    "djulej",
    "djumars",
    "djupont",
    "djurant",
    "djusenberi",
    "djuharst",
    "djuherst",
    "dovdje",
    "dogrdje",
    "dodjel",
    "drvodje",
    "drugdje",
    "elektrosnabdje",
    "žudje",
    "zabludje",
    "zavidje",
    "zavrijedje",
    "zagudje",
    "zadjev",
    "zadjen",
    "zalebdje",
    "zaludje",
    "zaodje",
    "zapodje",
    "zarudje",
    "zasjedje",
    "zasmrdje",
    "zastidje",
    "zaštedje",
    "zdje",
    "zlodje",
    "igdje",
    "izbledje",
    "izblijedje",
    "izvidje",
    "izdjejst",
    "izdjelj",
    "izludje",
    "isprdje",
    "jednonedje",
    "kojegdje",
    "kudjelj",
    "lebdje",
    "ludjel",
    "ludjet",
    "makfadjen",
    "marmadjuk",
    "međudjel",
    "nadjaha",
    "nadjača",
    "nadjeb",
    "nadjev",
    "nadjenul",
    "nadjenuo",
    "nadjenut",
    "negdje",
    "nedjel",
    "nadjunač",
    "nenadjača",
    "nenadjebi",
    "nenavidje",
    "neodje",
    "nepodjarm",
    "nerazdje",
    "nigdje",
    "obdjel",
    "obnevidje",
    "ovdje",
    "odjav",
    "odjah",
    "odjaš",
    "odjeb",
    "odjev",
    "odjed",
    "odjezd",
    "odjek",
    "odjel",
    "odjen",
    "odjeć",
    "odjec",
    "odjur",
    "odsjedje",
    "ondje",
    "opredje",
    "osijedje",
    "osmonedje",
    "pardju",
    "perdju",
    "petonedje",
    "poblijedje",
    "povidje",
    "pogdjegdje",
    "pogdje",
    "podjakn",
    "podjamč",
    "podjastu",
    "podjemč",
    "podjar",
    "podjeb",
    "podjed",
    "podjezič",
    "podjel",
    "podjen",
    "podjet",
    "pododjel",
    "pozavidje",
    "poludje",
    "poljodjel",
    "ponegdje",
    "ponedjelj",
    "porazdje",
    "posijedje",
    "posjedje",
    "postidje",
    "potpodjel",
    "poštedje",
    "pradjed",
    "prdje",
    "preblijedje",
    "previdje",
    "predvidje",
    "predjel",
    "preodjen",
    "preraspodje",
    "presjedje",
    "pridjev",
    "pridjen",
    "prismrdje",
    "prištedje",
    "probdje",
    "problijedje",
    "prodjen",
    "prolebdje",
    "prosijedje",
    "prosjedje",
    "protivdjel",
    "prošlonedje",
    "radjard",
    "razvidje",
    "razdjev",
    "razdjel",
    "razodje",
    "raspodje",
    "rasprdje",
    "remekdjel",
    "rudjen",
    "rudjet",
    "sadje",
    "svagdje",
    "svidje",
    "svugdje",
    "sedmonedjelj",
    "sijedje",
    "sjedje",
    "smrdje",
    "snabdje",
    "snovidje",
    "starosjedje",
    "stidje",
    "studje",
    "sudjel",
    "tronedje",
    "ublijedje",
    "uvidje",
    "udjel",
    "udjen",
    "uprdje",
    "usidjel",
    "usjedje",
    "usmrdje",
    "uštedje",
    "cjelonedje",
    "četvoronedje",
    "čukundjed",
    "šestonedjelj",
    "štedje",
    "štogdje",
    "šukundjed",
];

pub const DIGRAPH_EXCEPTIONS_DZ: &[&str] = &[
    "feldžandarm",
    "nadžanj",
    "nadždrel",
    "nadžel",
    "nadžeo",
    "nadžet",
    "nadživ",
    "nadžinj",
    "nadžnj",
    "nadžrec",
    "nadžup",
    "odžali",
    "odžari",
    "odžel",
    "odžive",
    "odživljava",
    "odžubor",
    "odžvaka",
    "odžval",
    "odžvać",
    "podžanr",
    "podžel",
    "podže",
    "podžig",
    "podžiz",
    "podžil",
    "podžnje",
    "podžupan",
    "predželu",
    "predživot",
];

pub const DIGRAPH_EXCEPTIONS_NJ: &[&str] = &[
    "anjon",
    "injaric",
    "injekc",
    "injekt",
    "injicira",
    "injurij",
    "kenjon",
    "konjug",
    "konjunk",
    "nekonjug",
    "nekonjunk",
    "ssrnj",
    "tanjug",
    "vanjezičk",
];
