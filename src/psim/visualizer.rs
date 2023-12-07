use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::glam::{vec2, Vec2};
use ggez::graphics::{Canvas, Color, Rect, Text, TextFragment, PxScale, Drawable};
use ggez::input::keyboard::{KeyCode, KeyInput};
use crate::psim::simulator::forcefield::{ForceField, Shape};
use crate::psim::simulator::particle::Particle;
use crate::psim::simulator::psim::PSim;

const DEFAULT_PARTICLE_RADIUS: f64 = 10.0;
const DEFAULT_PARTICLE_MASS: f64 = 1500.0;
const DEFAULT_PARTICLE_VELOCITY: Vec2 = Vec2 { x: 280.0, y: 0.0 };

const DEFAULT_BIG_PARTICLE_RADIUS: f64 = 100.0;
const DEFAULT_BIG_PARTICLE_MASS: f64 = 20.0 * 1e14;
const DEFAULT_GRAVITY_RADIUS: f64 = 40.0;
const DEFAULT_GRAVITY_MASS: f64 = 8.0 * 1e13;

const COLOR_BACKGROUND: Color = Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 };
const COLOR_PARTICLE: Color = Color { r: 0.9, g: 0.9, b: 0.6, a: 1.0 };
const COLOR_FORCE_FIELD: Color = Color { r: 0.2, g: 0.5, b: 0.9, a: 1.0 };

pub enum VisEvent {
    Quit,
    Reset,
    AddParticle { particle: Particle },
    AddForceField { force_field: ForceField },
    Delete { position: Vec2 },
}

pub struct Visualizer {
    mouse_position: Vec2,
    size: Vec2,
    simulator: PSim,
    running: bool,
    dt: f64,
    realtime:bool
}

impl Visualizer {
    pub fn new(width: u32, height: u32, dt: f64,realtime:bool) -> GameResult<Self> {
        //create drawable or canvas
        Ok(Visualizer {
            simulator: PSim::new(),
            mouse_position: Vec2::new(0.0, 0.0),
            size: Vec2::new(width as f32, height as f32),
            running: true,
            dt,
            realtime
        })
    }

    fn handle_event(&mut self, event: VisEvent) {
        match event {
            VisEvent::AddParticle { particle } => {
                self.add_particle(particle);
            }
            VisEvent::AddForceField { force_field } => {
                self.add_force_field(force_field);
            }
            VisEvent::Delete { position } => {
                self.simulator.particles.retain(|particle| {
                    particle.get_pos().distance(position) as f64 > particle.get_radius()
                });
                self.simulator.force_fields.retain(|force_field| {
                    let force_field_pos: &Vec2 = force_field.get_pos();
                    match force_field.get_shape() {
                        Shape::Circle { radius } => {
                            force_field_pos.distance(position) as f64 > *radius
                        }
                        Shape::Rectangle { width, height } => {
                            !((force_field_pos.x >= position.x - (width / 2.0) as f32 && force_field_pos.x <= position.x + (width / 2.0) as f32) &&
                                (force_field_pos.y >= position.y - (height / 2.0) as f32 && force_field_pos.y <= position.y + (height / 2.0) as f32))
                        }
                    }
                })
            }
            VisEvent::Quit => {
                self.running = false;
            }
            VisEvent::Reset => {
                self.simulator = PSim::new();
            }
        }
    }
    pub fn add_particle(&mut self, particle: Particle) {
        self.simulator.add_particle(particle);
    }

    pub fn add_force_field(&mut self, force_field: ForceField) {
        self.simulator.add_force_field(force_field);
    }

    fn clean(&mut self) {
        //remove particles out of bounds
        self.simulator.particles.retain(|particle| {
            let pos = particle.get_pos();
            pos.x >= 0.0 && pos.x <= self.size.x && pos.y >= 0.0 && pos.y <= self.size.y
        })
    }

    fn draw_simulator(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        self.simulator.get_force_fields().iter().for_each(|force_field| {
            let pos = force_field.get_pos();
            match &force_field.get_shape() {
                Shape::Circle { radius } => {
                    let circle_mesh = graphics::Mesh::new_circle(
                        ctx,
                        graphics::DrawMode::fill(),
                        vec2(0., 0.),
                        *radius as f32,
                        0.0,
                        COLOR_FORCE_FIELD,
                    ).unwrap();
                    canvas.draw(&circle_mesh, Vec2::new(pos.x, pos.y));
                }
                Shape::Rectangle { width, height } => {
                    let rectangle = Rect::new(
                        -(width / 2.0) as f32,
                        -(height / 2.0) as f32,
                        *width as f32,
                        *height as f32,
                    );
                    let rectangle_mesh = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        rectangle,
                        COLOR_FORCE_FIELD,
                    ).unwrap();
                    canvas.draw(&rectangle_mesh, Vec2::new(pos.x, pos.y));
                }
            }
        });

        //draw particle
        self.simulator.get_particles().iter().for_each(|particle| {
            let pos = particle.get_pos();
            let circle_mesh = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                vec2(0., 0.),
                particle.get_radius() as f32,
                0.1,
                COLOR_PARTICLE,
            ).unwrap();
            canvas.draw(&circle_mesh, Vec2::new(pos.x as f32, pos.y as f32));
        });
        Ok(())
    }

    fn draw_gui(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let rectangle = Rect::new(
            0.0,
            0.0,
            self.size.x,
            -170.0,
        );
        let rectangle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rectangle,
            Color::from_rgb(180, 200, 35),
        ).unwrap();
        canvas.draw(&rectangle_mesh, Vec2::new(0.0, self.size.y));

        // Create a Font object using the system font
        let frametime = ctx.time.delta().as_secs_f64();
        let text = Text::new(TextFragment {
            text: format!("Frametime: {}\nFPS: {:.2}", frametime, 1.0 / frametime),
            color: Some(Color::BLACK),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(20.0)),
        });
        canvas.draw(&text, Vec2::new(0.0, self.size.y - text.dimensions(ctx).unwrap().size().y));
        Ok(())
    }
}

impl EventHandler for Visualizer {
    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> GameResult {
        match input.keycode.unwrap() {
            KeyCode::Escape => {
                ctx.request_quit();
            }
            KeyCode::P => {
                self.handle_event(VisEvent::AddParticle {
                    particle: Particle::new(
                        Vec2::new(self.mouse_position.x, self.mouse_position.y),
                        DEFAULT_PARTICLE_VELOCITY,
                        DEFAULT_PARTICLE_MASS,
                        DEFAULT_PARTICLE_RADIUS,
                    ),
                });
            }
            KeyCode::O => {
                self.handle_event(VisEvent::AddParticle {
                    particle: Particle::new(
                        Vec2::new(self.mouse_position.x, self.mouse_position.y),
                        Vec2::new(0.0, 0.0),
                        DEFAULT_BIG_PARTICLE_MASS,
                        DEFAULT_BIG_PARTICLE_RADIUS,
                    ),
                });
            }
            KeyCode::G => {
                self.handle_event(VisEvent::AddParticle {
                    particle: Particle::new(
                        Vec2::new(self.mouse_position.x, self.mouse_position.y),
                        Vec2::new(0.0, 0.0),
                        DEFAULT_GRAVITY_MASS,
                        DEFAULT_GRAVITY_RADIUS,
                    ),
                });
            }
            KeyCode::R => {
                self.handle_event(VisEvent::Reset);
            }
            KeyCode::D => {
                self.handle_event(VisEvent::Delete {
                    position: Vec2::new(self.mouse_position.x, self.mouse_position.y),
                });
            }
            _ => {}
        }
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) -> GameResult {
        self.mouse_position.x = x;
        self.mouse_position.y = y;
        Ok(())
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.clean();
        self.simulator.step(self.dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //clear screen
        let mut canvas = Canvas::from_frame(ctx, COLOR_BACKGROUND);

        //draw force fields
        self.draw_simulator(ctx, &mut canvas)?;
        self.draw_gui(ctx, &mut canvas)?;

        canvas.finish(ctx).unwrap();
        Ok(())
    }
}