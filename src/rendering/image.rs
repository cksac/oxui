use crate::rendering::{BoxConstraints, RenderBox, RenderObject, Size};
use crate::ui::Image;
use std::any::{type_name, TypeId};
use std::borrow::Borrow;

#[derive(Debug)]
pub struct RenderImage {
    // RenderObject
    pub(crate) size: Size,

    // RenderImage
    pub(crate) image: Option<Image>,
    pub(crate) width: Option<f32>,
    pub(crate) height: Option<f32>,
}

impl Default for RenderImage {
    fn default() -> Self {
        RenderImage {
            image: None,
            width: None,
            height: None,
            size: Size::zero(),
        }
    }
}

impl RenderImage {
    pub fn new(width: impl Into<Option<f32>>, heigh: impl Into<Option<f32>>) -> Self {
        RenderImage {
            image: None,
            width: width.into(),
            height: heigh.into(),
            size: Size::zero(),
        }
    }
}

impl From<Image> for RenderImage {
    fn from(img: Image) -> Self {
        let size = Size::new(img.width() as f32, img.height() as f32);
        RenderImage {
            image: img.into(),
            width: None,
            height: None,
            size,
        }
    }
}

impl RenderBox for RenderImage {
    fn perform_layout(&mut self, constraints: &BoxConstraints) {
        let constraints = BoxConstraints::tight_for(self.width, self.height).enforce(constraints);
        self.size = match &self.image {
            Some(img) => constraints
                .borrow()
                .constrain_with_aspect_ratio((img.width() as f32, img.height() as f32)),
            None => constraints.borrow().smallest(),
        };
    }

    fn perform_resize(&mut self, constraints: &BoxConstraints) {
        todo!()
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl RenderObject for RenderImage {
    fn ty_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn ty_name(&self) -> &'static str {
        type_name::<Self>()
    }
}
