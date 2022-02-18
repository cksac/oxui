use crate::{
    rendering::{BoxConstraints, Layout, RenderConstrainedBox, RenderFlex, RenderObject, Size},
    ui::TextDirection,
};

#[test]
fn test_over_constrained() {
    let b = RenderConstrainedBox::new(BoxConstraints::tight((10.0, 10.0)));

    let mut flex = RenderFlex::default()
        .with_text_direction(TextDirection::LTR)
        .with_child(b);

    flex.layout(&BoxConstraints {
        min_width: 200.0,
        max_width: 200.0,
        min_height: 200.0,
        max_height: 200.0,
    });
    assert_eq!(flex.size(), Some(Size::new(200.0, 200.0)));
}
