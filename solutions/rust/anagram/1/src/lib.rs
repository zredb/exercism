use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let mut word_chars: Vec<char> = lower_word.chars().collect();
    word_chars.sort_unstable();
    possible_anagrams
        .iter()
        .filter(|candidate| {
            let lower_caldidate = candidate.to_lowercase();
            if lower_caldidate == lower_word {
                return false;
            }
            let mut candidate_chars: Vec<char> = lower_caldidate.chars().collect();
            candidate_chars.sort_unstable();
            candidate_chars == word_chars
        })
        .copied()
        .collect()
}
