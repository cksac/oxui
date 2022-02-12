use crate::rendering::{
    BoxConstraints, BoxedRenderObject, BoxedRenderObjectBuilder, Height, Layout, RenderObject,
    Size, Width,
};
use crate::ui::Image;
use std::borrow::Borrow;

#[derive(Debug, Default)]
pub struct RenderImage {
    // render object
    pub(crate) size: Option<Size>,
    // render image
    pub(crate) image: Option<Image>,
    pub(crate) width: Option<f32>,
    pub(crate) height: Option<f32>,
}

impl From<Width> for RenderImage {
    fn from(w: Width) -> Self {
        RenderImage {
            image: None,
            width: w.0.into(),
            height: None,
            size: None,
        }
    }
}

impl From<Height> for RenderImage {
    fn from(h: Height) -> Self {
        RenderImage {
            image: None,
            width: None,
            height: h.0.into(),
            size: None,
        }
    }
}

impl<T> From<T> for RenderImage
where
    T: Into<Size>,
{
    fn from(s: T) -> Self {
        let size = s.into();
        RenderImage {
            image: None,
            width: size.width.into(),
            height: size.height.into(),
            size: None,
        }
    }
}

impl From<Image> for RenderImage {
    fn from(img: Image) -> Self {
        RenderImage {
            image: img.into(),
            width: None,
            height: None,
            size: None,
        }
    }
}

impl Layout<BoxConstraints> for RenderImage {
    fn perform_layout(&mut self, constraints: &BoxConstraints) {
        let constraints = BoxConstraints::tight_for(self.width, self.height).enforce(constraints);
        self.size = Some(match &self.image {
            Some(img) => constraints
                .borrow()
                .constrain_with_aspect_ratio((img.width() as f32, img.height() as f32)),
            None => constraints.borrow().smallest(),
        });
    }
}

impl RenderObject for RenderImage {
    fn size(&self) -> Option<Size> {
        self.size
    }
}

impl From<RenderImage> for BoxedRenderObject {
    fn from(img: RenderImage) -> Self {
        BoxedRenderObjectBuilder::new(img)
            .register_layout_for::<BoxConstraints>()
            .build()
    }
}
