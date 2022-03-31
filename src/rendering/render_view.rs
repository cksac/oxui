use std::{
    any::{type_name, TypeId},
    cell::RefCell,
    rc::Rc,
};

use crate::{
    gestures::{HitTestEntry, HitTestResult, HitTestTarget},
    rendering::{BoxConstraints, Offset, PaintContext, RenderBox, RenderObject, Size},
};

#[derive(Debug)]
pub struct RenderView {
    // RenderObject
    pub(crate) size: Size,

    pub(crate) child: Option<Rc<RefCell<dyn RenderBox>>>,
}

impl RenderView {
    pub fn new() -> Self {
        RenderView {
            size: Size::zero(),
            child: None,
        }
    }
}

impl HitTestTarget for RenderView {}

impl RenderObject for RenderView {
    fn ty_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn ty_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn paint(&self, context: &mut PaintContext, offset: Offset) {
        if let Some(child) = &self.child {
            child.borrow().paint(context, offset);
        }
    }

    fn hit_test(&self, position: Offset, result: &mut HitTestResult) -> bool {
        if let Some(child) = &self.child {
            let is_hit = child.borrow().hit_test(position, result);
            if is_hit {
                let entry = HitTestEntry::new(child.clone());
                result.add(entry);
            }
            is_hit
        } else {
            false
        }
    }
}

impl RenderBox for RenderView {
    fn perform_layout(&mut self, constraints: &BoxConstraints) {
        self.size = match &mut self.child {
            Some(child) => {
                child.borrow_mut().layout(constraints, true);
                child.borrow().size()
            }
            None => constraints.constrain(Size::zero()),
        };
    }

    fn perform_resize(&mut self, constraints: &BoxConstraints) {
        if let Some(child) = &mut self.child {
            child.borrow_mut().perform_resize(constraints);
            self.size = child.borrow().size();
        }
    }

    fn size(&self) -> Size {
        self.size
    }
}
