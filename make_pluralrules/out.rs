use super::operands::PluralOperands;
use super::PluralCategory;
type PluralRule = fn(PluralOperands) -> PluralCategory;
pub fn get_pr(lang: &str) -> PluralRule {
    match lang {
        "ru" => |po| {
            if (po.v == 0 && (po.i % 10 == 2..=4) && (po.i % 100 != 12..=14)) {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11) {
                PluralCategory::ONE
            } else if (po.v == 0 && po.i % 10 == 0) || (po.v == 0 && (po.i % 10 == 5..=9))
                || (po.v == 0 && (po.i % 100 == 11..=14))
            {
                PluralCategory::MANY
            } else {
                PluralCategory::OTHER
            }
        },
        "tr" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "lo" => |po| PluralCategory::OTHER,
        "jw" => |po| PluralCategory::OTHER,
        "sn" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kde" => |po| PluralCategory::OTHER,
        "guw" => |po| {
            if (po.n == 0..=1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "en" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "hr" => |po| {
            if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else if (po.v == 0 && (po.i % 10 == 2..=4) && (po.i % 100 != 12..=14))
                || ((po.f % 10 == 2..=4) && (po.f % 100 != 12..=14))
            {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "lt" => |po| {
            if ((po.n % 10 == 2..=9) && (po.n % 100 != 11..=19)) {
                PluralCategory::FEW
            } else if (po.n % 10 == 1 && (po.n % 100 != 11..=19)) {
                PluralCategory::ONE
            } else if (po.f != 0) {
                PluralCategory::MANY
            } else {
                PluralCategory::OTHER
            }
        },
        "fa" => |po| {
            if (po.i == 0) || (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "cs" => |po| {
            if ((po.i == 2..=4) && po.v == 0) {
                PluralCategory::FEW
            } else if (po.v != 0) {
                PluralCategory::MANY
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "or" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "lb" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "af" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "gl" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ksh" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else if (po.n == 0) {
                PluralCategory::ZERO
            } else {
                PluralCategory::OTHER
            }
        },
        "ml" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "tk" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ka" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "uk" => |po| {
            if (po.v == 0 && po.i % 10 == 0) || (po.v == 0 && (po.i % 10 == 5..=9))
                || (po.v == 0 && (po.i % 100 == 11..=14))
            {
                PluralCategory::MANY
            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11) {
                PluralCategory::ONE
            } else if (po.v == 0 && (po.i % 10 == 2..=4) && (po.i % 100 != 12..=14)) {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "ii" => |po| PluralCategory::OTHER,
        "saq" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "pap" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nso" => |po| {
            if (po.n == 0..=1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kw" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else if (po.n == 2) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "bh" => |po| {
            if (po.n == 0..=1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "lg" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "iu" => |po| {
            if (po.n == 2) {
                PluralCategory::TWO
            } else if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "io" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kn" => |po| {
            if (po.i == 0) || (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kcg" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "lv" => |po| {
            if (po.n % 10 == 0) || (po.n % 100 == 11..=19) || (po.v == 2 && (po.f % 100 == 11..=19))
            {
                PluralCategory::ZERO
            } else if (po.n % 10 == 1 && po.n % 100 != 11)
                || (po.v == 2 && po.f % 10 == 1 && po.f % 100 != 11)
                || (po.v != 2 && po.f % 10 == 1)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sdh" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kl" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "id" => |po| PluralCategory::OTHER,
        "ckb" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "es" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ce" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "bem" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "smi" => |po| {
            if (po.n == 2) {
                PluralCategory::TWO
            } else if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mt" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else if (po.n == 0) || (po.n % 100 == 2..=10) {
                PluralCategory::FEW
            } else if (po.n % 100 == 11..=19) {
                PluralCategory::MANY
            } else {
                PluralCategory::OTHER
            }
        },
        "zu" => |po| {
            if (po.i == 0) || (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mg" => |po| {
            if (po.n == 0..=1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ga" => |po| {
            if (po.n == 3..=6) {
                PluralCategory::FEW
            } else if (po.n == 2) {
                PluralCategory::TWO
            } else if (po.n == 7..=10) {
                PluralCategory::MANY
            } else if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ja" => |po| PluralCategory::OTHER,
        "ig" => |po| PluralCategory::OTHER,
        "sq" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kea" => |po| PluralCategory::OTHER,
        "nqo" => |po| PluralCategory::OTHER,
        "sah" => |po| PluralCategory::OTHER,
        "os" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sg" => |po| PluralCategory::OTHER,
        "eo" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "jbo" => |po| PluralCategory::OTHER,
        "ve" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ssy" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sma" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else if (po.n == 2) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "th" => |po| PluralCategory::OTHER,
        "shi" => |po| {
            if (po.i == 0) || (po.n == 1) {
                PluralCategory::ONE
            } else if (po.n == 2..=10) {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "ars" => |po| {
            if (po.n % 100 == 3..=10) {
                PluralCategory::FEW
            } else if (po.n % 100 == 11..=99) {
                PluralCategory::MANY
            } else if (po.n == 1) {
                PluralCategory::ONE
            } else if (po.n == 0) {
                PluralCategory::ZERO
            } else if (po.n == 2) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "da" => |po| {
            if (po.n == 1) || (po.t != 0 && (po.i == 0 || po.i == 1)) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "dz" => |po| PluralCategory::OTHER,
        "hi" => |po| {
            if (po.i == 0) || (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "hsb" => |po| {
            if (po.v == 0 && (po.i % 100 == 3..=4)) || (po.f % 100 == 3..=4) {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 100 == 1) || (po.f % 100 == 1) {
                PluralCategory::ONE
            } else if (po.v == 0 && po.i % 100 == 2) || (po.f % 100 == 2) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "de" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "it" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "am" => |po| {
            if (po.i == 0) || (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "xh" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "pt" => |po| {
            if (po.i == 0..=1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ky" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "prg" => |po| {
            if (po.n % 10 == 1 && po.n % 100 != 11)
                || (po.v == 2 && po.f % 10 == 1 && po.f % 100 != 11)
                || (po.v != 2 && po.f % 10 == 1)
            {
                PluralCategory::ONE
            } else if (po.n % 10 == 0) || (po.n % 100 == 11..=19)
                || (po.v == 2 && (po.f % 100 == 11..=19))
            {
                PluralCategory::ZERO
            } else {
                PluralCategory::OTHER
            }
        },
        "bn" => |po| {
            if (po.i == 0) || (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kk" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nb" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "vi" => |po| PluralCategory::OTHER,
        "tig" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "smj" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else if (po.n == 2) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "ln" => |po| {
            if (po.n == 0..=1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "dsb" => |po| {
            if (po.v == 0 && po.i % 100 == 2) || (po.f % 100 == 2) {
                PluralCategory::TWO
            } else if (po.v == 0 && po.i % 100 == 1) || (po.f % 100 == 1) {
                PluralCategory::ONE
            } else if (po.v == 0 && (po.i % 100 == 3..=4)) || (po.f % 100 == 3..=4) {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "hy" => |po| {
            if (po.i == 0 || po.i == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nah" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "fr" => |po| {
            if (po.i == 0 || po.i == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mn" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "chr" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "gsw" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ee" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "eu" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "is" => |po| {
            if (po.t == 0 && po.i % 10 == 1 && po.i % 100 != 11) || (po.t != 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "lag" => |po| {
            if ((po.i == 0 || po.i == 1) && po.n != 0) {
                PluralCategory::ONE
            } else if (po.n == 0) {
                PluralCategory::ZERO
            } else {
                PluralCategory::OTHER
            }
        },
        "ne" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "rwk" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "et" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "km" => |po| PluralCategory::OTHER,
        "bm" => |po| PluralCategory::OTHER,
        "ur" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "te" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "cy" => |po| {
            if (po.n == 0) {
                PluralCategory::ZERO
            } else if (po.n == 3) {
                PluralCategory::FEW
            } else if (po.n == 2) {
                PluralCategory::TWO
            } else if (po.n == 6) {
                PluralCategory::MANY
            } else if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ta" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kaj" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ksb" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "haw" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sms" => |po| {
            if (po.n == 2) {
                PluralCategory::TWO
            } else if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "uz" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "smn" => |po| {
            if (po.n == 2) {
                PluralCategory::TWO
            } else if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mr" => |po| {
            if (po.i == 0) || (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "scn" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "si" => |po| {
            if (po.n == 0 || po.n == 1) || (po.i == 0 && po.f == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "bs" => |po| {
            if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else if (po.v == 0 && (po.i % 10 == 2..=4) && (po.i % 100 != 12..=14))
                || ((po.f % 10 == 2..=4) && (po.f % 100 != 12..=14))
            {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "ti" => |po| {
            if (po.n == 0..=1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ks" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mgo" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "br" => |po| {
            if (po.n != 0 && po.n % 1000000 == 0) {
                PluralCategory::MANY
            } else if (po.n % 10 == 2 && po.n % 100 != 12 && po.n % 100 != 72 && po.n % 100 != 92) {
                PluralCategory::TWO
            } else if (po.n % 10 == 1 && po.n % 100 != 11 && po.n % 100 != 71 && po.n % 100 != 91) {
                PluralCategory::ONE
            } else if ((po.n % 10 == 9 || (po.n % 10 == 3..=4)) && (po.n % 100 != 10..=19)
                && (po.n % 100 != 70..=79) && (po.n % 100 != 90..=99))
            {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "bez" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "syr" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kab" => |po| {
            if (po.i == 0 || po.i == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sr" => |po| {
            if (po.v == 0 && (po.i % 10 == 2..=4) && (po.i % 100 != 12..=14))
                || ((po.f % 10 == 2..=4) && (po.f % 100 != 12..=14))
            {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ast" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nl" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ug" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sk" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else if (po.v != 0) {
                PluralCategory::MANY
            } else if ((po.i == 2..=4) && po.v == 0) {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "kkj" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ts" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ny" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "iw" => |po| {
            if (po.i == 2 && po.v == 0) {
                PluralCategory::TWO
            } else if (po.v == 0 && (po.n != 0..=10) && po.n % 10 == 0) {
                PluralCategory::MANY
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nnh" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "root" => |po| PluralCategory::OTHER,
        "ptPT" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "bg" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "vun" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "yi" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "teo" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sh" => |po| {
            if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else if (po.v == 0 && (po.i % 10 == 2..=4) && (po.i % 100 != 12..=14))
                || ((po.f % 10 == 2..=4) && (po.f % 100 != 12..=14))
            {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "vo" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "pl" => |po| {
            if (po.v == 0 && po.i != 1 && (po.i % 10 == 0..=1))
                || (po.v == 0 && (po.i % 10 == 5..=9))
                || (po.v == 0 && (po.i % 100 == 12..=14))
            {
                PluralCategory::MANY
            } else if (po.v == 0 && (po.i % 10 == 2..=4) && (po.i % 100 != 12..=14)) {
                PluralCategory::FEW
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ps" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "wae" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ro" => |po| {
            if (po.v != 0) || (po.n == 0) || (po.n != 1 && (po.n % 100 == 1..=19)) {
                PluralCategory::FEW
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ms" => |po| PluralCategory::OTHER,
        "ji" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "rm" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "to" => |po| PluralCategory::OTHER,
        "he" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else if (po.v == 0 && (po.n != 0..=10) && po.n % 10 == 0) {
                PluralCategory::MANY
            } else if (po.i == 2 && po.v == 0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "ak" => |po| {
            if (po.n == 0..=1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "pa" => |po| {
            if (po.n == 0..=1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nyn" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "om" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ca" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "fur" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sd" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "fy" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mk" => |po| {
            if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "gu" => |po| {
            if (po.i == 0) || (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "in" => |po| PluralCategory::OTHER,
        "so" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "bo" => |po| PluralCategory::OTHER,
        "fo" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "tl" => |po| {
            if (po.v == 0 && (po.i == 1 || po.i == 2 || po.i == 3))
                || (po.v == 0 && po.i % 10 != 4 && po.i % 10 != 6 && po.i % 10 != 9)
                || (po.v != 0 && po.f % 10 != 4 && po.f % 10 != 6 && po.f % 10 != 9)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "naq" => |po| {
            if (po.n == 2) {
                PluralCategory::TWO
            } else if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "st" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nr" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "asa" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "wa" => |po| {
            if (po.n == 0..=1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "xog" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "el" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "rof" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mas" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "as" => |po| {
            if (po.i == 0) || (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "be" => |po| {
            if (po.n % 10 == 0) || (po.n % 10 == 5..=9) || (po.n % 100 == 11..=14) {
                PluralCategory::MANY
            } else if ((po.n % 10 == 2..=4) && (po.n % 100 != 12..=14)) {
                PluralCategory::FEW
            } else if (po.n % 10 == 1 && po.n % 100 != 11) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ku" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "tn" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "gd" => |po| {
            if (po.n == 1 || po.n == 11) {
                PluralCategory::ONE
            } else if ((po.n == 3..=10) || (po.n == 13..=19)) {
                PluralCategory::FEW
            } else if (po.n == 2 || po.n == 12) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "ko" => |po| PluralCategory::OTHER,
        "wo" => |po| PluralCategory::OTHER,
        "brx" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ar" => |po| {
            if (po.n % 100 == 11..=99) {
                PluralCategory::MANY
            } else if (po.n == 2) {
                PluralCategory::TWO
            } else if (po.n == 0) {
                PluralCategory::ZERO
            } else if (po.n == 1) {
                PluralCategory::ONE
            } else if (po.n % 100 == 3..=10) {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "yue" => |po| PluralCategory::OTHER,
        "ff" => |po| {
            if (po.i == 0 || po.i == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "tzm" => |po| {
            if (po.n == 0..=1) || (po.n == 11..=99) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mo" => |po| {
            if (po.v != 0) || (po.n == 0) || (po.n != 1 && (po.n % 100 == 1..=19)) {
                PluralCategory::FEW
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nn" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "cgg" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "jv" => |po| PluralCategory::OTHER,
        "zh" => |po| PluralCategory::OTHER,
        "ha" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "gv" => |po| {
            if (po.v == 0
                && (po.i % 100 == 0 || po.i % 100 == 20 || po.i % 100 == 40 || po.i % 100 == 60
                    || po.i % 100 == 80))
            {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 10 == 1) {
                PluralCategory::ONE
            } else if (po.v != 0) {
                PluralCategory::MANY
            } else if (po.v == 0 && po.i % 10 == 2) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "seh" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nd" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ses" => |po| PluralCategory::OTHER,
        "az" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "my" => |po| PluralCategory::OTHER,
        "fi" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "dv" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "yo" => |po| PluralCategory::OTHER,
        "hu" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "se" => |po| {
            if (po.n == 2) {
                PluralCategory::TWO
            } else if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "no" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sv" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "lkt" => |po| PluralCategory::OTHER,
        "fil" => |po| {
            if (po.v == 0 && (po.i == 1 || po.i == 2 || po.i == 3))
                || (po.v == 0 && po.i % 10 != 4 && po.i % 10 != 6 && po.i % 10 != 9)
                || (po.v != 0 && po.f % 10 != 4 && po.f % 10 != 6 && po.f % 10 != 9)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "jgo" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ss" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "jmc" => |po| {
            if (po.n == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sl" => |po| {
            if (po.v == 0 && (po.i % 100 == 3..=4)) || (po.v != 0) {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 100 == 1) {
                PluralCategory::ONE
            } else if (po.v == 0 && po.i % 100 == 2) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "sw" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        _ => panic!("Unknown locale!"),
    }
}