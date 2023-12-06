use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use sdl2::event::Event;
use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::psim::visualizer::VisEvent;

const COLOR_BACKGROUND: Color = Color::RGB(20, 20, 20);
const COLOR1: Color = Color::RGB(120, 250, 12);
const COLOR2: Color = Color::RGB(199, 99, 182);

struct Config {
    particle_small_radius: f64,
    particle_small_mass: f64,
    particle_medium_radius: f64,
    particle_medium_mass: f64,
    particle_big_radius: f64,
    particle_big_mass: f64,
}

impl Config {
    pub fn new() -> Self {
        Config {
            particle_small_radius: 2.0,
            particle_small_mass: 1500.0,
            particle_medium_radius: 10.0,
            particle_medium_mass: 8.0 * 1e16,
            particle_big_radius: 5.0,
            particle_big_mass: 5.972 * 1e24,
        }
    }
}

pub struct ConfigWindow<> {
    config: Config,
}

impl ConfigWindow {
    pub fn new() -> Self {
        ConfigWindow {
            config: Config::new(),
        }
    }
    pub fn run(&mut self,sdl_context_rc: Rc<RefCell<Sdl>>) {
        let sdl_context = sdl_context_rc.borrow();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Particle Simulator Config", 1000, 500)
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(50,50,50));
        loop {
            canvas.clear();
            canvas.present();
        }
    }
}
