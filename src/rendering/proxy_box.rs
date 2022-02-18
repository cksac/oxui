use std::any::{type_name, TypeId};

use crate::rendering::{BoxConstraints, Layout, RenderBox, RenderObject, Size};

#[derive(Default)]
pub struct RenderConstrainedBox {
    // RenderObject
    pub(crate) size: Option<Size>,

    // RenderConstrainedBox
    pub(crate) additional_constraints: BoxConstraints,
    pub(crate) child: Option<Box<dyn RenderBox>>,
}

impl RenderConstrainedBox {
    pub fn new(additional_constraints: BoxConstraints) -> Self {
        RenderConstrainedBox {
            size: None,
            additional_constraints,
            child: None,
        }
    }
}

impl Layout<BoxConstraints> for RenderConstrainedBox {
    fn perform_layout(&mut self, constraints: &BoxConstraints) {
        match &mut self.child {
            Some(child) => {
                child.layout_parent_use_size(constraints, true);
                self.size = child.size();
            }
            None => {
                self.size = Some(
                    self.additional_constraints
                        .enforce(constraints)
                        .constrain(Size::zero()),
                )
            }
        };
    }
}

impl RenderObject for RenderConstrainedBox {
    fn ty_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn ty_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn size(&self) -> Option<Size> {
        self.size
    }
}
