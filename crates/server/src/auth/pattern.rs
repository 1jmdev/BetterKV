pub(super) fn wildcard_match(pattern: &[u8], text: &[u8]) -> bool {
    let mut pattern_index = 0;
    let mut text_index = 0;
    let mut star = None;
    let mut star_match = 0;

    while text_index < text.len() {
        if pattern_index < pattern.len()
            && (pattern[pattern_index] == text[text_index] || pattern[pattern_index] == b'?')
        {
            pattern_index += 1;
            text_index += 1;
            continue;
        }

        if pattern_index < pattern.len() && pattern[pattern_index] == b'*' {
            star = Some(pattern_index);
            pattern_index += 1;
            star_match = text_index;
            continue;
        }

        match star {
            Some(position) => {
                pattern_index = position + 1;
                star_match += 1;
                text_index = star_match;
            }
            None => return false,
        }
    }

    while pattern_index < pattern.len() && pattern[pattern_index] == b'*' {
        pattern_index += 1;
    }

    pattern_index == pattern.len()
}

pub(super) fn any_pattern_matches(patterns: &[Vec<u8>], value: &[u8]) -> bool {
    patterns
        .iter()
        .any(|pattern| wildcard_match(pattern, value))
}
