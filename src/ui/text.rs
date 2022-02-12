#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextBaseline {
    Alphabetic,
    Ideographic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirection {
    /// The text flows from right to left (e.g. Arabic, Hebrew).
    RTL,

    /// The text flows from left to right (e.g., English, French).
    LTR,
}
