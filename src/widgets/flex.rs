use typed_builder::TypedBuilder;

use crate::{
    rendering::{
        Axis, Clip, CrossAxisAlignment, FlexFit, MainAxisAlignment, MainAxisSize, RenderFlex,
        RenderFlexible, TextBaseline, TextDirection, VerticalDirection,
    },
    widgets::{Element, Widget},
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
    #[topo::nested]
    fn create(&self) -> Element {
        let mut flex = RenderFlex::default();
        for child in self.children.iter() {
            flex.children.push(child.create());
        }
        flex.into()
    }
}

#[derive(Debug, TypedBuilder)]
pub struct Flexible {
    pub flex: usize,
    pub fit: FlexFit,
    pub child: Box<dyn Widget>,
}

impl Flexible {
    #[topo::nested]
    fn create(&self) -> RenderFlexible {
        let child = self.child.create().into();
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
