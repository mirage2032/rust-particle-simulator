use ggez::glam::Vec2;
use physical_constants;
use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;
use crate::psim::simulator::particle::Particle;

pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

pub enum ForceType {
    Gravity { mass: f64 },
    Force { force: Vec2 },
}

pub struct ForceField {
    position: Vec2,
    shape: Shape,
    force_type: ForceType,
}

impl ForceField {
    pub fn new(position: Vec2, shape: Shape, force_type: ForceType) -> Self {
        ForceField { position, shape, force_type }
    }

    pub fn get_pos(&self) -> &Vec2 {
        &self.position
    }

    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    pub fn get_force_type(&self) -> &ForceType {
        &self.force_type
    }
    pub fn affects_particle(&self, particle: &Particle) -> bool {
        let particle_pos = particle.get_pos();
        let particle_radius = particle.get_radius();
        match self.shape {
            Shape::Circle { radius } => {
                let distance = particle_pos.distance(self.position) as f64;
                distance < radius + particle_radius
            }
            Shape::Rectangle { width, height } => {
                (particle_pos.x >= self.position.x - (width / 2.0) as f32 - particle_radius as f32 && particle_pos.x <= self.position.x + (width / 2.0) as f32 + particle_radius as f32) &&
                    (particle_pos.y >= self.position.y - (height / 2.0) as f32 - particle_radius as f32 && particle_pos.y <= self.position.y + (height / 2.0) as f32 + particle_radius as f32)
            }
        }
    }

    pub fn calculate_force(&self, particle: &Particle) -> Vec2 {
        match &self.force_type {
            ForceType::Gravity { mass } => {
                let distance = particle.get_pos().distance(self.position);
                let force = NEWTONIAN_CONSTANT_OF_GRAVITATION * mass * particle.get_mass() / distance.powi(2) as f64;
                let direction = (self.position - particle.get_pos().clone()).normalize();
                direction * (force as f32 / distance)
            }
            ForceType::Force { force } => {
                force.clone()
            }
        }
    }
}