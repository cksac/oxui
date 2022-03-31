use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::rendering::Offset;

pub trait HitTestTarget: Debug {
    fn handle_event(&mut self, event: PointerEvent, entry: HitTestEntry) {}
}

#[derive(Debug)]
pub struct HitTestResult {
    pub(crate) path: Vec<HitTestEntry>,
}

impl HitTestResult {
    pub fn new() -> HitTestResult {
        HitTestResult { path: Vec::new() }
    }

    pub fn add(&mut self, entry: HitTestEntry) {
        self.path.push(entry)
    }
}

#[derive(Debug, Clone)]
pub struct HitTestEntry {
    pub(crate) target: Rc<RefCell<dyn HitTestTarget>>,
    //transform: Option<Matrix4>
}

impl HitTestEntry {
    pub fn new(target: Rc<RefCell<dyn HitTestTarget>>) -> Self {
        HitTestEntry { target }
    }
}

#[derive(Debug)]
pub struct PointerEvent {
    down: bool,
    position: Offset,
}

impl PointerEvent {
    pub fn new(position: Offset) -> Self {
        PointerEvent {
            down: false,
            position,
        }
    }
}
