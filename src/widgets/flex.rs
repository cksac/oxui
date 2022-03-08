use std::{cell::RefCell, rc::Rc};

use typed_builder::TypedBuilder;

use crate::{
    rendering::{
        Axis, Clip, CrossAxisAlignment, FlexFit, MainAxisAlignment, MainAxisSize, RenderBox,
        RenderFlex, RenderFlexible, TextBaseline, TextDirection, VerticalDirection,
    },
    widgets::{BuildContext, Widget},
};

#[derive(Debug, TypedBuilder)]
pub struct Flex {
    #[builder(default=Axis::Horizontal)]
    pub direction: Axis,

    #[builder(default=MainAxisSize::Max)]
    pub main_axis_size: MainAxisSize,

    #[builder(default=MainAxisAlignment::Start)]
    pub main_axis_alignment: MainAxisAlignment,

    #[builder(default=CrossAxisAlignment::Center)]
    pub cross_axis_alignment: CrossAxisAlignment,

    #[builder(default=VerticalDirection::Down)]
    pub vertical_direction: VerticalDirection,

    #[builder(default=Some(TextDirection::LTR), setter(strip_option))]
    pub text_direction: Option<TextDirection>,

    #[builder(default, setter(strip_option))]
    pub text_baseline: Option<TextBaseline>,

    #[builder(default=Clip::None)]
    pub clip_behavior: Clip,

    pub children: Vec<Flexible>,
}

impl Widget for Flex {
    fn create(&self, context: &BuildContext) -> Rc<RefCell<dyn RenderBox>> {
        context.once_with(
            || {
                let mut flex = RenderFlex::default();
                for child in self.children.iter() {
                    flex.children.push(child.create(context));
                }
                flex
            },
            |flex| {
                flex.children.clear();
                for child in self.children.iter() {
                    flex.children.push(child.create(context));
                }
            },
        )
    }
}

#[derive(Debug, TypedBuilder)]
pub struct Flexible {
    pub flex: usize,
    pub fit: FlexFit,
    pub child: Box<dyn Widget>,
}

impl Flexible {
    fn create(&self, context: &BuildContext) -> RenderFlexible {
        let child = self.child.create(context).into();
        RenderFlexible::new(child, self.flex, self.fit)
    }
}

impl<T> From<T> for Flexible
where
    T: Widget + 'static,
{
    fn from(w: T) -> Self {
        Self {
            flex: 1,
            fit: FlexFit::Loose,
            child: Box::new(w),
        }
    }
}
