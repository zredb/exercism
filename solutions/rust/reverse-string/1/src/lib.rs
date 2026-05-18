use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    UnicodeSegmentation::graphemes(input, true)
        .rev()
        .collect::<String>()
        .nfc()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse("uüu"), "uüu");
    }
}
