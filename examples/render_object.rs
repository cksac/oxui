use std::any::{type_name, TypeId};

use oxui::rendering::{
    register_layout_vt, BoxConstraints, Layout, RenderImage, RenderObject, Size,
};

#[derive(Debug, Default)]
pub struct RenderFoo;

impl Layout<BoxConstraints> for RenderFoo {
    fn perform_layout(&mut self, constraints: &BoxConstraints) {
        println!("perform_layout {:#?}", constraints);
    }
}

impl RenderObject for RenderFoo {
    fn ty_id(&self) -> std::any::TypeId {
        TypeId::of::<Self>()
    }

    fn ty_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn size(&self) -> Option<Size> {
        None
    }
}

fn main() {
    register_layout_vt::<BoxConstraints, RenderFoo>();

    let mut img: Box<dyn RenderObject> = Box::new(RenderImage::default());
    img.layout(&BoxConstraints {
        min_width: 25.0,
        min_height: 25.0,
        max_width: 75.0,
        max_height: 75.0,
    });
    println!("{:#?}", img.size());

    let foo: &mut dyn RenderObject = &mut RenderFoo;
    foo.layout(&BoxConstraints {
        min_width: 25.0,
        min_height: 25.0,
        max_width: 75.0,
        max_height: 75.0,
    });

    let mut foo_box: Box<dyn RenderObject> = Box::new(RenderFoo::default());
    foo_box.layout(&BoxConstraints {
        min_width: 25.0,
        min_height: 25.0,
        max_width: 75.0,
        max_height: 75.0,
    });
}
