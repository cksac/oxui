use std::{cell::RefCell, rc::Rc};

use crate::{
    rendering::{BoxConstraints, Element, RenderConstrainedBox},
    widgets::{BuildContext, Widget},
};

#[derive(Debug)]
pub struct ConstrainedBox {
    pub constraints: BoxConstraints,
}

impl Default for ConstrainedBox {
    fn default() -> Self {
        Self {
            constraints: BoxConstraints::expand(),
        }
    }
}

impl Widget for ConstrainedBox {
    #[track_caller]
    fn create(&self, context: BuildContext) -> Element {
        context.memo(
            |_| Rc::new(RefCell::new(RenderConstrainedBox::new(self.constraints))),
            |n| n.borrow().additional_constraints == self.constraints,
            |n| n.borrow_mut().additional_constraints = self.constraints,
            |n| Element::new(n.clone()),
        )
    }
}
