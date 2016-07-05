use std::collections::HashMap;
use std::path::PathBuf;

pub enum Fixture {
    One,
    #[allow(dead_code)]
    Two,
}

impl Fixture {
    pub fn path(&self) -> PathBuf {
        match *self {
            Fixture::One => "tests/fixtures/SourceSerifPro-Regular.otf",
            Fixture::Two => "tests/fixtures/OpenSans-Italic.ttf",
        }.into()
    }
}

pub fn mapping() -> HashMap<u16, u16> {
    macro_rules! map(
        ($($key:expr => $value:expr,)*) => ({
            let mut map = HashMap::new();
            $(map.insert($key, $value);)*;
            map
        });
    );

    map![
        32 => 1,
        33 => 262,
        34 => 269,
        35 => 326,
        36 => 419,
        37 => 434,
        38 => 214,
        39 => 268,
        40 => 289,
        41 => 290,
        42 => 314,
        43 => 445,
        44 => 258,
        45 => 280,
        46 => 257,
        47 => 310,
        48 => 215,
        49 => 216,
        50 => 217,
        51 => 218,
        52 => 219,
        53 => 220,
        54 => 221,
        55 => 222,
        56 => 223,
        57 => 224,
        58 => 259,
        59 => 260,
        60 => 451,
        61 => 450,
        62 => 452,
        63 => 265,
        64 => 324,
        65 => 2,
        66 => 3,
        67 => 4,
        68 => 5,
        69 => 6,
        70 => 7,
        71 => 8,
        72 => 9,
        73 => 10,
        74 => 11,
        75 => 12,
        76 => 13,
        77 => 14,
        78 => 15,
        79 => 16,
        80 => 17,
        81 => 18,
        82 => 19,
        83 => 20,
        84 => 21,
        85 => 22,
        86 => 23,
        87 => 24,
        88 => 25,
        89 => 26,
        90 => 27,
        91 => 291,
        92 => 312,
        93 => 292,
        94 => 456,
        95 => 288,
        96 => 502,
        97 => 28,
        98 => 29,
        99 => 30,
        100 => 31,
        101 => 32,
        102 => 33,
        103 => 34,
        104 => 35,
        105 => 36,
        106 => 37,
        107 => 38,
        108 => 39,
        109 => 40,
        110 => 41,
        111 => 42,
        112 => 43,
        113 => 44,
        114 => 45,
        115 => 46,
        116 => 47,
        117 => 48,
        118 => 49,
        119 => 50,
        120 => 51,
        121 => 52,
        122 => 53,
        123 => 293,
        124 => 311,
        125 => 294,
        126 => 458,
        160 => 430,
        161 => 263,
        162 => 423,
        163 => 420,
        164 => 418,
        165 => 421,
        166 => 313,
        167 => 317,
        168 => 510,
        169 => 319,
        170 => 387,
        171 => 278,
        172 => 460,
        173 => 281,
        174 => 321,
        175 => 511,
        176 => 417,
        177 => 455,
        178 => 329,
        179 => 330,
        180 => 503,
        181 => 463,
        182 => 318,
        183 => 286,
        184 => 516,
        185 => 328,
        186 => 388,
        187 => 279,
        188 => 436,
        189 => 437,
        190 => 438,
        191 => 266,
        192 => 54,
        193 => 55,
        194 => 56,
        195 => 57,
        196 => 58,
        197 => 61,
        198 => 63,
        199 => 64,
        200 => 69,
        201 => 70,
        202 => 71,
        203 => 73,
        204 => 80,
        205 => 81,
        206 => 82,
        207 => 83,
        208 => 129,
        209 => 94,
        210 => 97,
        211 => 98,
        212 => 99,
        213 => 100,
        214 => 101,
        215 => 447,
        216 => 104,
        217 => 116,
        218 => 117,
        219 => 118,
        220 => 119,
        221 => 124,
        222 => 130,
        223 => 187,
        224 => 131,
        225 => 132,
        226 => 133,
        227 => 134,
        228 => 135,
        229 => 138,
        230 => 140,
        231 => 141,
        232 => 146,
        233 => 147,
        234 => 148,
        235 => 150,
        236 => 157,
        237 => 158,
        238 => 159,
        239 => 160,
        240 => 209,
        241 => 172,
        242 => 175,
        243 => 176,
        244 => 177,
        245 => 178,
        246 => 179,
        247 => 448,
        248 => 182,
        249 => 196,
        250 => 197,
        251 => 198,
        252 => 199,
        253 => 204,
        254 => 210,
        255 => 205,
        256 => 59,
        257 => 136,
        258 => 60,
        259 => 137,
        260 => 62,
        261 => 139,
        262 => 65,
        263 => 142,
        268 => 66,
        269 => 143,
        270 => 67,
        271 => 144,
        272 => 68,
        273 => 145,
        274 => 74,
        275 => 151,
        278 => 75,
        279 => 152,
        280 => 76,
        281 => 153,
        282 => 72,
        283 => 149,
        286 => 77,
        287 => 154,
        288 => 78,
        289 => 155,
        290 => 79,
        291 => 156,
        298 => 84,
        299 => 161,
        302 => 86,
        303 => 162,
        304 => 85,
        305 => 164,
        310 => 87,
        311 => 165,
        313 => 88,
        314 => 166,
        315 => 545,
        316 => 546,
        317 => 89,
        318 => 167,
        319 => 90,
        320 => 168,
        321 => 91,
        322 => 169,
        323 => 92,
        324 => 170,
        325 => 95,
        326 => 173,
        327 => 93,
        328 => 171,
        332 => 102,
        333 => 180,
        336 => 103,
        337 => 181,
        338 => 105,
        339 => 183,
        340 => 106,
        341 => 184,
        342 => 108,
        343 => 185,
        344 => 107,
        345 => 186,
        346 => 109,
        347 => 189,
        350 => 111,
        351 => 191,
        352 => 110,
        353 => 190,
        354 => 114,
        355 => 194,
        356 => 113,
        357 => 193,
        362 => 120,
        363 => 200,
        366 => 121,
        367 => 201,
        368 => 122,
        369 => 202,
        370 => 123,
        371 => 203,
        376 => 125,
        377 => 126,
        378 => 206,
        379 => 128,
        380 => 208,
        381 => 127,
        382 => 207,
        383 => 188,
        402 => 424,
        536 => 112,
        537 => 192,
        538 => 115,
        539 => 195,
        688 => 396,
        690 => 398,
        691 => 406,
        695 => 411,
        696 => 413,
        699 => 500,
        700 => 501,
        710 => 504,
        711 => 505,
        713 => 506,
        714 => 507,
        715 => 508,
        728 => 512,
        729 => 515,
        730 => 513,
        731 => 519,
        732 => 509,
        733 => 514,
        737 => 400,
        738 => 407,
        739 => 412,
        768 => 520,
        769 => 522,
        770 => 524,
        771 => 526,
        772 => 528,
        774 => 530,
        775 => 532,
        776 => 534,
        778 => 536,
        779 => 538,
        780 => 540,
        806 => 542,
        807 => 543,
        808 => 544,
        960 => 461,
        7491 => 389,
        7495 => 390,
        7496 => 392,
        7497 => 393,
        7501 => 395,
        7503 => 399,
        7504 => 401,
        7506 => 403,
        7510 => 404,
        7511 => 408,
        7512 => 409,
        7515 => 410,
        7580 => 391,
        7584 => 394,
        7611 => 414,
        7748 => 96,
        7749 => 174,
        8199 => 431,
        8210 => 284,
        8211 => 282,
        8212 => 283,
        8213 => 285,
        8216 => 270,
        8217 => 271,
        8218 => 274,
        8220 => 272,
        8221 => 273,
        8222 => 275,
        8224 => 315,
        8225 => 316,
        8226 => 287,
        8230 => 261,
        8240 => 435,
        8242 => 498,
        8243 => 499,
        8249 => 276,
        8250 => 277,
        8260 => 427,
        8304 => 327,
        8305 => 397,
        8308 => 331,
        8309 => 332,
        8310 => 333,
        8311 => 334,
        8312 => 335,
        8313 => 336,
        8317 => 337,
        8318 => 338,
        8319 => 402,
        8320 => 345,
        8321 => 346,
        8322 => 347,
        8323 => 348,
        8324 => 349,
        8325 => 350,
        8326 => 351,
        8327 => 352,
        8328 => 353,
        8329 => 354,
        8333 => 355,
        8334 => 356,
        8364 => 422,
        8377 => 425,
        8378 => 426,
        8467 => 471,
        8471 => 320,
        8480 => 323,
        8482 => 322,
        8486 => 468,
        8494 => 472,
        8531 => 439,
        8532 => 440,
        8539 => 441,
        8540 => 442,
        8541 => 443,
        8542 => 444,
        8592 => 473,
        8593 => 475,
        8594 => 477,
        8595 => 479,
        8598 => 474,
        8599 => 476,
        8600 => 478,
        8601 => 480,
        8706 => 464,
        8710 => 467,
        8719 => 470,
        8721 => 469,
        8722 => 446,
        8725 => 428,
        8729 => 449,
        8730 => 466,
        8734 => 462,
        8747 => 465,
        8776 => 459,
        8800 => 457,
        8804 => 453,
        8805 => 454,
        9632 => 481,
        9650 => 485,
        9651 => 486,
        9654 => 487,
        9655 => 488,
        9660 => 489,
        9661 => 490,
        9664 => 491,
        9665 => 492,
        9670 => 482,
        9673 => 483,
        9674 => 497,
        9744 => 493,
        9745 => 494,
        9834 => 496,
        10003 => 495,
        10066 => 484,
        64257 => 211,
        64258 => 212,
    ]
}
