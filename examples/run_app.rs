use oxui::rendering::PaintContext;
use oxui::rendering::{FlexFit, PipelineOwner, Size};
use oxui::widgets::{ConstrainedBox, Element, Flex, Widget};
use skulpin::app::AppBuilder;
use skulpin::app::AppDrawArgs;
use skulpin::app::AppError;
use skulpin::app::AppHandler;
use skulpin::app::AppUpdateArgs;
use skulpin::CoordinateSystem;
use skulpin::LogicalSize;

fn root() -> Element {
    Flex::builder()
        .children(vec![
            ConstrainedBox::default().into_flexible(1, FlexFit::Loose),
            ConstrainedBox::default().into_flexible(2, FlexFit::Loose),
        ])
        .build()
        .create()
}

struct App {
    pipeline: PipelineOwner,
}

impl App {
    pub fn new(width: u32, height: u32, root: fn() -> Element) -> Self {
        App {
            pipeline: PipelineOwner::new(Size::new(width as f32, height as f32), root),
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
    }

    fn draw(&mut self, draw_args: AppDrawArgs) {
        let canvas = draw_args.canvas;
        let mut context = PaintContext::new(canvas);
        self.pipeline.draw_frame(&mut context);
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
        .filter_level(log::LevelFilter::Debug)
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
