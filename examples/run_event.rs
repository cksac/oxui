use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Instant;

use compose_rt::Recomposer;
use oxui::rendering::{Axis, FlexFit, Offset, PipelineOwner, Size};
use oxui::rendering::{PaintContext, RenderBox};
use oxui::widgets::{BuildContext, ConstrainedBox, Flex, Widget};
use skulpin::app::AppDrawArgs;
use skulpin::app::AppError;
use skulpin::app::AppHandler;
use skulpin::app::AppUpdateArgs;
use skulpin::app::{AppBuilder, MouseButton};
use skulpin::CoordinateSystem;
use skulpin::LogicalSize;

#[derive(Debug)]
pub struct RootWidget;
impl Widget for RootWidget {
    #[track_caller]
    fn create(&self, context: BuildContext) -> Rc<RefCell<dyn RenderBox>> {
        let state = context.state(|| Rc::new(RefCell::new(
            (2usize..57).chain((3usize..=58).rev()).cycle(),
        )));
        let count: usize = state.borrow_mut().next().unwrap();

        let mut children = Vec::new();
        for j in 1..=count {
            children.push({
                let v_state = context.state(|| Rc::new(RefCell::new(
                    (2usize..57).chain((3usize..=58).rev()).cycle(), //(2usize..4).cycle(),
                )));
                let v_count: usize = v_state.borrow_mut().next().unwrap();
                let mut children = Vec::new();
                for i in 1..=v_count {
                    children
                        .push(ConstrainedBox::default().into_flexible(i as usize, FlexFit::Loose))
                }

                Flex::builder()
                    .direction(Axis::Vertical)
                    .children(children)
                    .build()
                    .into_flexible(j, FlexFit::Loose)
            });
        }

        Flex::builder().children(children).build().create(context)
    }
}

struct App {
    recomposer: Recomposer,
    pipeline: PipelineOwner,
    previous_clicks: VecDeque<Offset>,
    previous_frame: Instant,
}

impl App {
    pub fn new<W>(width: u32, height: u32, root: W) -> Self
    where
        W: 'static + Widget,
    {
        App {
            recomposer: Recomposer::new(),
            pipeline: PipelineOwner::new(Size::new(width as f32, height as f32), root),
            previous_clicks: VecDeque::new(),
            previous_frame: Instant::now(),
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
        if update_args
            .input_state
            .is_mouse_just_down(MouseButton::Left)
        {
            let p = update_args.input_state.mouse_position();
            let position = Offset::new(p.x as f32, p.y as f32);
            self.pipeline.handle_event(position);
            self.previous_clicks.push_back(position);
        }
    }

    fn draw(&mut self, draw_args: AppDrawArgs) {
        // click to next frame
        if let Some(_) = self.previous_clicks.pop_front() {
            //if self.previous_frame.elapsed() > Duration::from_millis(100) {
            let canvas = draw_args.canvas;
            canvas.clear(0);

            let mut context = PaintContext::new(canvas);
            self.recomposer.compose(
                |cx| {
                    self.pipeline.draw_frame(cx, &mut context);
                }
            );
            self.previous_frame = draw_args.time_state.current_instant();
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
        .filter_level(log::LevelFilter::Warn)
        .filter_module("compose_rt", log::LevelFilter::Trace)
        .init();

    // Set up the coordinate system to be fixed at 900x600, and use this as the default window size
    // This means the drawing code can be written as though the window is always 900x600. The
    // output will be automatically scaled so that it's always visible.
    let logical_size = LogicalSize::new(900, 600);

    let app = App::new(logical_size.width, logical_size.height, RootWidget);

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
