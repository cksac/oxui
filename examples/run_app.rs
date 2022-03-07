use std::collections::VecDeque;

use oxui::rendering::PaintContext;
use oxui::rendering::{FlexFit, PipelineOwner, Size};
use oxui::widgets::{ConstrainedBox, Element, Flex, Widget};
use skulpin::app::{AppBuilder, MouseButton};
use skulpin::app::AppDrawArgs;
use skulpin::app::AppError;
use skulpin::app::AppHandler;
use skulpin::app::AppUpdateArgs;
use skulpin::CoordinateSystem;
use skulpin::LogicalSize;

#[topo::nested]
fn root() -> Element {
    let (count, count_mut) = moxie::state(|| 0usize);
    let mut children = Vec::new();
    
    for i in 1..=*count+1 {
        children.push(ConstrainedBox::default().into_flexible(i, FlexFit::Loose))
    }

    if *count < 10 {
        count_mut.set(*count + 1);
    }

    Flex::builder().children(children).build().create()
}

struct App {
    pipeline: PipelineOwner,
    previous_clicks: VecDeque<bool>,
}

impl App {
    pub fn new(width: u32, height: u32, root: fn() -> Element) -> Self {
        App {
            pipeline: PipelineOwner::new(Size::new(width as f32, height as f32), root),
            previous_clicks: VecDeque::new()
        }
    }
}

impl AppHandler for App {
    fn update(&mut self, update_args: AppUpdateArgs) {
        // let input_state = update_args.input_state;
        // let app_control = update_args.app_control;

        // if input_state.is_key_down(VirtualKeyCode::Escape) {
        //     app_control.enqueue_terminate_process();
        // }
        if update_args.input_state.is_mouse_just_down(MouseButton::Left) {
            self.previous_clicks.push_back(true);
        }
    }

    fn draw(&mut self, draw_args: AppDrawArgs) {
        // click to next frame
        if let Some(_) = self.previous_clicks.pop_front() {
            let canvas = draw_args.canvas;
            canvas.clear(0);
    
            let mut context = PaintContext::new(canvas);
            self.pipeline.draw_frame(&mut context);
        }
    }

    fn fatal_error(&mut self, error: &AppError) {
        println!("{}", error);
    }
}

// This example shows how to use the "app" helpers to get a window open and drawing with minimal code
// It's not as flexible as working with winit directly, but it's quick and simple
fn main() {
    // Setup logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Set up the coordinate system to be fixed at 900x600, and use this as the default window size
    // This means the drawing code can be written as though the window is always 900x600. The
    // output will be automatically scaled so that it's always visible.
    let logical_size = LogicalSize::new(900, 600);

    let app = App::new(logical_size.width, logical_size.height, root);

    let visible_range = skulpin::skia_safe::Rect {
        left: 0.0,
        right: logical_size.width as f32,
        top: 0.0,
        bottom: logical_size.height as f32,
    };
    let scale_to_fit = skulpin::skia_safe::matrix::ScaleToFit::Center;

    AppBuilder::new()
        .inner_size(logical_size)
        .coordinate_system(CoordinateSystem::VisibleRange(visible_range, scale_to_fit))
        .run(app);
}
