use phf::phf_map;

#[allow(dead_code)]
pub const SPACES: &'static [char] = &[
    '\u{0020}', '\u{00A0}', '\u{1680}', '\u{180E}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}',
    '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}', '\u{200A}', '\u{200B}',
    '\u{202F}', '\u{205F}', '\u{3000}', '\u{FEFF}',
];

// Borrowed from Jovan Turanjanin's Ćirilizator
// https://github.com/turanjanin/cirilizator

#[allow(dead_code)]
pub const LATIN: &'static [&'static str] = &[
    "A", "B", "V", "G", "D", "Đ", "Ð", "DJ", "Dj", "E", "Ž", "Ž", "Z", "I", "J", "K", "L", "LJ",
    "Ǉ", "Lj", "ǈ", "M", "N", "NJ", "Ǌ", "Nj", "ǋ", "O", "P", "R", "S", "T", "Ć", "Ć", "U", "F",
    "H", "C", "Č", "Č", "DŽ", "Ǆ", "DŽ", "Dž", "ǅ", "Dž", "Š", "Š", "a", "æ", "b", "v", "g", "d",
    "đ", "dj", "e", "ž", "ž", "z", "i", "ĳ", "j", "k", "l", "lj", "ǉ", "m", "n", "nj", "ǌ", "o",
    "œ", "p", "r", "s", "ﬆ", "t", "ć", "ć", "u", "f", "ﬁ", "ﬂ", "h", "c", "č", "č", "dž", "ǆ",
    "dž", "š", "š",
];

#[allow(dead_code)]
pub const CYRILLIC: &'static [&'static str] = &[
    "А", "Б", "В", "Г", "Д", "Ђ", "Ђ", "Ђ", "Ђ", "Е", "Ж", "Ж", "З", "И", "Ј", "К", "Л", "Љ", "Љ",
    "Љ", "Љ", "М", "Н", "Њ", "Њ", "Њ", "Њ", "О", "П", "Р", "С", "Т", "Ћ", "Ћ", "У", "Ф", "Х", "Ц",
    "Ч", "Ч", "Џ", "Џ", "Џ", "Џ", "Џ", "Џ", "Ш", "Ш", "а", "ае", "б", "в", "г", "д", "ђ", "ђ", "е",
    "ж", "ж", "з", "и", "иј", "ј", "к", "л", "љ", "љ", "м", "н", "њ", "њ", "о", "ое", "п", "р",
    "с", "ст", "т", "ћ", "ћ", "у", "ф", "фи", "фл", "х", "ц", "ч", "ч", "џ", "џ", "џ", "ш", "ш",
];

#[allow(dead_code)]
pub const DIAGRAPH_EXCEPTIONS_LETTERS: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    "dj" => &[
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
        "šukundjed"
    ],
    "dž" => &[
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
        "predživot"
    ],
    "nj" => &[
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
        "vanjezičk"
    ],
};
