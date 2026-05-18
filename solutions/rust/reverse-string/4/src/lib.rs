use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    UnicodeSegmentation::graphemes(input, true).rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse("uüu"), "uüu");
    }

    #[test]
    fn test_reverse_grapheme() {
        let input = "Würstchenstand";
        let output = reverse(input);
        let expected = "dnatsnehctsrüW";
        assert_eq!(output, expected);
    }
}
