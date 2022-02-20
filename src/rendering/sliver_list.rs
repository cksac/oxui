use std::any::{type_name, TypeId};

use crate::rendering::{
    Axis, AxisDirection, BoxConstraints, GrowthDirection, Offset, RenderBox, RenderObject,
    RenderSliver, ScrollDirection, Size, SliverConstraints,
};

pub struct RenderSliverList {
    // RenderBox
    pub(crate) size: Size,

    // RenderViewport
    pub(crate) direction: Axis,
    pub(crate) cache_extent: f32,
    pub(crate) children: Vec<Box<dyn RenderSliver>>,
}

impl RenderObject for RenderSliverList {
    fn ty_id(&self) -> std::any::TypeId {
        TypeId::of::<Self>()
    }

    fn ty_name(&self) -> &'static str {
        type_name::<Self>()
    }
}

impl RenderBox for RenderSliverList {
    fn size(&self) -> Size {
        self.size
    }

    fn perform_layout(&mut self, constraints: &BoxConstraints) {
        self.size = constraints.biggest();
        if self.size.is_empty() {
            return;
        }

        // if first layout
        let mut sliver_constraints = match self.direction {
            Axis::Horizontal => SliverConstraints {
                axis_direction: AxisDirection::Left,
                growth_direction: GrowthDirection::Forward,
                scroll_direction: ScrollDirection::Idle,
                scroll_offset: 0.0,
                preceding_scroll_extent: 0.0,
                overlap: 0.0,
                remaining_paint_extent: self.size.width,
                cross_axis_extent: self.size.height,
                cross_axis_direction: AxisDirection::Down,
                viewport_main_axis_extent: self.size.width,
                remaining_cache_extent: self.cache_extent,
                cache_origin: 0.0,
            },
            Axis::Vertical => SliverConstraints {
                axis_direction: AxisDirection::Down,
                growth_direction: GrowthDirection::Forward,
                scroll_direction: ScrollDirection::Idle,
                scroll_offset: 0.0,
                preceding_scroll_extent: 0.0,
                overlap: 0.0,
                remaining_paint_extent: self.size.height,
                cross_axis_extent: self.size.width,
                cross_axis_direction: AxisDirection::Right,
                viewport_main_axis_extent: self.size.height,
                remaining_cache_extent: self.cache_extent,
                cache_origin: 0.0,
            },
        };

        for child in self.children.iter_mut() {
            child.layout(&sliver_constraints);
        }
    }

    fn perform_resize(&mut self, constraints: &BoxConstraints) {
        todo!()
    }
}
