use std::any::{type_name, TypeId};

use crate::{
    painting::{Axis, VerticalDirection},
    rendering::{BoxConstraints, Layout, Offset, RenderObject, Size},
    ui::{Clip, TextBaseline, TextDirection},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexFit {
    Tight,
    Loose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossAxisAlignment {
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainAxisAlignment {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainAxisSize {
    Min,
    Max,
}

pub struct RenderFlex {
    // RenderObject
    pub(crate) size: Option<Size>,

    // RenderFlex
    pub(crate) children: Vec<Flexible>,
    pub(crate) direction: Axis,
    pub(crate) main_axis_size: MainAxisSize,
    pub(crate) main_axis_alignment: MainAxisAlignment,
    pub(crate) cross_axis_alignment: CrossAxisAlignment,
    pub(crate) vertical_direction: VerticalDirection,
    pub(crate) text_direction: Option<TextDirection>,
    pub(crate) text_baseline: Option<TextBaseline>,
    pub(crate) clip_behavior: Clip,
    pub(crate) _overflow: f32,
}

impl Default for RenderFlex {
    fn default() -> Self {
        Self {
            size: None,
            children: Vec::new(),
            direction: Axis::Horizontal,
            main_axis_size: MainAxisSize::Max,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Center,
            vertical_direction: VerticalDirection::Down,
            text_direction: None,
            text_baseline: None,
            clip_behavior: Clip::None,
            _overflow: 0.0,
        }
    }
}

impl RenderFlex {
    pub fn with_direction(mut self, direction: Axis) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_text_direction(mut self, text_direction: impl Into<Option<TextDirection>>) -> Self {
        self.text_direction = text_direction.into();
        self
    }

    pub fn with_vertical_direction(mut self, vertical_direction: VerticalDirection) -> Self {
        self.vertical_direction = vertical_direction;
        self
    }

    pub fn with_child(mut self, child: impl Into<Flexible>) -> Self {
        self.children.push(child.into());
        self
    }
}

pub struct Flexible {
    pub(crate) offset: Offset,
    pub(crate) flex: usize,
    pub(crate) fit: FlexFit,
    pub(crate) inner: Box<dyn RenderObject>,
}

impl Flexible {
    pub fn new<T>(child: T, flex: usize, fit: FlexFit) -> Self
    where
        T: RenderObject + 'static,
    {
        Flexible {
            offset: Offset::zero(),
            flex,
            fit,
            inner: Box::new(child),
        }
    }
}

impl<T> From<T> for Flexible
where
    T: RenderObject + 'static,
{
    fn from(child: T) -> Self {
        Flexible {
            offset: Offset::zero(),
            flex: 0,
            fit: FlexFit::Tight,
            inner: Box::new(child),
        }
    }
}

struct LayoutSizes {
    main_size: f32,
    cross_size: f32,
    allocated_size: f32,
}

impl Size {
    fn main_size(&self, direction: Axis) -> f32 {
        match direction {
            Axis::Horizontal => self.width,
            Axis::Vertical => self.height,
        }
    }

    fn cross_size(&self, direction: Axis) -> f32 {
        match direction {
            Axis::Horizontal => self.height,
            Axis::Vertical => self.width,
        }
    }
}

impl RenderFlex {
    fn compute_sizes(&mut self, constraints: &BoxConstraints) -> LayoutSizes {
        // Determine used flex factor, size inflexible items, calculate free space.
        let mut total_flex = 0.0f32;
        let max_main_size = match self.direction {
            Axis::Horizontal => constraints.max_width,
            Axis::Vertical => constraints.max_height,
        };
        let can_flex = max_main_size < f32::INFINITY;

        let mut cross_size = 0.0f32;
        let mut allocated_size = 0.0f32;
        let mut last_flex_child_idx = 0;

        for (child_idx, child) in self.children.iter_mut().enumerate() {
            if child.flex > 0 {
                total_flex += child.flex as f32;
                last_flex_child_idx = child_idx;
            } else {
                let inner_constraints = match self.cross_axis_alignment {
                    CrossAxisAlignment::Stretch => match self.direction {
                        Axis::Horizontal => BoxConstraints::tight_for(constraints.max_height, None),
                        Axis::Vertical => BoxConstraints::tight_for(None, constraints.max_height),
                    },
                    _ => match self.direction {
                        Axis::Horizontal => {
                            BoxConstraints::default().with_max_height(constraints.max_height)
                        }
                        Axis::Vertical => {
                            BoxConstraints::default().with_max_width(constraints.max_width)
                        }
                    },
                };
                child.inner.layout(&inner_constraints);
                let child_size = child.inner.size().expect("has size after layout");
                allocated_size += child_size.main_size(self.direction);
                cross_size = cross_size.max(child_size.cross_size(self.direction));
            }
        }

        // Distribute free space to flexible children.
        let free_space = ((if can_flex { max_main_size } else { 0.0 }) - allocated_size).max(0.0);
        let mut allocated_flex_space = 0.0f32;
        if total_flex > 0.0 {
            let space_per_flex = if can_flex {
                free_space / total_flex
            } else {
                f32::NAN
            };

            for (child_idx, child) in self.children.iter_mut().enumerate() {
                if child.flex > 0 {
                    let max_child_extent = if can_flex {
                        if child_idx == last_flex_child_idx {
                            free_space - allocated_flex_space
                        } else {
                            space_per_flex * child.flex as f32
                        }
                    } else {
                        f32::INFINITY
                    };
                    let min_child_extend = match child.fit {
                        FlexFit::Tight => max_child_extent,
                        FlexFit::Loose => 0.0,
                    };
                    let inner_constraints = match self.cross_axis_alignment {
                        CrossAxisAlignment::Stretch => match self.direction {
                            Axis::Horizontal => BoxConstraints {
                                min_width: min_child_extend,
                                max_width: max_child_extent,
                                min_height: constraints.max_height,
                                max_height: constraints.max_height,
                            },
                            Axis::Vertical => BoxConstraints {
                                min_width: constraints.max_width,
                                max_width: constraints.max_width,
                                min_height: min_child_extend,
                                max_height: max_child_extent,
                            },
                        },
                        _ => match self.direction {
                            Axis::Horizontal => BoxConstraints {
                                min_width: min_child_extend,
                                max_width: max_child_extent,
                                min_height: 0.0,
                                max_height: constraints.max_height,
                            },
                            Axis::Vertical => BoxConstraints {
                                min_width: 0.0,
                                max_width: constraints.max_width,
                                min_height: min_child_extend,
                                max_height: max_child_extent,
                            },
                        },
                    };

                    child.inner.layout(&inner_constraints);
                    let child_size = child.inner.size().expect("has size after layout");

                    let child_main_size = child_size.main_size(self.direction);
                    allocated_size += child_main_size;
                    allocated_flex_space += child_main_size;
                    cross_size = cross_size.max(child_size.cross_size(self.direction));
                }
            }
        }

        let ideal_size = if can_flex && self.main_axis_size == MainAxisSize::Max {
            max_main_size
        } else {
            allocated_size
        };

        LayoutSizes {
            main_size: ideal_size,
            cross_size,
            allocated_size,
        }
    }
}

impl Layout<BoxConstraints> for RenderFlex {
    fn perform_layout(&mut self, constraints: &BoxConstraints) {
        let sizes = self.compute_sizes(constraints);
        let allocated_size = sizes.allocated_size;
        let mut actual_size = sizes.main_size;
        let mut cross_size = sizes.cross_size;

        let max_baseline_distance = 0.0f32;
        // TODO: handle baseline alignment
        // if self.cross_axis_alignment == CrossAxisAlignment::Baseline {
        //     let max_size_above_baseline = 0.0f32;
        //     let max_size_below_baseline = 0.0f32;
        //     for child in self.children.iter_mut() {
        //         debug_assert!(
        //             self.text_baseline.is_none(),
        //             "you must also specify which baseline to use"
        //         )
        //     }
        // }

        // Align items along the main axis.
        match self.direction {
            Axis::Horizontal => {
                let size = constraints.constrain((actual_size, cross_size));
                actual_size = size.width;
                cross_size = size.height;
                self.size = Some(size);
            }
            Axis::Vertical => {
                let size = constraints.constrain((cross_size, actual_size));
                actual_size = size.height;
                cross_size = size.width;
                self.size = Some(size);
            }
        }
        let actual_size_delta = actual_size - allocated_size;
        self._overflow = (-actual_size_delta).max(0.0);
        let remaining_space = actual_size.max(0.0);

        let children_count = self.children.len();
        let (leading_space, between_space) = match self.main_axis_alignment {
            MainAxisAlignment::Start => (0.0, 0.0),
            MainAxisAlignment::End => (remaining_space, 0.0),
            MainAxisAlignment::Center => (remaining_space / 2.0, 0.0),
            MainAxisAlignment::SpaceBetween => (
                0.0,
                if children_count > 1 {
                    (children_count - 1) as f32
                } else {
                    0.0
                },
            ),
            MainAxisAlignment::SpaceAround => {
                let between_space = if children_count > 1 {
                    remaining_space / children_count as f32
                } else {
                    0.0
                };
                let leading_space = between_space / 2.0;
                (leading_space, between_space)
            }
            MainAxisAlignment::SpaceEvenly => {
                let between_space = if children_count > 1 {
                    remaining_space / (children_count + 1) as f32
                } else {
                    0.0
                };
                (between_space, between_space)
            }
        };

        let flip_main_axis =
            start_is_top_left(self.direction, self.text_direction, self.vertical_direction)
                .unwrap_or(true);

        // Position elements
        let mut child_main_position = if flip_main_axis {
            actual_size - leading_space
        } else {
            leading_space
        };
        for child in self.children.iter_mut() {
            let child_size = child.inner.size().expect("has size after layout");

            let child_cross_position = match self.cross_axis_alignment {
                CrossAxisAlignment::Start | CrossAxisAlignment::End => {
                    if start_is_top_left(
                        self.direction.flip(),
                        self.text_direction,
                        self.vertical_direction,
                    )
                    .unwrap_or(false)
                        == (self.cross_axis_alignment == CrossAxisAlignment::Start)
                    {
                        0.0
                    } else {
                        cross_size - child_size.cross_size(self.direction)
                    }
                }
                CrossAxisAlignment::Center => cross_size - child_size.cross_size(self.direction),
                CrossAxisAlignment::Stretch => 0.0,
                CrossAxisAlignment::Baseline => {
                    match self.direction {
                        Axis::Horizontal => {
                            // TODO: child.getDistanceToBaseline
                            let distance: Option<f32> = None;
                            match distance {
                                Some(d) => max_baseline_distance - d,
                                None => 0.0,
                            }
                        }
                        Axis::Vertical => 0.0,
                    }
                }
            };

            if flip_main_axis {
                child_main_position = child_size.main_size(self.direction);
            }

            child.offset = match self.direction {
                Axis::Horizontal => Offset::new(child_main_position, child_cross_position),
                Axis::Vertical => Offset::new(child_cross_position, child_main_position),
            };

            if flip_main_axis {
                child_main_position -= between_space;
            } else {
                child_main_position += child_size.main_size(self.direction) + between_space;
            }
        }
    }
}

impl RenderObject for RenderFlex {
    fn ty_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn ty_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn size(&self) -> Option<Size> {
        self.size
    }
}

fn start_is_top_left(
    direction: Axis,
    text_direction: impl Into<Option<TextDirection>>,
    vertical_direction: impl Into<Option<VerticalDirection>>,
) -> Option<bool> {
    match direction {
        Axis::Horizontal => match text_direction.into() {
            Some(TextDirection::LTR) => Some(true),
            Some(TextDirection::RTL) => Some(false),
            None => None,
        },
        Axis::Vertical => match vertical_direction.into() {
            Some(VerticalDirection::Down) => Some(true),
            Some(VerticalDirection::Up) => Some(false),
            None => None,
        },
    }
}
