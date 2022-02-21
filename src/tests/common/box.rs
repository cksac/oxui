use std::any::{type_name, TypeId};

use crate::rendering::{BoxConstraints, RenderBox, RenderObject, Size};

#[derive(Debug)]
pub struct RenderSizedBox {
    pub(crate) size: Size,
}

impl RenderSizedBox {
    pub fn new(width: f32, height: f32) -> Self {
        RenderSizedBox {
            size: Size::new(width, height),
        }
    }
}

impl RenderObject for RenderSizedBox {
    fn ty_id(&self) -> std::any::TypeId {
        TypeId::of::<Self>()
    }

    fn ty_name(&self) -> &'static str {
        type_name::<Self>()
    }
}

impl RenderBox for RenderSizedBox {
    fn perform_layout(&mut self, constraints: &BoxConstraints) {}

    fn perform_resize(&mut self, constraints: &BoxConstraints) {
        todo!()
    }

    fn size(&self) -> Size {
        self.size
    }
}
