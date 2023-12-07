use std::f32::consts::PI;
use ggez::glam::Vec2;
use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

pub struct Particle {
    position: Vec2,
    velocity: Vec2,
    total_forces: Vec2,
    mass: f64,
    radius: f64,
    //TODO: Make particles able to have different shapes
    is_static: bool,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, mass: f64, radius: f64) -> Self {
        Particle { position, velocity, total_forces: Vec2::new(0.0, 0.0), mass, radius, is_static: false }
    }

    pub fn new_static(position: Vec2, velocity: Vec2, mass: f64, radius: f64) -> Self {
        Particle { position, velocity, total_forces: Vec2::new(0.0, 0.0), mass, radius, is_static: true }
    }

    fn collides_with(&self, other: &Particle) -> bool {
        (self.position.distance(other.position) as f64) < self.radius + other.radius
    }

    pub fn interact(&mut self, other: &mut Particle) {
        if !self.is_static && !other.is_static {
            // Check for collision
            if self.collides_with(other) {
                // Resolve collision
                self.resolve_collision(other);
            }

            // Calculate gravitational force
            let distance = self.position.distance(other.position) as f64;
            let force_magnitude = NEWTONIAN_CONSTANT_OF_GRAVITATION * (self.mass * other.mass) / (distance * distance);

            // Calculate force direction
            let force_direction = (other.position - self.position).normalize_or_zero();

            // Apply gravitational forces to both particles
            self.apply_force(force_direction * force_magnitude as f32);
            other.apply_force(-force_direction * force_magnitude as f32);
        }
    }

    fn resolve_collision(&mut self, other: &mut Particle) {
        // Move particles to avoid overlap
        let overlap = (self.radius + other.radius) - self.position.distance(other.position) as f64;

        // Ensure particles are separated only if they are overlapping
        if overlap > 0.0 {
            let total_mass = self.mass + other.mass;
            let self_mass_ratio = self.mass / total_mass;
            let other_mass_ratio = other.mass / total_mass;
            let self_move_distance = overlap * self_mass_ratio;
            let other_move_distance = overlap * other_mass_ratio;

            // Calculate normalized direction vector from self to other
            let direction_self_to_other = (other.position - self.position).normalize_or_zero();

            // Move particles explicitly to avoid overlap along the correct direction
            self.move_by(-direction_self_to_other * other_move_distance as f32);
            other.move_by(direction_self_to_other * self_move_distance as f32);
        }

        // Calculate relative position and velocity after unoverlapping
        let relative_position = other.position - self.position;
        let relative_velocity = self.velocity - other.velocity;

        // Calculate normal vector pointing from self to other
        let normal = relative_position.normalize_or_zero();

        // Calculate impulse along the normal direction
        let impulse = -(1.0 + 1.0) * relative_velocity.dot(normal) / ((1.0 / self.mass) + (1.0 / other.mass)) as f32;

        // Apply impulse to update velocities
        self.velocity += impulse * normal / self.mass as f32;
        other.velocity -= impulse * normal / other.mass as f32;
    }


    pub fn move_by(&mut self, delta: Vec2) {
        if !self.is_static {
            self.position = self.position + delta;
        }
    }

    pub fn get_pos(&self) -> &Vec2 {
        &self.position
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn apply_force(&mut self, external_force: Vec2) {
        if !self.is_static {
            self.total_forces += external_force
        }
    }

    pub fn apply_forces(&mut self) {
        if !self.is_static {
            const DAMPING_FACTOR: f32 = 0.00;
            const FRICTION_COEFFICIENT: f32 = 0.00;
            const AIR_SPHERE_FRICTION_COEFFICIENT: f32 = 0.47;
            const AIR_DENSITY: f32 = 0.0;

            // Apply damping to velocity
            self.velocity *= 1.0 - DAMPING_FACTOR;

            // Apply friction to velocity
            let friction = -self.velocity.normalize_or_zero() * FRICTION_COEFFICIENT * PI * self.radius as f32 * self.radius as f32;
            self.apply_force(friction);

            // Apply air friction
            let reference_circumference = std::f32::consts::PI * 2.0 * self.radius as f32;
            let air_friction = -self.velocity * AIR_SPHERE_FRICTION_COEFFICIENT * AIR_DENSITY * reference_circumference * (2.0 * PI * self.radius as f32);
            self.apply_force(air_friction);

            // Calculate acceleration from forces
            let acceleration = self.total_forces / self.mass as f32;

            // Update velocity based on acceleration
            self.velocity += acceleration;
        }
    }

    pub fn reset_forces(&mut self) {
        if !self.is_static {
            self.total_forces = Vec2::new(0.0, 0.0);
        }
    }

    pub fn update_position(&mut self, dt: f64) {
        if !self.is_static {
            self.position += self.velocity * dt as f32;
        }
    }

    pub fn step(&mut self, dt: f64) {
        if !self.is_static {
            self.apply_forces();
            self.update_position(dt);
            self.reset_forces();
        }
    }
}