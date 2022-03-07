use crate::{
    rendering::{
        Axis, Clip, CrossAxisAlignment, FlexFit, MainAxisAlignment, MainAxisSize, RenderFlex,
        RenderFlexible, TextBaseline, TextDirection, VerticalDirection,
    },
    widgets::{Element, Widget},
};

pub struct Flex {
    pub direction: Axis,
    pub main_axis_size: MainAxisSize,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
    pub vertical_direction: VerticalDirection,
    pub text_direction: Option<TextDirection>,
    pub text_baseline: Option<TextBaseline>,
    pub clip_behavior: Clip,
    pub children: Vec<Flexible>,
}

impl Widget for Flex {
    fn build(&self) -> Element {
        let mut flex = RenderFlex::default();
        for child in self.children.iter() {
            flex.children.push(child.build());
        }
        flex.into()
    }
}

pub struct Flexible {
    pub flex: usize,
    pub fit: FlexFit,
    pub child: Box<dyn Widget>,
}

impl Flexible {
    fn build(&self) -> RenderFlexible {
        let child = self.child.build().into();
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
