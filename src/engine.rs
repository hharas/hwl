// بسم الله الرحمن الرحيم

pub fn transliterate(source: &str) -> String {
    source
        // 1, Replace all common combinations
            .replace("الله", "allāh")

            .replace("بالله", "billāh")
            .replace("بِالله", "billāh")

            .replace("تالله", "tallāh")

            .replace("في الله", "fillāh")
            .replace("فِي الله", "fillāh")

            .replace("َ ال", "-al-")
            .replace("ُ ال", "-ul-")
            .replace("ِ ال", "-il-")
            
            .replace(" ال", "al-")
            .replace(" اْل", "al-")
            .replace(" الْ", "al-")
            .replace(" اْلْ", "al-")

            .replace("ًا", "an")

            .replace("يا", "yā")
            .replace("يَا", "yā")
            .replace("ياْ", "yā")
            .replace("يَاْ", "yā")

            .replace("َا", "ā")
            .replace("َي", "ay")
            .replace("ُي", "uy")
            .replace("ِي", "ī")
            .replace("َو", "aw")
            .replace("ُو", "ū")

        // 2. Replace all pieces of Tashkeel
            // 2.1. Replace Harakat
                // 2.1.1. Replace all Fatha's
                    .replace('أ', "ʾa")
                    .replace("أَ", "ʾa")
                    .replace("بَ", "ba")
                    .replace("تَ", "ta")
                    .replace("ثَ", "tha")
                    .replace("جَ", "ja")
                    .replace("حَ", "ḥa")
                    .replace("خَ", "kha")
                    .replace("دَ", "da")
                    .replace("ذَ", "dha")
                    .replace("رَ", "ra")
                    .replace("زَ", "za")
                    .replace("سَ", "sa")
                    .replace("شَ", "sha")
                    .replace("صَ", "ṣa")
                    .replace("ضَ", "ḍa")
                    .replace("طَ", "ṭa")
                    .replace("ظَ", "ẓa")
                    .replace("عَ", "ʿa")
                    .replace("غَ", "gha")
                    .replace("فَ", "fa")
                    .replace("قَ", "qa")
                    .replace("كَ", "ka")
                    .replace("لَ", "la")
                    .replace("مَ", "ma")
                    .replace("نَ", "na")
                    .replace("هَ", "ha")
                    .replace("وَ", "wa")
                    .replace("يَ", "ya")

                // 2.1.2. Replace all Dhammah's
                    .replace("أُ", "ʾu")
                    .replace("بُ", "bu")
                    .replace("تُ", "tu")
                    .replace("ثُ", "thu")
                    .replace("جُ", "ju")
                    .replace("حُ", "ḥu")
                    .replace("خُ", "khu")
                    .replace("دُ", "du")
                    .replace("ذُ", "dhu")
                    .replace("رُ", "ru")
                    .replace("زُ", "zu")
                    .replace("سُ", "su")
                    .replace("شُ", "shu")
                    .replace("صُ", "ṣu")
                    .replace("ضُ", "ḍu")
                    .replace("طُ", "ṭu")
                    .replace("ظِ", "ẓu")
                    .replace("عُ", "ʿu")
                    .replace("غُ", "ghu")
                    .replace("فُ", "fu")
                    .replace("قُ", "qu")
                    .replace("كُ", "ku")
                    .replace("لُ", "lu")
                    .replace("مُ", "mu")
                    .replace("نُ", "nu")
                    .replace("هُ", "hu")
                    .replace("وُ", "wu")
                    .replace("يُ", "yu")

                // 2.1.3. Replace all Kasrah's
                    .replace("إِ", "ʾi")
                    .replace('إ', "ʾi")
                    .replace("بِ", "bi")
                    .replace("تِ", "ti")
                    .replace("ثِ", "thi")
                    .replace("جِ", "ji")
                    .replace("حِ", "ḥi")
                    .replace("خِ", "khi")
                    .replace("دِ", "di")
                    .replace("ذِ", "dhi")
                    .replace("رِ", "ri")
                    .replace("زِ", "zi")
                    .replace("سِ", "si")
                    .replace("شِ", "shi")
                    .replace("صِ", "ṣi")
                    .replace("ضِ", "ḍi")
                    .replace("طِ", "ṭi")
                    .replace("ظِ", "ẓi")
                    .replace("عِ", "ʿi")
                    .replace("غِ", "ghi")
                    .replace("فِ", "fi")
                    .replace("قِ", "qi")
                    .replace("كِ", "ki")
                    .replace("لِ", "li")
                    .replace("مِ", "mi")
                    .replace("نِ", "ni")
                    .replace("هِ", "hi")
                    .replace("وِ", "wi")
                    .replace("يِ", "yi")

                // 2.1.4. Replace all Sukoon's
                    .replace("أْ", "ʾ")
                    .replace("بْ", "b")
                    .replace("تْ", "t")
                    .replace("ثْ", "th")
                    .replace("جْ", "j")
                    .replace("حْ", "ḥ")
                    .replace("خْ", "kh")
                    .replace("دْ", "d")
                    .replace("ذْ", "dh")
                    .replace("رْ", "r")
                    .replace("زْ", "z")
                    .replace("سْ", "s")
                    .replace("شْ", "sh")
                    .replace("صْ", "ṣ")
                    .replace("ضْ", "ḍ")
                    .replace("طْ", "ṭ")
                    .replace("ظْ", "ẓ")
                    .replace("عْ", "ʿ")
                    .replace("غْ", "gh")
                    .replace("فْ", "f")
                    .replace("قْ", "q")
                    .replace("كْ", "k")
                    .replace("لْ", "l")
                    .replace("مْ", "m")
                    .replace("نْ", "n")
                    .replace("هْ", "h")
                    .replace("وْ", "w")
                    .replace("يْ", "ī")

            // 2.2. Replace all Tanweenat
                // 2.2.1. Replace all Tanween Fath's
                    .replace("ىً", "an")

                // 2.2.2. Replace all Tanween Dhamm's
                    .replace("أٌ", "ʾun")
                    .replace("بٌ", "bun")
                    .replace("تٌ", "tun")
                    .replace("ثٌ", "thun")
                    .replace("جٌ", "jun")
                    .replace("حٌ", "ḥun")
                    .replace("خٌ", "khun")
                    .replace("دٌ", "dun")
                    .replace("ذٌ", "dhun")
                    .replace("رٌ", "run")
                    .replace("زٌ", "zun")
                    .replace("سٌ", "sun")
                    .replace("شٌ", "shun")
                    .replace("صٌ", "ṣun")
                    .replace("ضٌ", "ḍun")
                    .replace("طٌ", "ṭun")
                    .replace("ظٌ", "ẓun")
                    .replace("عٌ", "ʿun")
                    .replace("غٌ", "ghun")
                    .replace("فٌ", "fun")
                    .replace("قٌ", "qun")
                    .replace("كٌ", "kun")
                    .replace("لٌ", "lun")
                    .replace("مٌ", "mun")
                    .replace("نٌ", "nun")
                    .replace("هٌ", "hun")
                    .replace("وٌ", "wun")
                    .replace("يٌ", "yun")

                // 2.2.3. Replace all Tanween Kasr's
                    .replace("بٍ", "bin")
                    .replace("تٍ", "tin")
                    .replace("ثٍ", "thin")
                    .replace("جٍ", "jin")
                    .replace("حٍ", "ḥin")
                    .replace("خٍ", "khin")
                    .replace("دٍ", "din")
                    .replace("ذٍ", "dhin")
                    .replace("رٍ", "rin")
                    .replace("زٍ", "zin")
                    .replace("سٍ", "sin")
                    .replace("شٍ", "shin")
                    .replace("صٍ", "ṣin")
                    .replace("ضٍ", "ḍin")
                    .replace("طٍ", "ṭin")
                    .replace("ظٍ", "ẓin")
                    .replace("عٍ", "ʿin")
                    .replace("غٍ", "ghin")
                    .replace("فٍ", "fin")
                    .replace("قٍ", "qin")
                    .replace("كٍ", "kin")
                    .replace("لٍ", "lin")
                    .replace("مٍ", "min")
                    .replace("نٍ", "nin")
                    .replace("هٍ", "hin")
                    .replace("وٍ", "win")
                    .replace("يٍ", "yin")

                // 2.3.1 Replace all Shadd Fath's
                    .replace("بَّ", "bba")
                    .replace("تَّ", "tta")
                    .replace("ثَّ", "tha")
                    .replace("جَّ", "jja")
                    .replace("حَّ", "ḥḥa")
                    .replace("خَّ", "kha")
                    .replace("دَّ", "dda")
                    .replace("ذَّ", "dha")
                    .replace("رَّ", "rra")
                    .replace("زَّ", "zza")
                    .replace("سَّ", "ssa")
                    .replace("شَّ", "sha")
                    .replace("صَّ", "ṣṣa")
                    .replace("ضَّ", "ḍḍa")
                    .replace("طَّ", "ṭṭa")
                    .replace("ظَّ", "ẓẓa")
                    .replace("عَّ", "ʿʿa")
                    .replace("غَّ", "gha")
                    .replace("فَّ", "ffa")
                    .replace("قَّ", "qqa")
                    .replace("كَّ", "kka")
                    .replace("لَّ", "lla")
                    .replace("مَّ", "mma")
                    .replace("نَّ", "nna")
                    .replace("هَّ", "hha")
                    .replace("وَّ", "wwa")
                    .replace("يَّ", "yya")

                // 2.3.2 Replace all Shadd Dhamm's
                    .replace("بُّ", "bbu")
                    .replace("تُّ", "ttu")
                    .replace("ثُّ", "thu")
                    .replace("جُّ", "jju")
                    .replace("حُّ", "ḥḥu")
                    .replace("خُّ", "khu")
                    .replace("دُّ", "ddu")
                    .replace("ذُّ", "dhu")
                    .replace("رُّ", "rru")
                    .replace("زُّ", "zzu")
                    .replace("سُّ", "ssu")
                    .replace("شُّ", "shu")
                    .replace("صُّ", "ṣṣu")
                    .replace("ضُّ", "ḍḍu")
                    .replace("طُّ", "ṭṭu")
                    .replace("ظُّ", "ẓẓu")
                    .replace("عُّ", "ʿʿu")
                    .replace("غُّ", "ghu")
                    .replace("فُّ", "ffu")
                    .replace("قُّ", "qqu")
                    .replace("كُّ", "kku")
                    .replace("لُّ", "llu")
                    .replace("مُّ", "mmu")
                    .replace("نُّ", "nnu")
                    .replace("هُّ", "hhu")
                    .replace("وُّ", "wwu")
                    .replace("يُّ", "yyu")

                // 2.3.3 Replace all Shadd Kasr's
                    .replace("بِّ", "bbi")
                    .replace("تِّ", "tti")
                    .replace("ثِّ", "thi")
                    .replace("جِّ", "jji")
                    .replace("حِّ", "ḥḥi")
                    .replace("خِّ", "khi")
                    .replace("دِّ", "ddi")
                    .replace("ذِّ", "dhi")
                    .replace("رِّ", "rri")
                    .replace("زِّ", "zzi")
                    .replace("سِّ", "ssi")
                    .replace("شِّ", "shi")
                    .replace("صِّ", "ṣṣi")
                    .replace("ضِّ", "ḍḍi")
                    .replace("طِّ", "ṭṭi")
                    .replace("ظِّ", "ẓẓi")
                    .replace("عِّ", "ʿʿi")
                    .replace("غِّ", "ghi")
                    .replace("فِّ", "ffi")
                    .replace("قِّ", "qqi")
                    .replace("كِّ", "kki")
                    .replace("لِّ", "lli")
                    .replace("مِّ", "mmi")
                    .replace("نِّ", "nni")
                    .replace("هِّ", "hhi")
                    .replace("وِّ", "wwi")
                    .replace("يِّ", "yyi")

        // 3. Replace all clean letters
            .replace('ا', "ā")
            .replace('ب', "b")
            .replace('ت', "t")
            .replace('ث', "th")
            .replace('ج', "j")
            .replace('ح', "ḥ")
            .replace('خ', "kh")
            .replace('د', "d")
            .replace('ذ', "dh")
            .replace('ر', "r")
            .replace('ز', "z")
            .replace('س', "s")
            .replace('ش', "sh")
            .replace('ص', "ṣ")
            .replace('ض', "ḍ")
            .replace('ط', "ṭ")
            .replace('ظ', "ẓ")
            .replace('ع', "ʿ")
            .replace('غ', "gh")
            .replace('ف', "f")
            .replace('ق', "q")
            .replace('ك', "k")
            .replace('ل', "l")
            .replace('م', "m")
            .replace('ن', "n")
            .replace('ه', "h")
            .replace('و', "w")
            .replace('ي', "ī")
            .replace('ى', "ā")

        // 4. Replace all symbols
            .replace('؟', "?")
            .replace('،', ",")
            .replace('؛', ";")
}

// تم بحمد الله