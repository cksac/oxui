use std::{
    any::{type_name, TypeId},
    cell::RefCell,
    rc::Rc,
};

use crate::{
    gestures::HitTestTarget,
    rendering::{BoxConstraints, Offset, PaintContext, RenderBox, RenderObject, Size},
};

#[derive(Debug)]
pub struct RenderConstrainedBox {
    // RenderObject
    pub(crate) size: Size,

    // RenderConstrainedBox
    pub(crate) additional_constraints: BoxConstraints,
    pub(crate) child: Option<Rc<RefCell<dyn RenderBox>>>,
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

impl HitTestTarget for RenderConstrainedBox {}

impl RenderObject for RenderConstrainedBox {
    fn ty_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn ty_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn paint(&self, context: &mut PaintContext, offset: Offset) {
        context.draw_rect(offset, self.size);
    }
}

impl RenderBox for RenderConstrainedBox {
    fn perform_layout(&mut self, constraints: &BoxConstraints) {
        self.size = match &mut self.child {
            Some(child) => {
                child.borrow_mut().layout(constraints, true);
                child.borrow().size()
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
