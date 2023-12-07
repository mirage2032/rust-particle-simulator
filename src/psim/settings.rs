use ggez::glam::Vec2;
use crate::psim::simulator::particle::Particle;

struct Settings<'a>{
    size:Vec2,
    scale:f64,
    dt: f64,
    realtime:bool,
    active_particle:Option<&'a Particle>,
}