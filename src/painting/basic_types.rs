#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn flip(&self) -> Self {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalDirection {
    Down,
    Up,
}
