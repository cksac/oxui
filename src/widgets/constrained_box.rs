use std::{cell::RefCell, rc::Rc};

use crate::{
    rendering::{BoxConstraints, RenderBox, RenderConstrainedBox},
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
    fn create(&self, context: &BuildContext) -> Rc<RefCell<dyn RenderBox>> {
        context.once_with(
            || RenderConstrainedBox::new(self.constraints),
            |element| {
                element.additional_constraints = self.constraints;
            },
        )
    }
}
