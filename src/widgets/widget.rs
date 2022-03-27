use compose_rt::Composer;

use crate::{
    rendering::{Element, FlexFit, RenderSliver},
    widgets::Flexible,
};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

// pub struct BuildContext<'a> {
//     pub cx: &'a mut Composer,
// }

pub type BuildContext<'a> = &'a mut Composer;

pub trait Widget: Debug {
    #[track_caller]
    fn create(&self, context: BuildContext) -> Element;

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

pub trait SliverWidget: Debug {
    fn create(&self, context: &BuildContext) -> Rc<RefCell<dyn RenderSliver>>;
}
