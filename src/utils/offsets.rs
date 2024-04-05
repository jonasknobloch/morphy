use unicode_segmentation::UnicodeSegmentation;

pub fn collect_grapheme_offsets(segments: Vec<String>) -> Vec<(usize, usize)> {
    let mut grapheme_offsets: Vec<(usize, usize)> = vec![];

    let mut index = 0;

    for segment in segments {
        let length = segment.graphemes(true).count();

        grapheme_offsets.push((index, index + length));

        index += length;
    }

    return grapheme_offsets;
}

pub fn to_byte_offsets_scp(message: &str, character_offsets: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let chars = message.chars().collect::<Vec<char>>(); // unicode scalar values

    if character_offsets[character_offsets.len()-1].1 < chars.len() {
        panic!("invalid character offsets")
    }

    let mut byte_offsets: Vec<(usize, usize)> = vec![];

    let mut index = 0;

    for offsets in character_offsets {
        let length = chars[offsets.0..offsets.1]
            .iter()
            .map(|c| c.len_utf8())
            .sum::<usize>();

        byte_offsets.push((index, index + length));

        index += length;
    }

    return byte_offsets;
}

pub fn to_byte_offsets(message: &str, grapheme_offsets: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let graphemes = message.graphemes(true).collect::<Vec<&str>>(); // unicode grapheme clusters

    if grapheme_offsets[grapheme_offsets.len()-1].1 < graphemes.len() {
        panic!("invalid grapheme offsets")
    }

    let mut byte_offsets: Vec<(usize, usize)> = vec![];

    let mut index = 0;

    for offsets in grapheme_offsets {
        let length = graphemes[offsets.0..offsets.1]
            .iter()
            .map(|s| s.chars())
            .flatten()
            .map(|c| c.len_utf8())
            .sum::<usize>();

        byte_offsets.push((index, index + length));

        index += length;
    }

    return byte_offsets;
}

pub fn unicode_bounds(message: &str) -> Vec<usize> {
    let mut bounds: Vec<usize> = vec![];

    let mut index = 0;

    for grapheme in message.graphemes(true) {
        let length = grapheme.len();

        bounds.push(index + length);

        index += length;
    }

    return bounds;
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://stackoverflow.com/a/33349765
    // Binary    Hex          Comments
    // 0xxxxxxx  0x00..0x7F   Only byte of a 1-byte character encoding
    // 10xxxxxx  0x80..0xBF   Continuation byte: one of 1-3 bytes following the first
    // 110xxxxx  0xC0..0xDF   First byte of a 2-byte character encoding
    // 1110xxxx  0xE0..0xEF   First byte of a 3-byte character encoding
    // 11110xxx  0xF0..0xF7   First byte of a 4-byte character encoding

    const COMPOSED: &str = "\u{00E9}"; // "é" -> UTF-8: 0xC3 0xA9
    const DECOMPOSED: &str = "\u{0065}\u{0301}"; // "é" -> UTF-8: 0x65 0xCC 0x81

    #[test]
    fn test_to_byte_offsets_composed() {
        assert_eq!(to_byte_offsets_scp(COMPOSED, vec![(0, 1)]), vec![(0, 2)]);
    }

    #[test]
    #[ignore]
    fn test_to_byte_offset_decomposed() {
        assert_eq!(to_byte_offsets_scp(DECOMPOSED, vec![(0, 1)]), vec![(0, 3)]); // returns [(0, 1)]
    }

    #[test]
    fn test_to_byte_offsets_unicode() {
        assert_eq!(to_byte_offsets(COMPOSED, vec![(0, 1)]), vec![(0, 2)]);
        assert_eq!(to_byte_offsets(DECOMPOSED, vec![(0, 1)]), vec![(0, 3)]);
    }

    #[test]
    fn test_unicode_bounds() {
        assert_eq!(unicode_bounds("foo"), vec![1, 2, 3]);
        assert_eq!(unicode_bounds("lél"), vec![1, 3, 4]);
    }
}