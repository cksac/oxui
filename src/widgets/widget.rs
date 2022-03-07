use std::{cell::RefCell, rc::Rc};

use crate::{
    rendering::{FlexFit, RenderBox, RenderSliver},
    widgets::Flexible,
};

pub trait Widget {
    fn build(&self) -> Element;

    fn into_flexible(self, flex: usize, fit: FlexFit) -> Flexible
    where
        Self: 'static + Sized,
    {
        Flexible {
            flex,
            fit,
            child: Box::new(self),
        }
    }
}

pub trait SliverWidget {
    fn build(&self) -> SliverElement;
}

pub struct Element {
    pub(crate) inner: Rc<RefCell<dyn RenderBox>>,
}

impl Element {
    pub fn new(obj: Rc<RefCell<dyn RenderBox>>) -> Self {
        Self { inner: obj }
    }
}

impl<T: 'static> From<T> for Element
where
    T: RenderBox,
{
    fn from(obj: T) -> Self {
        Element {
            inner: Rc::new(RefCell::new(obj)),
        }
    }
}
impl From<Element> for Rc<RefCell<dyn RenderBox>> {
    fn from(e: Element) -> Self {
        e.inner
    }
}

pub struct SliverElement {
    inner: Rc<RefCell<dyn RenderSliver>>,
}

impl SliverElement {
    pub fn new(obj: Rc<RefCell<dyn RenderSliver>>) -> Self {
        Self { inner: obj }
    }
}

impl<T: 'static> From<T> for SliverElement
where
    T: RenderSliver,
{
    fn from(obj: T) -> Self {
        SliverElement {
            inner: Rc::new(RefCell::new(obj)),
        }
    }
}

impl From<SliverElement> for Rc<RefCell<dyn RenderSliver>> {
    fn from(e: SliverElement) -> Self {
        e.inner
    }
}
