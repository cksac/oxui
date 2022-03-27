use crate::rendering::RenderBox;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Element {
    pub render_box: Rc<RefCell<dyn RenderBox>>,
}

impl Element {
    pub fn new(v: Rc<RefCell<dyn RenderBox>>) -> Self {
        Element { render_box: v }
    }
}

impl<T> From<T> for Element
where
    T: 'static + RenderBox,
{
    fn from(val: T) -> Self {
        Element {
            render_box: Rc::new(RefCell::new(val)),
        }
    }
}

impl Deref for Element {
    type Target = Rc<RefCell<dyn RenderBox>>;

    fn deref(&self) -> &Self::Target {
        &self.render_box
    }
}

impl DerefMut for Element {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.render_box
    }
}