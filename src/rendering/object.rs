use std::{any::TypeId, cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    rendering::{BoxConstraints, Offset, RenderBox, Size},
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

pub trait RenderObject: Debug {
    fn ty_id(&self) -> TypeId;

    fn ty_name(&self) -> &'static str;

    fn paint(&self, context: &mut PaintContext, offset: Offset);
}

pub struct PipelineOwner {
    size: Size,
    root: Box<dyn Widget>,
    build_context: BuildContext,
}

impl PipelineOwner {
    pub fn new<T>(size: Size, root: T) -> Self
    where
        T: 'static + Widget,
    {
        let root = Box::new(root);
        let build_context = BuildContext::new();
        PipelineOwner {
            size,
            root,
            build_context,
        }
    }

    pub fn draw_frame(&mut self, context: &mut PaintContext) {
        // build render tree;
        self.build_context.reset_index();
        let tree = self.root.create(&self.build_context);
        //println!("{:#?}", tree);
        //println!("{:#?}", self.context);

        self.flush_layout(tree.clone());
        self.flush_paint(tree, context);
    }

    pub fn flush_layout(&mut self, tree: Rc<RefCell<dyn RenderBox>>) {
        let ref constraints = BoxConstraints::tight(self.size);
        tree.borrow_mut().layout(constraints, false)
    }

    pub fn flush_paint(&mut self, tree: Rc<RefCell<dyn RenderBox>>, context: &mut PaintContext) {
        tree.borrow_mut().paint(context, Offset::zero());
    }
}
