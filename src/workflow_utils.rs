/// Simple aligned text: left and right with spacing (no theme-based truncation).
pub fn aligned_text(left: &str, right: &str) -> String {
    format!("{}    {}", left, right)
}
