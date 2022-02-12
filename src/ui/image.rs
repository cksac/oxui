use skia_safe::image::Image as SkImage;

#[derive(Debug)]
pub struct Image {
    pub(crate) inner: SkImage,
}

impl Image {
    pub fn width(&self) -> i32 {
        self.inner.width()
    }

    pub fn height(&self) -> i32 {
        self.inner.height()
    }
}
