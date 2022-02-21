use crate::{
    rendering::{Axis, RenderBox, RenderSliverList},
    tests::common::RenderSizedBox,
};

#[test]
fn test_sliver_list() {
    let list = RenderSliverList::new(Axis::Vertical, 200.0)
        .with_child(RenderSizedBox::new(100.0, 400.0).into_sliver())
        .with_child(RenderSizedBox::new(100.0, 400.0).into_sliver())
        .with_child(RenderSizedBox::new(100.0, 400.0).into_sliver())
        .with_child(RenderSizedBox::new(100.0, 400.0).into_sliver())
        .with_child(RenderSizedBox::new(100.0, 400.0).into_sliver());
}
