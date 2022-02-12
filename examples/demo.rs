use oxui::rendering::{BoxConstraints, Layout, RenderImage, RenderObject, Size};

fn main() {
    let mut image = RenderImage::from(Size::new(10.0, 20.0));
    image.layout(&BoxConstraints {
        min_width: 25.0,
        min_height: 25.0,
        max_width: 75.0,
        max_height: 75.0,
    });

    println!("{:?}", image.size());
}
