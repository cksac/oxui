use std::{any::TypeId, cell::RefCell, rc::Rc};
use typed_builder::TypedBuilder;

use crate::{
    rendering::{
        Axis, BoxConstraints, Clip, CrossAxisAlignment, FlexFit, MainAxisAlignment, MainAxisSize,
        RenderBox, RenderConstrainedBox, RenderFlex, RenderFlexible, TextBaseline, TextDirection,
        VerticalDirection,
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
    #[track_caller]
    fn create(&self, context: BuildContext) -> Rc<RefCell<dyn RenderBox>> {
        context.group(
            |_| {
                let mut flex = RenderFlex::default();
                flex.direction = self.direction;
                Rc::new(RefCell::new(flex))
            },
            |_| {
                // TODO: condition to skip whole Flex?
                false
            },
            |cx| {
                let mut children = Vec::with_capacity(self.children.len());
                for child in self.children.iter() {
                    children.push(child.create(cx));
                }
                children
            },
            |n, children| {
                let mut flex = n.borrow_mut();
                flex.children.clear();
                flex.children = children;
            },
            |n| n.clone(),
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
    #[track_caller]
    fn create(&self, context: BuildContext) -> RenderFlexible {
        context.group(
            |cx| {
                // temp set child to RenderConstrainedBox first
                RenderFlexible::new(
                    Rc::new(RefCell::new(RenderConstrainedBox::new(
                        BoxConstraints::default(),
                    ))),
                    self.flex,
                    self.fit,
                )
            },
            |n| false,
            |cx| self.child.create(cx),
            |n, child| {
                n.flex = self.flex;
                n.fit = self.fit;
                n.inner = child
            },
            |n| n.clone(),
        )
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
