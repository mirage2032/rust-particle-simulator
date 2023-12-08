use core::fmt;

use ggez::glam::Vec2;
use rand::random;
use crate::psim::simulator::particle::Particle;

pub struct ParticleData {
    mass: f64,
    radius: f64,
    position: Vec2,
    total_forces: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

impl fmt::Display for ParticleData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mass: {}\nRadius: {}\nPosition: X:{:.4} Y:{:.4}\nTotal Forces:\nX:{:.4}\nY:{:.4}\nVelocity: X:{:.4} Y:{:.4}\nAcceleration: X:{:.4} Y:{:.4}", self.mass, self.radius, self.position.x, self.position.y, self.total_forces.x, self.total_forces.y, self.velocity.x, self.velocity.y, self.acceleration.x, self.acceleration.y)
    }
}

impl From<Particle> for ParticleData {
    fn from(particle: Particle) -> Self {
        ParticleData {
            mass: particle.get_mass(),
            radius: particle.get_radius(),
            position: particle.get_pos().clone(),
            total_forces: particle.get_total_forces().clone(),
            velocity: particle.get_velocity().clone(),
            acceleration: particle.get_acceleration(),
        }
    }
}

pub struct Gui {
    size: Vec2,
    scale: f64,
    dt: f64,
    realtime: bool,
    running: bool,
    active_particle_id: u64,
    active_particle_data: Option<ParticleData>,
}

impl Gui {
    pub fn new(size: Vec2, scale: f64, dt: f64, realtime: bool) -> Self {
        Gui {
            size,
            scale,
            dt,
            realtime,
            active_particle_id: random::<u64>(),
            running: false,
            active_particle_data: None,
        }
    }
    pub fn get_size(&self) -> Vec2 {
        self.size
    }
    pub fn get_scale(&self) -> f64 {
        self.scale
    }
    pub fn get_dt(&self) -> f64 {
        self.dt
    }
    pub fn get_realtime(&self) -> bool {
        self.realtime
    }
    pub fn get_active_particle_id(&self) -> u64 {
        self.active_particle_id
    }
    pub fn get_active_particle_data(&self) -> Option<&ParticleData> {
        self.active_particle_data.as_ref()
    }
    pub fn get_running(&self) -> bool {
        self.running
    }
    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }
    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }
    pub fn set_dt(&mut self, dt: f64) {
        self.dt = dt;
    }
    pub fn set_realtime(&mut self, realtime: bool) {
        self.realtime = realtime;
    }
    pub fn set_active_particle_id(&mut self, active_particle: u64) {
        self.active_particle_id = active_particle;
    }
    pub fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    pub fn set_active_particle_data(&mut self, particle: Particle) {
        self.active_particle_data = Some(ParticleData::from(particle));
    }
}