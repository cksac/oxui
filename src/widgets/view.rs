use std::{cell::RefCell, rc::Rc};

use crate::{
    rendering::{RenderBox, RenderView},
    widgets::{BuildContext, Widget},
};

#[derive(Debug)]
pub struct View {
    pub child: Box<dyn Widget>,
}
impl View {
    pub fn new<T>(child: T) -> Self
    where
        T: 'static + Widget,
    {
        View {
            child: Box::new(child),
        }
    }
}

impl Widget for View {
    #[track_caller]
    fn create(&self, context: BuildContext) -> Rc<RefCell<dyn RenderBox>> {
        context.group(
            |_| Rc::new(RefCell::new(RenderView::new())),
            |_| false,
            |cx| self.child.create(cx),
            |n, child| {
                n.borrow_mut().child = Some(child);
            },
            |n| n.clone(),
        )
    }
}
