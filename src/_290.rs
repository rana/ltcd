/// 290. Word Pattern
///
/// Given a pattern and a string s, find if s
/// follows the same pattern.
///
/// Here follow means a full match, such that there
/// is a bijection between a letter in pattern and a
/// non-empty word in s.
///
/// Constraints:
/// * 1 <= pattern.length <= 300
/// * pattern contains only lower-case English letters.
/// * 1 <= s.length <= 3000
/// * s contains only lowercase English letters and spaces ' '.
/// * s does not contain any leading or trailing spaces.
/// * All the words in s are separated by a single space.

fn word_pattern(pattern: &str, s: &str) -> bool {
    use std::collections::HashMap;

    let wrds: Vec<&str> = s.split_whitespace().collect();

    if pattern.len() != wrds.len() {
        return false;
    }

    let mut chr_to_wrd: HashMap<char, &str> = HashMap::new();
    let mut wrd_to_chr: HashMap<&str, char> = HashMap::new();

    // Loop through each character and word simultaneously.
    // Contributes to O(n + m) time complexity.
    for (chr, wrd) in pattern.chars().zip(wrds.into_iter()) {
        // chr-wrd mapping.
        if let Some(&wrd_map) = chr_to_wrd.get(&chr) {
            if wrd_map != wrd {
                return false;
            }
        } else {
            chr_to_wrd.insert(chr, wrd);
        }

        // wrd-chr mapping.
        if let Some(&chr_map) = wrd_to_chr.get(wrd) {
            if chr_map != chr {
                return false;
            }
        } else {
            wrd_to_chr.insert(wrd, chr);
        }
    }

    true
}

fn word_pattern_a(pattern: &str, s: &str) -> bool {
    use std::collections::HashMap;

    let words: Vec<&str> = s.split_whitespace().collect();

    if pattern.len() != words.len() {
        return false;
    }

    let mut char_to_word: HashMap<char, &str> = HashMap::new();
    let mut word_to_char: HashMap<&str, char> = HashMap::new();

    for (i, ch) in pattern.chars().enumerate() {
        let word = words[i];

        if let Some(&mapped_word) = char_to_word.get(&ch) {
            if mapped_word != word {
                return false;
            }
        } else {
            char_to_word.insert(ch, word);
        }

        if let Some(&mapped_char) = word_to_char.get(word) {
            if mapped_char != ch {
                return false;
            }
        } else {
            word_to_char.insert(word, ch);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::word_pattern;

    #[test]
    fn test_common_case_1() {
        let pattern = "abba";
        let s = "dog cat cat dog";
        assert!(word_pattern(pattern, s));
    }

    #[test]
    fn test_common_case_2() {
        let pattern = "abba";
        let s = "dog cat cat fish";
        assert!(!word_pattern(pattern, s));
    }

    #[test]
    fn test_one_to_one_mapping() {
        let pattern = "abc";
        let s = "dog cat fish";
        assert!(word_pattern(pattern, s));
    }

    #[test]
    fn test_repeating_pattern_no_repeat_words() {
        let pattern = "abba";
        let s = "dog cat cat cat";
        assert!(!word_pattern(pattern, s));
    }

    #[test]
    fn test_edge_case_empty_pattern() {
        let pattern = "";
        let s = "";
        assert!(word_pattern(pattern, s));
    }

    #[test]
    fn test_edge_case_unmatched_length() {
        let pattern = "a";
        let s = "dog cat";
        assert!(!word_pattern(pattern, s));
    }

    #[test]
    fn test_edge_case_single_word() {
        let pattern = "a";
        let s = "dog";
        assert!(word_pattern(pattern, s));
    }

    #[test]
    fn test_edge_case_no_repeated_words() {
        let pattern = "abcabc";
        let s = "dog cat fish dog cat fish";
        assert!(word_pattern(pattern, s));
    }
}
