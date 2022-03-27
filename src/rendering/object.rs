use std::any::TypeId;

use compose_rt::Composer;

use crate::{
    gestures::HitTestTarget,
    rendering::{BoxConstraints, Element, Offset, Size},
    widgets::{BuildContext, Widget},
};
use std::fmt::Debug;

pub struct PaintContext<'a> {
    pub canvas: &'a mut skia_safe::Canvas,
}

impl<'a> PaintContext<'a> {
    pub fn new(canvas: &'a mut skia_safe::Canvas) -> Self {
        PaintContext { canvas }
    }

    pub fn draw_rect(&mut self, offset: Offset, size: Size) {
        let mut paint = skia_safe::Paint::new(skia_safe::Color4f::new(1.0, 1.0, 1.0, 1.0), None);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Stroke);
        paint.set_stroke_width(2.0);

        self.canvas.draw_rect(
            skia_safe::Rect {
                left: offset.x,
                top: offset.y,
                right: offset.x + size.width,
                bottom: offset.y + size.height,
            },
            &paint,
        );
    }
}

pub trait RenderObject: Debug + HitTestTarget {
    fn ty_id(&self) -> TypeId;

    fn ty_name(&self) -> &'static str;

    fn paint(&self, context: &mut PaintContext, offset: Offset);
}

pub struct PipelineOwner {
    size: Size,
    root_fn: Box<dyn Fn(&mut Composer) -> Element>,
    render_view: Option<Element>,
}

impl PipelineOwner {
    pub fn new<T>(size: Size, root: T) -> Self
    where
        T: 'static + Widget,
    {
        let root_fn = Box::new(move |cx: BuildContext| root.create(cx));
        PipelineOwner {
            size,
            root_fn,
            render_view: None,
        }
    }

    pub fn handle_event(&mut self, position: Offset) {
        if let Some(view) = &mut self.render_view {
            view.borrow().hit_test(position);
        }
    }

    pub fn draw_frame(&mut self, cx: &mut Composer, context: &mut PaintContext) {
        // re-build render tree;
        self.render_view = Some((self.root_fn)(cx));

        //println!("{:#?}", self.render_view);
        //println!("{:#?}", self.context);

        self.flush_layout();
        self.flush_paint(context);
    }

    pub fn flush_layout(&mut self) {
        if let Some(view) = &mut self.render_view {
            let ref constraints = BoxConstraints::tight(self.size);
            view.borrow_mut().layout(constraints, false)
        }
    }

    pub fn flush_paint(&mut self, context: &mut PaintContext) {
        if let Some(view) = &mut self.render_view {
            view.borrow_mut().paint(context, Offset::zero());
        }
    }
}
