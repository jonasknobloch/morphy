pub fn collect_scalar_offsets(segments: Vec<String>) -> Vec<(usize, usize)> {
    let mut character_offsets: Vec<(usize, usize)> = vec![];

    let mut index = 0;

    for segment in segments {
        let length = segment.chars().count();

        character_offsets.push((index, index + length));

        index += length;
    }

    return character_offsets;
}

pub fn scalar_to_byte_offsets(
    message: &str,
    character_offsets: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let chars = message.chars().collect::<Vec<char>>(); // unicode scalar values

    if character_offsets[character_offsets.len() - 1].1 < chars.len() {
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

pub fn unicode_scalar_bounds(message: &str) -> Vec<usize> {
    let mut bounds: Vec<usize> = vec![];

    let mut index = 0;

    for character in message.chars() {
        let length = character.len_utf8();

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
    fn test_scalar_to_byte_offsets_composed() {
        assert_eq!(scalar_to_byte_offsets(COMPOSED, vec![(0, 1)]), vec![(0, 2)]);
    }

    #[test]
    fn test_scalar_to_byte_offset_decomposed() {
        assert_eq!(
            scalar_to_byte_offsets(DECOMPOSED, vec![(0, 2)]),
            vec![(0, 3)]
        );
    }

    #[test]
    fn test_unicode_scalar_bounds() {
        assert_eq!(unicode_scalar_bounds("foo"), vec![1, 2, 3]);
        assert_eq!(unicode_scalar_bounds("l\u{00E9}l"), vec![1, 3, 4]);
        assert_eq!(
            unicode_scalar_bounds("l\u{0065}\u{0301}l"),
            vec![1, 2, 4, 5]
        );
    }

    #[test]
    fn test_collect_scalar_offsets() {
        assert_eq!(
            collect_scalar_offsets(vec!["foo".to_string()]),
            vec![(0, 3)]
        );
        assert_eq!(
            collect_scalar_offsets(vec![
                "l".to_string(),
                "\u{00E9}".to_string(),
                "l".to_string()
            ]),
            vec![(0, 1), (1, 2), (2, 3)]
        );
        assert_eq!(
            collect_scalar_offsets(vec![
                "l".to_string(),
                "\u{0065}\u{0301}".to_string(),
                "l".to_string()
            ]),
            vec![(0, 1), (1, 3), (3, 4)]
        );
    }
}
