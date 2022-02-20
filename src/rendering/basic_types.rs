#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn zero() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    pub fn main_size(&self, direction: Axis) -> f32 {
        match direction {
            Axis::Horizontal => self.width,
            Axis::Vertical => self.height,
        }
    }

    pub fn cross_size(&self, direction: Axis) -> f32 {
        match direction {
            Axis::Horizontal => self.height,
            Axis::Vertical => self.width,
        }
    }
}

impl From<(f32, f32)> for Size {
    fn from(val: (f32, f32)) -> Self {
        Self {
            width: val.0,
            height: val.1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Offset {
    pub x: f32,
    pub y: f32,
}

impl Offset {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl std::ops::Add<Offset> for Offset {
    type Output = Self;
    fn add(self, rhs: Offset) -> Self::Output {
        Offset {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<Offset> for Offset {
    fn add_assign(&mut self, rhs: Offset) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisDirection {
    Up,
    Right,
    Down,
    Left,
}

impl From<AxisDirection> for Axis {
    fn from(dir: AxisDirection) -> Self {
        match dir {
            AxisDirection::Up | AxisDirection::Down => Axis::Vertical,
            AxisDirection::Left | AxisDirection::Right => Axis::Horizontal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Idle,
    Forward,
    Reverse,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clip {
    None,
    HardEdge,
    AntiAlias,
    AntiAliasWithSaveLayer,
}
