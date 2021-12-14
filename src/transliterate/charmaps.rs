#[allow(dead_code)]
pub const SEPARATORS: &[char] = &[
    '\u{0020}', '\u{00A0}', '\u{1680}', '\u{180E}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}',
    '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}', '\u{200A}', '\u{200B}',
    '\u{202F}', '\u{205F}', '\u{3000}', '\u{FEFF}',
];

pub const EMPTY: &[&[char]] = &[];

// Borrowed from Jovan Turanjanin's Ćirilizator
// https://github.com/turanjanin/cirilizator

pub const LATIN_CLEAN: &[&[char]] = &[
    &['A'],
    &['B'],
    &['V'],
    &['G'],
    &['D'],
    &['Đ'],
    &['E'],
    &['Ž'],
    &['Z'],
    &['I'],
    &['J'],
    &['K'],
    &['L'],
    &['L', 'J'], //
    &['L', 'j'], //
    &['M'],
    &['N'],
    &['N', 'N'], //
    &['N', 'j'], //
    &['O'],
    &['P'],
    &['R'],
    &['S'],
    &['T'],
    &['Ć'],
    &['U'],
    &['F'],
    &['H'],
    &['C'],
    &['Č'],
    &['D', 'Ž'], //
    &['D', 'ž'], //
    &['Š'],
    &['a'],
    &['b'],
    &['v'],
    &['g'],
    &['d'],
    &['đ'],
    &['e'],
    &['ž'],
    &['z'],
    &['i'],
    &['j'],
    &['k'],
    &['l'],
    &['l', 'j'],
    &['m'],
    &['n'],
    &['n', 'j'],
    &['o'],
    &['p'],
    &['r'],
    &['s'],
    &['t'],
    &['ć'],
    &['u'],
    &['f'],
    &['h'],
    &['c'],
    &['č'],
    &['d', 'ž'],
    &['š'],
];

pub const LATIN_CLEAN_UNICODE: &[&[char]] = &[
    &['A'],
    &['B'],
    &['V'],
    &['G'],
    &['D'],
    &['Đ'],
    &['E'],
    &['Ž'],
    &['Z'],
    &['I'],
    &['J'],
    &['K'],
    &['L'],
    &['Ǉ'], //
    &['ǈ'], //
    &['M'],
    &['N'],
    &['Ǌ'], //
    &['ǋ'], //
    &['O'],
    &['P'],
    &['R'],
    &['S'],
    &['T'],
    &['Ć'],
    &['U'],
    &['F'],
    &['H'],
    &['C'],
    &['Č'],
    &['Ǆ'], //
    &['ǅ'], //
    &['Š'],
    &['a'],
    &['b'],
    &['v'],
    &['g'],
    &['d'],
    &['đ'],
    &['e'],
    &['ž'],
    &['z'],
    &['i'],
    &['j'],
    &['k'],
    &['l'],
    &['ǉ'],
    &['m'],
    &['n'],
    &['ǌ'],
    &['o'],
    &['p'],
    &['r'],
    &['s'],
    &['t'],
    &['ć'],
    &['u'],
    &['f'],
    &['h'],
    &['c'],
    &['č'],
    &['ǆ'],
    &['š'],
];

pub const CYRILLIC_CLEAN: &[&[char]] = &[
    &['А'],
    &['Б'],
    &['В'],
    &['Г'],
    &['Д'],
    &['Ђ'],
    &['Е'],
    &['Ж'],
    &['З'],
    &['И'],
    &['Ј'],
    &['К'],
    &['Л'],
    &['Љ'], //
    &['Љ'], //
    &['М'],
    &['Н'],
    &['Њ'], //
    &['Њ'], //
    &['О'],
    &['П'],
    &['Р'],
    &['С'],
    &['Т'],
    &['Ћ'],
    &['У'],
    &['Ф'],
    &['Х'],
    &['Ц'],
    &['Ч'],
    &['Џ'], //
    &['Џ'], //
    &['Ш'],
    &['а'],
    &['б'],
    &['в'],
    &['г'],
    &['д'],
    &['ђ'],
    &['е'],
    &['ж'],
    &['з'],
    &['и'],
    &['ј'],
    &['к'],
    &['л'],
    &['љ'],
    &['м'],
    &['н'],
    &['њ'],
    &['о'],
    &['п'],
    &['р'],
    &['с'],
    &['т'],
    &['ћ'],
    &['у'],
    &['ф'],
    &['х'],
    &['ц'],
    &['ч'],
    &['џ'],
    &['ш'],
];

pub const LATIN_DIRTY: &[&[char]] = &[
    &['A'],
    &['B'],
    &['V'],
    &['G'],
    &['D'],
    &['Đ'],
    &['Ð'],
    &['D', 'J'],
    &['D', 'j'],
    &['E'],
    &['Z'],
    &['Ž'],
    &['\u{17d}'],
    &['I'],
    &['J'],
    &['K'],
    &['L'],
    &['L', 'J'],
    &['Ǉ'],
    &['L', 'j'],
    &['ǈ'],
    &['M'],
    &['N'],
    &['N', 'J'],
    &['Ǌ'],
    &['N', 'j'],
    &['ǋ'],
    &['O'],
    &['P'],
    &['R'],
    &['S'],
    &['T'],
    &['Ć'],
    &['\u{106}'],
    &['U'],
    &['F'],
    &['H'],
    &['C'],
    &['Č'],
    &['\u{10c}'],
    &['D', 'Ž'],
    &['Ǆ'],
    &['D', '\u{17d}'],
    &['D', 'ž'],
    &['ǅ'],
    &['D', '\u{17e}'],
    &['Š'],
    &['\u{160}'],
    &['a'],
    &['æ'],
    &['b'],
    &['v'],
    &['g'],
    &['d'],
    &['đ'],
    &['d', 'j'],
    &['e'],
    &['z'],
    &['ž'],
    &['\u{17e}'],
    &['i'],
    &['ĳ'],
    &['j'],
    &['k'],
    &['l'],
    &['l', 'j'],
    &['ǉ'],
    &['m'],
    &['n'],
    &['n', 'j'],
    &['ǌ'],
    &['o'],
    &['œ'],
    &['p'],
    &['r'],
    &['s'],
    &['ﬆ'],
    &['t'],
    &['ć'],
    &['\u{107}'],
    &['u'],
    &['f'],
    &['ﬁ'],
    &['ﬂ'],
    &['h'],
    &['c'],
    &['č'],
    &['\u{10d}'],
    &['d', 'ž'],
    &['ǆ'],
    &['d', '\u{17e}'],
    &['š'],
    &['\u{161}'],
];

pub const LATIN_DIRTY_UNICODE: &[&[char]] = &[
    &['A'],
    &['B'],
    &['V'],
    &['G'],
    &['D'],
    &['Đ'],
    &['Đ'],
    &['Đ'],
    &['Đ'],
    &['E'],
    &['Z'],
    &['Ž'],
    &['Ž'],
    &['I'],
    &['J'],
    &['K'],
    &['L'],
    &['Ǉ'],
    &['Ǉ'],
    &['ǈ'],
    &['ǈ'],
    &['M'],
    &['N'],
    &['Ǌ'],
    &['Ǌ'],
    &['ǋ'],
    &['ǋ'],
    &['O'],
    &['P'],
    &['R'],
    &['S'],
    &['T'],
    &['Ć'],
    &['Ć'],
    &['U'],
    &['F'],
    &['H'],
    &['C'],
    &['Č'],
    &['Č'],
    &['Ǆ'],
    &['Ǆ'],
    &['Ǆ'],
    &['ǅ'],
    &['ǅ'],
    &['ǅ'],
    &['Š'],
    &['Š'],
    &['a'],
    &['a', 'e'],
    &['b'],
    &['v'],
    &['g'],
    &['d'],
    &['đ'],
    &['đ'],
    &['e'],
    &['z'],
    &['ž'],
    &['ž'],
    &['i'],
    &['i', 'j'],
    &['j'],
    &['k'],
    &['l'],
    &['ǉ'],
    &['ǉ'],
    &['m'],
    &['n'],
    &['ǌ'],
    &['ǌ'],
    &['o'],
    &['o', 'e'],
    &['p'],
    &['r'],
    &['s'],
    &['š', 't'],
    &['t'],
    &['ć'],
    &['ć'],
    &['u'],
    &['f'],
    &['f', 'i'],
    &['f', 'l'],
    &['h'],
    &['c'],
    &['č'],
    &['č'],
    &['ǆ'],
    &['ǆ'],
    &['ǆ'],
    &['š'],
    &['š'],
];

pub const CYRILLIC_DIRTY: &[&[char]] = &[
    &['А'],
    &['Б'],
    &['В'],
    &['Г'],
    &['Д'],
    &['Ђ'],
    &['Ђ'],
    &['Ђ'],
    &['Ђ'],
    &['Е'],
    &['З'],
    &['Ж'],
    &['Ж'],
    &['И'],
    &['Ј'],
    &['К'],
    &['Л'],
    &['Љ'],
    &['Љ'],
    &['Љ'],
    &['Љ'],
    &['М'],
    &['Н'],
    &['Њ'],
    &['Њ'],
    &['Њ'],
    &['Њ'],
    &['О'],
    &['П'],
    &['Р'],
    &['С'],
    &['Т'],
    &['Ћ'],
    &['Ћ'],
    &['У'],
    &['Ф'],
    &['Х'],
    &['Ц'],
    &['Ч'],
    &['Ч'],
    &['Џ'],
    &['Џ'],
    &['Џ'],
    &['Џ'],
    &['Џ'],
    &['Џ'],
    &['Ш'],
    &['Ш'],
    &['а'],
    &['а', 'е'],
    &['б'],
    &['в'],
    &['г'],
    &['д'],
    &['ђ'],
    &['ђ'],
    &['е'],
    &['з'],
    &['ж'],
    &['ж'],
    &['и'],
    &['и', 'ј'],
    &['ј'],
    &['к'],
    &['л'],
    &['љ'],
    &['љ'],
    &['м'],
    &['н'],
    &['њ'],
    &['њ'],
    &['о'],
    &['о', 'е'],
    &['п'],
    &['р'],
    &['с'],
    &['с', 'т'],
    &['т'],
    &['ћ'],
    &['ћ'],
    &['у'],
    &['ф'],
    &['ф', 'и'],
    &['ф', 'л'],
    &['х'],
    &['ц'],
    &['ч'],
    &['ч'],
    &['џ'],
    &['џ'],
    &['џ'],
    &['ш'],
    &['ш'],
];

pub struct DigraphException<'a> {
    pub latin: &'a [&'a [char]],
    pub cyrillic: &'a [&'a [char]],
    pub exceptions: &'a [&'a str],
}

pub const DIGRAPH_EXCEPTIONS: &[DigraphException<'static>] = &[
    DigraphException {
        latin: &[
            &['Đ'],
            &['Ð'],
            &['D', 'J'],
            &['D', 'j'],
            &['đ'],
            &['d', 'j'],
        ],
        cyrillic: &[
            &['Д', 'ј'],
            &['Д', 'ј'],
            &['Д', 'Ј'],
            &['Д', 'ј'],
            &['д', 'ј'],
            &['д', 'ј'],
        ],
        exceptions: DIGRAPH_EXCEPTIONS_DJ,
    },
    DigraphException {
        latin: &[
            &['D', 'Ž'],
            &['Ǆ'],
            &['D', '\u{17d}'],
            &['D', 'ž'],
            &['ǅ'],
            &['D', '\u{17e}'],
            &['d', 'ž'],
            &['ǆ'],
        ],
        cyrillic: &[
            &['Д', 'Ж'],
            &['Д', 'Ж'],
            &['Д', 'Ж'],
            &['Д', 'ж'],
            &['Д', 'ж'],
            &['Д', 'ж'],
            &['д', 'ж'],
            &['д', 'ж'],
        ],
        exceptions: DIGRAPH_EXCEPTIONS_DZ,
    },
    DigraphException {
        latin: &[
            &['N', 'J'],
            &['Ǌ'],
            &['N', 'j'],
            &['ǋ'],
            &['n', 'j'],
            &['ǌ'],
        ],
        cyrillic: &[
            &['Н', 'Ј'],
            &['Н', 'Ј'],
            &['Н', 'ј'],
            &['Н', 'ј'],
            &['н', 'ј'],
            &['н', 'ј'],
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
