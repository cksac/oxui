use crate::rendering::{BoxConstraints, RenderBox, RenderImage, Size};
use crate::tests::common::create_image;

#[test]
fn test_square_image() {
    let mut image = create_image(10, 10);

    image.layout(
        &BoxConstraints {
            min_width: 25.0,
            min_height: 25.0,
            max_width: 100.0,
            max_height: 100.0,
        },
        false,
    );
    assert_eq!(image.size(), Size::new(25.0, 25.0));

    image.layout(
        &BoxConstraints {
            min_width: 4.0,
            min_height: 4.0,
            max_width: 8.0,
            max_height: 8.0,
        },
        false,
    );
    assert_eq!(image.size(), Size::new(8.0, 8.0));
}

#[test]
fn test_wide_image() {
    let mut image = create_image(20, 10);

    image.layout(
        &BoxConstraints {
            min_width: 5.0,
            min_height: 30.0,
            max_width: 100.0,
            max_height: 100.0,
        },
        false,
    );
    assert_eq!(image.size(), Size::new(60.0, 30.0));

    image.layout(
        &BoxConstraints {
            min_width: 5.0,
            min_height: 5.0,
            max_width: 100.0,
            max_height: 100.0,
        },
        false,
    );
    assert_eq!(image.size(), Size::new(20.0, 10.0));

    image.layout(
        &BoxConstraints {
            min_width: 20.0,
            min_height: 20.0,
            max_width: 30.0,
            max_height: 30.0,
        },
        false,
    );
    assert_eq!(image.size(), Size::new(30.0, 20.0));
}

#[test]
fn test_tall_image() {
    let mut image = create_image(10, 20);

    image.layout(
        &BoxConstraints {
            min_width: 50.0,
            min_height: 5.0,
            max_width: 75.0,
            max_height: 75.0,
        },
        false,
    );
    assert_eq!(image.size(), Size::new(50.0, 75.0));

    image.layout(
        &BoxConstraints {
            min_width: 5.0,
            min_height: 5.0,
            max_width: 16.0,
            max_height: 16.0,
        },
        false,
    );
    assert_eq!(image.size(), Size::new(8.0, 16.0));

    image.layout(
        &BoxConstraints {
            min_width: 20.0,
            min_height: 20.0,
            max_width: 30.0,
            max_height: 30.0,
        },
        false,
    );
    assert_eq!(image.size(), Size::new(20.0, 30.0));
}

#[test]
fn test_none_image() {
    let mut image = RenderImage::default();
    image.layout(
        &BoxConstraints {
            min_width: 25.0,
            min_height: 25.0,
            max_width: 100.0,
            max_height: 100.0,
        },
        false,
    );
    assert_eq!(image.size(), Size::new(25.0, 25.0));

    let mut image = RenderImage::new(50.0, None);
    image.layout(
        &BoxConstraints {
            min_width: 25.0,
            min_height: 25.0,
            max_width: 100.0,
            max_height: 100.0,
        },
        false,
    );
    assert_eq!(image.size(), Size::new(50.0, 25.0));

    let mut image = RenderImage::new(None, 50.0);
    image.layout(
        &BoxConstraints {
            min_width: 25.0,
            min_height: 25.0,
            max_width: 100.0,
            max_height: 100.0,
        },
        false,
    );
    assert_eq!(image.size(), Size::new(25.0, 50.0));

    let mut image = RenderImage::new(100.0, 100.0);
    image.layout(
        &BoxConstraints {
            min_width: 25.0,
            min_height: 25.0,
            max_width: 75.0,
            max_height: 75.0,
        },
        false,
    );
    assert_eq!(image.size(), Size::new(75.0, 75.0));
}
