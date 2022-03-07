use std::any::TypeId;

use moxie::runtime::RunLoop;

use crate::{
    rendering::{BoxConstraints, Offset, RenderBox, Size},
    widgets::Element,
};

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

pub trait RenderObject {
    fn ty_id(&self) -> TypeId;

    fn ty_name(&self) -> &'static str;

    fn paint(&self, context: &mut PaintContext, offset: Offset);
}

pub struct PipelineOwner {
    size: Size,
    rt: RunLoop<fn() -> Element>,
}

impl PipelineOwner {
    pub fn new(size: Size, root: fn() -> Element) -> Self {
        let rt = RunLoop::new(root);
        PipelineOwner { size, rt }
    }

    pub fn draw_frame(&mut self, context: &mut PaintContext) {
        // build render tree;
        let mut tree = self.rt.run_once();

        self.flush_layout(&mut tree);
        self.flush_paint(&mut tree, context);
    }

    pub fn flush_layout(&mut self, tree: &mut Element) {
        let ref constraints = BoxConstraints::tight(self.size);
        tree.inner.borrow_mut().layout(constraints, false)
    }

    pub fn flush_paint(&mut self, tree: &mut Element, context: &mut PaintContext) {
        tree.inner.borrow_mut().paint(context, Offset::zero());
    }
}
