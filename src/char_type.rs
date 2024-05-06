use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum CharType {
    Space,
    WesternLetter,
    CjkChar,
    HalfwidthPauseOrStop,
    FullwidthPauseOrStop,
    HalfwidthQuotation,
    FullwidthQuotation,
    HalfwidthBracket,
    FullwidthBracket,
    HalfwidthOtherPunctuation,
    FullwidthOtherPunctuation,
    Unknown,
}

const HALFWIDTH_PAUSE_OR_STOP: [char; 6] = [
    ',', '.', ';', ':', '?', '!',
];
const FULLWIDTH_PAUSE_OR_STOP: [char; 10] = [
    // normal punctuation marks
    '，', '。', '；', '：', '？', '！',
    // special punctuation marks
    '⁈', '⁇', '‼', '⁉',
];
const HALFWIDTH_QUOTATION: [char; 2] = [
    '"', '\'',
];
const FULLWIDTH_QUOTATION: [char; 16] = [
    '“', '”', '‘', '’',
    '《', '》', '〈', '〉',
    '『', '』', '「', '」',
    '【', '】', '〖', '〗',
];
const HALFWIDTH_BRACKET: [char; 6] = [
    '(', ')', '[', ']', '{', '}',
];
const FULLWIDTH_BRACKET: [char; 8] = [
    '（', '）', '〔', '〕', '［', '］', '｛', '｝',
];
const HALFWIDTH_OTHER_PUNCTUATION: [char; 19] = [
    // on-keyboard symbols
    '~', '-', '+', '*', '/', '\\', '%', '=', '&', '|', '`', '<', '>', '@', '#', '$', '^',
    // symbols of death
    '†', '‡'
];
const FULLWIDTH_OTHER_PUNCTUATION: [char; 10] = [
    // U+2E3A TWO-EM DASH, U+2014 EM DASH
    '—', '⸺',
    // U+2026 HORIZONTAL ELLIPSIS, U+22EF MIDLINE HORIZONTAL ELLIPSIS
    '…', '⋯',
    // U+FF5E FULLWIDTH TILDE
    '～',
    // U+25CF BLACK CIRCLE, U+2022 BULLET, U+00B7 MIDDLE DOT,
    // U+2027 HYPHENATION POINT, U+30FB KATAKANA MIDDLE DOT
    '●', '•', '·', '‧', '・'
];

fn is_match(c: char, pattern: &str) -> bool {
    let re = Regex::new(pattern).unwrap();
    re.is_match(&c.to_string())
}

#[allow(dead_code)]
pub fn get_char_type(c: char) -> CharType {
    // space
    if is_match(c, "\\s") {
        return CharType::Space;
    }

    // punctuation marks
    if HALFWIDTH_PAUSE_OR_STOP.contains(&c) {
        return CharType::HalfwidthPauseOrStop;
    } else if FULLWIDTH_PAUSE_OR_STOP.contains(&c) {
        return CharType::FullwidthPauseOrStop;
    } else if HALFWIDTH_QUOTATION.contains(&c) {
        return CharType::HalfwidthQuotation;
    } else if FULLWIDTH_QUOTATION.contains(&c) {
        return CharType::FullwidthQuotation;
    } else if HALFWIDTH_BRACKET.contains(&c) {
        return CharType::HalfwidthBracket;
    } else if FULLWIDTH_BRACKET.contains(&c) {
        return CharType::FullwidthBracket;
    } else if HALFWIDTH_OTHER_PUNCTUATION.contains(&c) {
        return CharType::HalfwidthOtherPunctuation;
    } else if FULLWIDTH_OTHER_PUNCTUATION.contains(&c) {
        return CharType::FullwidthOtherPunctuation;
    }

    // 0-9
    if is_match(c, "[0-9]") {
        return CharType::WesternLetter;
    }

    // Basic Latin
    if is_match(c, "[\\u0020-\\u007F]") {
        return CharType::WesternLetter;
    }

    // Latin-1 Supplement
    if is_match(c, "[\\u00A0-\\u00FF]") {
        return CharType::WesternLetter;
    }
    // Latin Extended-A
    if is_match(c, "[\\u0100-\\u017F]") {
        return CharType::WesternLetter;
    }
    // Latin Extended-B
    if is_match(c, "[\\u0180-\\u024F]") {
        return CharType::WesternLetter;
    }
    // Greek and Coptic
    if is_match(c, "[\\u0370-\\u03FF]") {
        return CharType::WesternLetter;
    }

    // CJK Unified Ideographs
    if is_match(c, "[\\u4E00-\\u9FFF]") {
        return CharType::CjkChar
    }
    // CJK Unified Ideographs Extension A
    if is_match(c, "[\\u3400-\\u4DBF]") {
        return CharType::CjkChar
    }
    // CJK Unified Ideographs Extension B
    if is_match(c, "[\\ud840-\\ud868][\\udc00-\\udfff]|\\ud869[\\udc00-\\uded6]") {
        return CharType::CjkChar
    }
    // CJK Unified Ideographs Extension C
    if is_match(c, "\\ud869[\\udf00-\\udfff]|[\\ud86a-\\ud86c][\\udc00-\\udfff]|\\ud86d[\\udc00-\\udf34]") {
        return CharType::CjkChar
    }
    // CJK Unified Ideographs Extension D
    if is_match(c, "\\ud86d[\\udf40-\\udfff]|\\ud86e[\\udc00-\\udc1d]") {
        return CharType::CjkChar
    }
    // CJK Compatibility Ideographs
    if is_match(c, "[\\uF900-\\uFAFF]") {
        return CharType::CjkChar
    }
    // CJK Compatibility Forms
    if is_match(c, "[\\uFE30-\\uFE4F]") {
        return CharType::CjkChar
    }
    // CJK Radicals Supplement
    if is_match(c, "[\\u2E80-\\u2EFF]") {
        return CharType::CjkChar
    }
    // Private Use Area (part)
    if is_match(c, "[\\uE815-\\uE864]") {
        return CharType::CjkChar
    }
    // CJK Unified Ideographs Extension B
    if is_match(c, "[\\u{20000}-\\u{2A6DF}]") {
        return CharType::CjkChar
    }
    // CJK Compatibility Ideographs Supplement
    if is_match(c, "[\\u{2F800}-\\u{2FA1F}]") {
        return CharType::CjkChar
    }

    // CJK Symbols and Punctuation
    if is_match(c, "[\\u3000-\\u303F]") {
        return CharType::FullwidthOtherPunctuation
    }

    return CharType::Unknown;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_char_type() {
        assert_eq!(get_char_type(' '), CharType::Space);
        assert_eq!(get_char_type('a'), CharType::WesternLetter);
        assert_eq!(get_char_type('A'), CharType::WesternLetter);
        assert_eq!(get_char_type('0'), CharType::WesternLetter);
        assert_eq!(get_char_type('9'), CharType::WesternLetter);
        assert_eq!(get_char_type(','), CharType::HalfwidthPauseOrStop);
        assert_eq!(get_char_type('.'), CharType::HalfwidthPauseOrStop);
        assert_eq!(get_char_type(';'), CharType::HalfwidthPauseOrStop);
        assert_eq!(get_char_type(':'), CharType::HalfwidthPauseOrStop);
        assert_eq!(get_char_type('?'), CharType::HalfwidthPauseOrStop);
        assert_eq!(get_char_type('!'), CharType::HalfwidthPauseOrStop);
        assert_eq!(get_char_type('，'), CharType::FullwidthPauseOrStop);
        assert_eq!(get_char_type('。'), CharType::FullwidthPauseOrStop);
        assert_eq!(get_char_type('；'), CharType::FullwidthPauseOrStop);
        assert_eq!(get_char_type('：'), CharType::FullwidthPauseOrStop);
        assert_eq!(get_char_type('？'), CharType::FullwidthPauseOrStop);
        assert_eq!(get_char_type('！'), CharType::FullwidthPauseOrStop);
        assert_eq!(get_char_type('⁈'), CharType::FullwidthPauseOrStop);
        assert_eq!(get_char_type('⁇'), CharType::FullwidthPauseOrStop);
        assert_eq!(get_char_type('‼'), CharType::FullwidthPauseOrStop);
        assert_eq!(get_char_type('⁉'), CharType::FullwidthPauseOrStop);
        assert_eq!(get_char_type('"'), CharType::HalfwidthQuotation);
        assert_eq!(get_char_type('\''), CharType::HalfwidthQuotation);
        assert_eq!(get_char_type('“'), CharType::FullwidthQuotation);
        assert_eq!(get_char_type('”'), CharType::FullwidthQuotation);
        assert_eq!(get_char_type('‘'), CharType::FullwidthQuotation);
        assert_eq!(get_char_type('’'), CharType::FullwidthQuotation);
    }
}