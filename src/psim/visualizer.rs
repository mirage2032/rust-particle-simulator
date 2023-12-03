use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::psim::simulator::forcefield::{ForceField, Shape};
use crate::psim::simulator::particle::Particle;
use crate::psim::simulator::psim::PSim;
use crate::psim::simulator::vector2::Vector2;

const DEFAULT_PARTICLE_RADIUS: f64 = 2.0;
const DEFAULT_PARTICLE_MASS: f64 = 1500.0;
const DEFAULT_PARTICLE_VELOCITY: Vector2 = Vector2 { x: 0.0, y: 0.0 };

const DEFAULT_BIG_PARTICLE_RADIUS: f64 = 100.0;
const DEFAULT_BIG_PARTICLE_MASS: f64 = 5.972 * 1e24;
const DEFAULT_GRAVITY_RADIUS: f64 = 40.0;
const DEFAULT_GRAVITY_MASS: f64 = 8.0 * 1e16;

const COLOR_BACKGROUND: Color = Color::RGB(20, 20, 20);
const COLOR_PARTICLE: Color = Color::RGB(127, 180, 99);
const COLOR_FORCE_FIELD: Color = Color::RGB(38, 64, 200);

fn random_draw_color() -> Color {
    let r = rand::random::<u8>();
    let g = rand::random::<u8>();
    let b = rand::random::<u8>();
    Color::RGB(r, g, b)
}

enum Event {
    Quit,
    Reset,
    AddParticle { particle: Particle },
    AddForceField { force_field: ForceField },
    Delete { position: Vector2 },
}

pub struct Visualizer {
    simulator: PSim,
    canvas: WindowCanvas,
    event_pump: sdl2::EventPump,
    running: bool,
}

impl Visualizer {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Particle Simulator", width, height)
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        Visualizer {
            simulator: PSim::new(),
            canvas,
            event_pump,
            running: true,
        }
    }

    fn get_events(&mut self) -> Vec<Event> {
        let mut events: Vec<Event> = vec![];
        let mouse_state = self.event_pump.mouse_state();
        let x = mouse_state.x();
        let y = mouse_state.y();
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    events.push(
                        Event::Quit
                    )
                }
                //Add Particle
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::P), .. } => {
                    events.push(Event::AddParticle {
                        particle: Particle::new(
                            Vector2::new(x as f64, y as f64),
                            DEFAULT_PARTICLE_VELOCITY,
                            DEFAULT_PARTICLE_MASS,
                            DEFAULT_PARTICLE_RADIUS
                        )
                    });
                }
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::O), .. } => {
                    events.push(Event::AddParticle {
                        particle: Particle::new(
                            Vector2::new(x as f64, y as f64),
                            Vector2::zero(),
                            DEFAULT_BIG_PARTICLE_MASS,
                            DEFAULT_BIG_PARTICLE_RADIUS
                        )
                    });
                }
                //Add gravity field
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::G), .. } => {
                    events.push(Event::AddParticle {
                        particle: Particle::new(
                            Vector2::new(x as f64, y as f64),
                            Vector2::zero(),
                            DEFAULT_GRAVITY_MASS,
                            DEFAULT_GRAVITY_RADIUS,
                        )
                    });

                }
                //Reset
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::R), .. } => {
                    events.push(Event::Reset)
                }
                //Delete
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), .. } => {
                    events.push(Event::Delete {
                        position: Vector2::new(x as f64, y as f64),
                    })
                }
                _ => {}
            }
        }
        events
    }

    fn handle_events(&mut self) {
        let events = self.get_events();
        for event in events {
            match event {
                Event::AddParticle { particle } => {
                    self.add_particle(particle);
                }
                Event::AddForceField { force_field } => {
                    self.add_force_field(force_field);
                }
                Event::Delete { position } => {
                    self.simulator.particles.retain(|particle| {
                        particle.get_pos().distance(&position) > particle.get_radius()
                    });
                    self.simulator.force_fields.retain(|force_field| {
                        let force_field_pos: &Vector2 = force_field.get_pos();
                        match force_field.get_shape() {
                            Shape::Circle { radius } => {
                                force_field_pos.distance(&position) > *radius
                            }
                            Shape::Rectangle { width, height } => {
                                !((force_field_pos.x >= position.x - (width / 2.0) && force_field_pos.x <= position.x + (width / 2.0)) &&
                                    (force_field_pos.y >= position.y - (height / 2.0) && force_field_pos.y <= position.y + (height / 2.0)))
                            }
                        }
                    })
                }
                Event::Quit => {
                    self.running = false;
                }
                Event::Reset => {
                    self.simulator = PSim::new();
                }
            }
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.simulator.add_particle(particle);
    }

    pub fn add_force_field(&mut self, force_field: ForceField) {
        self.simulator.add_force_field(force_field);
    }

    fn draw(&mut self) {
        //clear screen
        self.canvas.set_draw_color(COLOR_BACKGROUND);
        self.canvas.clear();

        //draw force fields
        self.simulator.get_force_fields().iter().for_each(|force_field| {
            let pos = force_field.get_pos();
            match &force_field.get_shape() {
                Shape::Circle { radius } => {
                    self.canvas.aa_circle(pos.x as i16, pos.y as i16, *radius as i16, COLOR_FORCE_FIELD).unwrap();
                }
                Shape::Rectangle { width, height } => {
                    let rectangle = sdl2::rect::Rect::new(
                        (pos.x - (width / 2.0)) as i32,
                        (pos.y - (height / 2.0)) as i32,
                        *width as u32,
                        *height as u32,
                    );
                    self.canvas.set_draw_color(COLOR_FORCE_FIELD);
                    self.canvas.fill_rect(rectangle).unwrap();
                }
            }
        });

        //draw particle
        self.simulator.get_particles().iter().for_each(|particle| {
            let pos = particle.get_pos();
            self.canvas.aa_circle(pos.x as i16, pos.y as i16, particle.get_radius() as i16, COLOR_PARTICLE).unwrap();
        });
        self.canvas.present();
    }

    fn clean(&mut self) {
        let canvas_size = self.canvas.output_size().unwrap();
        //remove particles out of bounds
        self.simulator.particles.retain(|particle| {
            let pos = particle.get_pos();
            pos.x >= 0.0 && pos.x <= canvas_size.0 as f64 && pos.y >= 0.0 && pos.y <= canvas_size.1 as f64
        })
    }

    fn step(&mut self, dt: f64) {
        self.clean();
        self.handle_events();
        self.draw();
        self.simulator.step(dt);
    }
    
    pub fn run(&mut self) {
        self.running = true;
        let mut time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64();
        let mut dt: f64 = 0.0;
        while self.running {
            self.step(dt);
            let new_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64();
            dt = new_time - time;
            time = new_time;
        }
    }

    pub fn run_constant_time(&mut self, dt: f64) {
        self.running = true;
        while self.running {
            self.step(dt);
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}