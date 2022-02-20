use crate::rendering::{
    Axis, BoxConstraints, FlexFit, Flexible, RenderBox, RenderConstrainedBox, RenderFlex, Size,
    TextDirection,
};

#[test]
fn test_over_constrained() {
    let b = RenderConstrainedBox::new(BoxConstraints::tight((10.0, 10.0)));

    let mut flex = RenderFlex::default()
        .with_text_direction(TextDirection::LTR)
        .with_child(b);

    flex.layout(
        &BoxConstraints {
            min_width: 200.0,
            max_width: 200.0,
            min_height: 200.0,
            max_height: 200.0,
        },
        false,
    );
    assert_eq!(flex.size(), Size::new(200.0, 200.0));
}

#[test]
fn test_vertical_overflow() {
    let mut flex = RenderFlex::default()
        .with_direction(Axis::Vertical)
        .with_child(RenderConstrainedBox::new(BoxConstraints::tight_for(
            None, 200.0,
        )))
        .with_child(Flexible::new(
            RenderConstrainedBox::new(BoxConstraints::expand(None, None)),
            1,
            FlexFit::Loose,
        ));

    let viewport = BoxConstraints::default()
        .with_max_height(100.0)
        .with_max_width(100.0);

    flex.layout(&viewport, false);

    assert_eq!(flex.size(), Size::new(100.0, 100.0));
    assert_eq!(flex.children[1].inner.size(), Size::new(100.0, 0.0));
}
