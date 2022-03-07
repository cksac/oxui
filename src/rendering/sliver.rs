use std::{cell::RefCell, rc::Rc};

use crate::rendering::{Axis, AxisDirection, BoxConstraints, RenderBox, ScrollDirection};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrowthDirection {
    Forward,
    Reverse,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SliverConstraints {
    pub axis_direction: AxisDirection,
    pub growth_direction: GrowthDirection,
    pub scroll_direction: ScrollDirection,
    pub scroll_offset: f32,
    pub preceding_scroll_extent: f32,
    pub overlap: f32,
    pub remaining_paint_extent: f32,
    pub cross_axis_extent: f32,
    pub cross_axis_direction: AxisDirection,
    pub viewport_main_axis_extent: f32,
    pub remaining_cache_extent: f32,
    pub cache_origin: f32,
}

impl SliverConstraints {
    pub fn axis(&self) -> Axis {
        self.axis_direction.into()
    }

    pub fn calculate_paint_offset(&self, from: f32, to: f32) -> f32 {
        debug_assert!(from <= to);
        let a = self.scroll_offset;
        let b = self.scroll_offset + self.remaining_paint_extent;
        (to.clamp(a, b) - from.clamp(a, b)).clamp(0.0, self.remaining_paint_extent)
    }

    pub fn calculate_cache_offset(&self, from: f32, to: f32) -> f32 {
        debug_assert!(from <= to);
        let a = self.scroll_offset + self.cache_origin;
        let b = self.scroll_offset + self.remaining_cache_extent;
        (to.clamp(a, b) - from.clamp(a, b)).clamp(0.0, self.remaining_cache_extent)
    }
}

impl<'a> From<&'a SliverConstraints> for BoxConstraints {
    fn from(s: &'a SliverConstraints) -> Self {
        match s.axis() {
            Axis::Horizontal => BoxConstraints {
                min_width: 0.0,
                max_width: f32::INFINITY,
                min_height: s.cross_axis_extent,
                max_height: s.cross_axis_extent,
            },
            Axis::Vertical => BoxConstraints {
                min_width: s.cross_axis_extent,
                max_width: s.cross_axis_extent,
                min_height: 0.0,
                max_height: f32::INFINITY,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SliverGeometry {
    pub scroll_extent: f32,
    pub paint_origin: f32,
    pub paint_extent: f32,
    pub layout_extent: f32,
    pub max_paint_extent: f32,
    pub max_scroll_obstruction_extent: f32,
    pub visible: bool,
    pub has_visual_overflow: bool,
    pub cache_extent: f32,
    pub hit_test_extent: f32,
}

pub trait RenderSliver {
    fn layout(&mut self, constraints: &SliverConstraints) {
        self.perform_layout(constraints)
    }

    fn perform_layout(&mut self, constraints: &SliverConstraints);

    fn perform_resize(&mut self, constraints: &SliverConstraints);

    fn geometry(&self) -> &SliverGeometry;
}

pub struct RenderSliverToBoxAdapter {
    geometry: SliverGeometry,
    child: Rc<RefCell<dyn RenderBox>>,
}

impl RenderSliver for RenderSliverToBoxAdapter {
    fn perform_layout(&mut self, constraints: &SliverConstraints) {
        let ref box_constraints = constraints.into();
        self.child.borrow_mut().layout(box_constraints, true);

        let child_size = self.child.borrow().size();
        let child_extent = match constraints.axis() {
            Axis::Horizontal => child_size.width,
            Axis::Vertical => child_size.height,
        };

        let painted_child_size = constraints.calculate_paint_offset(0.0, child_extent);
        let cache_extent = constraints.calculate_cache_offset(0.0, child_extent);
        let has_visual_overflow =
            child_extent > constraints.remaining_paint_extent || constraints.scroll_offset > 0.0;

        self.geometry = SliverGeometry {
            scroll_extent: child_extent,
            paint_origin: 0.0,
            paint_extent: painted_child_size,
            layout_extent: painted_child_size,
            max_paint_extent: child_extent,
            max_scroll_obstruction_extent: 0.0,
            visible: painted_child_size > 0.0,
            has_visual_overflow: has_visual_overflow,
            cache_extent: cache_extent,
            hit_test_extent: painted_child_size,
        };
    }

    fn perform_resize(&mut self, constraints: &SliverConstraints) {
        todo!()
    }

    fn geometry(&self) -> &SliverGeometry {
        todo!()
    }
}
