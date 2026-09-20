/// Simple aligned text: left and right with spacing (no theme-based truncation).
pub fn aligned_text(left: &str, right: &str) -> String {
    format!("{}    {}", left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aligned_text_joins_with_four_spaces() {
        assert_eq!(aligned_text("hello", "world"), "hello    world");
    }

    #[test]
    fn aligned_text_keeps_empty_sides() {
        assert_eq!(aligned_text("", "right"), "    right");
        assert_eq!(aligned_text("left", ""), "left    ");
        assert_eq!(aligned_text("", ""), "    ");
    }
}
