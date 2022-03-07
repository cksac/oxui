use crate::rendering::{RenderBox, RenderSliver};

pub trait Widget {
    fn build(&self) -> Element;
}

pub trait SliverWidget {
    fn build(&self) -> SliverElement;
}

pub struct Element {
    pub(crate) inner: Box<dyn RenderBox>,
}
impl<T: 'static> From<T> for Element
where
    T: RenderBox,
{
    fn from(obj: T) -> Self {
        Element {
            inner: Box::new(obj),
        }
    }
}
impl From<Element> for Box<dyn RenderBox> {
    fn from(e: Element) -> Self {
        e.inner
    }
}

pub struct SliverElement {
    inner: Box<dyn RenderSliver>,
}

impl<T: 'static> From<T> for SliverElement
where
    T: RenderSliver,
{
    fn from(obj: T) -> Self {
        SliverElement {
            inner: Box::new(obj),
        }
    }
}

impl From<SliverElement> for Box<dyn RenderSliver> {
    fn from(e: SliverElement) -> Self {
        e.inner
    }
}
