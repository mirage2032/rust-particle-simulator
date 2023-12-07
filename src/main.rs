use ggez::{conf, event};
use crate::psim::visualizer::Visualizer;

mod psim;

fn main() {
    let cb = ggez::ContextBuilder::new("window1", "author1")
        .window_setup(conf::WindowSetup::default().title("Window 1"))
        .window_mode(conf::WindowMode::default().dimensions(1000 as f32, 1000 as f32));
    let visualizer = Visualizer::new(1000,700,0.00000000001).unwrap();
    let (ctx, event_loop) = cb.build().unwrap();
    event::run(ctx, event_loop, visualizer);
}

