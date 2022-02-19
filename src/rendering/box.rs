use crate::rendering::{Layout, RenderObject, Size};
use std::borrow::Borrow;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BoxConstraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Default for BoxConstraints {
    fn default() -> Self {
        Self {
            min_width: 0.0,
            max_width: f32::INFINITY,
            min_height: 0.0,
            max_height: f32::INFINITY,
        }
    }
}

impl BoxConstraints {
    pub fn with_min_width(mut self, min_width: f32) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn with_max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn with_max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn with_min_height(mut self, min_height: f32) -> Self {
        self.min_height = min_height;
        self
    }

    pub fn tight(size: impl Into<Size>) -> Self {
        let size = size.into();
        Self {
            min_width: size.width,
            max_width: size.width,
            min_height: size.height,
            max_height: size.height,
        }
    }

    pub fn tight_for(width: impl Into<Option<f32>>, height: impl Into<Option<f32>>) -> Self {
        let (min_width, max_width) = width.into().map(|v| (v, v)).unwrap_or((0.0, f32::INFINITY));
        let (min_height, max_height) = height
            .into()
            .map(|v| (v, v))
            .unwrap_or((0.0, f32::INFINITY));

        Self {
            min_width,
            max_width,
            min_height,
            max_height,
        }
    }

    pub fn expand(width: impl Into<Option<f32>>, height: impl Into<Option<f32>>) -> Self {
        let (min_width, max_width) = width
            .into()
            .map(|v| (v, v))
            .unwrap_or((f32::INFINITY, f32::INFINITY));
        let (min_height, max_height) = height
            .into()
            .map(|v| (v, v))
            .unwrap_or((f32::INFINITY, f32::INFINITY));

        Self {
            min_width,
            max_width,
            min_height,
            max_height,
        }
    }

    pub fn enforce(&self, other: impl Borrow<BoxConstraints>) -> Self {
        let other = other.borrow();
        Self {
            min_width: self.min_width.clamp(other.min_width, other.max_width),
            max_width: self.max_width.clamp(other.min_width, other.max_width),
            min_height: self.min_height.clamp(other.min_height, other.max_height),
            max_height: self.max_height.clamp(other.min_height, other.max_height),
        }
    }

    pub fn has_tight_width(&self) -> bool {
        self.min_width >= self.max_width
    }

    pub fn has_tight_height(&self) -> bool {
        self.min_height >= self.max_height
    }

    pub fn is_tight(&self) -> bool {
        self.has_tight_width() && self.has_tight_height()
    }

    pub fn constrain_width(&self, width: f32) -> f32 {
        width.clamp(self.min_width, self.max_width)
    }

    pub fn constrain_height(&self, height: f32) -> f32 {
        height.clamp(self.min_height, self.max_height)
    }

    pub fn biggest(&self) -> Size {
        Size {
            width: self.constrain_width(f32::INFINITY),
            height: self.constrain_height(f32::INFINITY),
        }
    }

    pub fn smallest(&self) -> Size {
        Size {
            width: self.constrain_width(0.0),
            height: self.constrain_height(0.0),
        }
    }

    pub fn constrain(&self, size: impl Into<Size>) -> Size {
        let size = size.into();
        Size {
            width: self.constrain_width(size.width),
            height: self.constrain_height(size.height),
        }
    }

    pub fn constrain_with_aspect_ratio(&self, size: impl Into<Size>) -> Size {
        let size = size.into();
        if self.is_tight() {
            self.smallest()
        } else {
            let mut width = size.width;
            let mut height = size.height;
            let aspect_ratio = width / height;

            if width > self.max_width {
                width = self.max_width;
                height = width / aspect_ratio;
            }

            if height > self.max_height {
                height = self.max_height;
                width = height * aspect_ratio;
            }

            if width < self.min_width {
                width = self.min_width;
                height = width / aspect_ratio;
            }

            if height < self.min_height {
                height = self.min_height;
                width = height * aspect_ratio;
            }

            Size {
                width: self.constrain_width(width),
                height: self.constrain_height(height),
            }
        }
    }
}

pub trait RenderBox: Layout<BoxConstraints> + RenderObject {
    fn unwrap_size(&self) -> Size {
        self.size()
            .unwrap_or_else(|| panic!("RenderBox {} was not laid out", self.ty_name()))
    }
}

impl<T> RenderBox for T where T: Layout<BoxConstraints> + RenderObject {}
