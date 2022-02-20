use std::any::{type_name, TypeId};

use crate::rendering::{BoxConstraints, RenderBox, RenderObject, Size};

pub struct RenderConstrainedBox {
    // RenderObject
    pub(crate) size: Size,

    // RenderConstrainedBox
    pub(crate) additional_constraints: BoxConstraints,
    pub(crate) child: Option<Box<dyn RenderBox>>,
}

impl RenderConstrainedBox {
    pub fn new(constraints: BoxConstraints) -> Self {
        RenderConstrainedBox {
            size: Size::zero(),
            additional_constraints: constraints,
            child: None,
        }
    }
}

impl RenderObject for RenderConstrainedBox {
    fn ty_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn ty_name(&self) -> &'static str {
        type_name::<Self>()
    }
}

impl RenderBox for RenderConstrainedBox {
    fn perform_layout(&mut self, constraints: &BoxConstraints) {
        self.size = match &mut self.child {
            Some(child) => {
                child.layout(constraints, true);
                child.size()
            }
            None => self
                .additional_constraints
                .enforce(constraints)
                .constrain(Size::zero()),
        };
    }

    fn perform_resize(&mut self, constraints: &BoxConstraints) {
        todo!()
    }

    fn size(&self) -> Size {
        self.size
    }
}
