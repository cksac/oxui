use crate::{rendering::RenderImage, ui::Image};
use skia_safe::Surface;

pub fn create_image(width: i32, height: i32) -> RenderImage {
    let mut surface = Surface::new_raster_n32_premul((width, height)).unwrap();
    let image = Image {
        inner: surface.image_snapshot(),
    };
    RenderImage::from(image)
}
