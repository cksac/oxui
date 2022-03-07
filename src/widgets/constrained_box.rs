use std::{cell::RefCell, rc::Rc};

use crate::{
    rendering::{BoxConstraints, RenderConstrainedBox},
    widgets::{Element, Widget},
};

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
    #[topo::nested]
    fn build(&self) -> Element {
        // same call site return same render object
        let obj = moxie::once(|| {
            Rc::new(RefCell::new(RenderConstrainedBox::new(self.constraints)))
        });

        // apply changes
        // if self.constraints != obj.borrow().additional_constraints {
        //     // TODO: mark need layout / need paint
        // }

        obj.borrow_mut().additional_constraints = self.constraints;
        Element::new(obj)
    }
}
