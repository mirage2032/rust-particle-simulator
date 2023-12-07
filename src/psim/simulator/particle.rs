use ggez::glam::Vec2;
use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

pub struct Particle {
    position: Vec2,
    velocity: Vec2,
    mass: f64,
    radius: f64,
    //TODO: Make particles able to have different shapes
    is_static: bool,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, mass: f64, radius: f64) -> Self {
        Particle { position, velocity, mass, radius, is_static: false }
    }

    pub fn new_static(position: Vec2, velocity: Vec2, mass: f64, radius: f64) -> Self {
        Particle { position, velocity, mass, radius, is_static: true }
    }

    fn collides_with(&self, other: &Particle) -> bool {
        (self.position.distance(other.position) as f64) < self.radius + other.radius
    }

    pub fn collision_force(&self, other: &Particle) -> Option<Vec2> {
        // Calculate the direction vector between the particles
        let direction =(self.position - other.position).normalize();

        // Calculate the relative velocity
        let relative_velocity = self.velocity-other.velocity;

        // Calculate the repulsive force, considering the masses
        let repulsive_force = (self.mass * other.mass) / self.position.distance(other.position).powi(2) as f64;

        // Introduce damping to reduce extreme speeds
        let damping_factor = 0.1; // Adjust as needed

        // Limit the maximum force to avoid unrealistic behavior
        let max_force = 1000.0; // Adjust as needed
        let repulsive_force = repulsive_force.min(max_force);

        // Calculate the damping force
        let damping_force = relative_velocity * (-damping_factor); // Note the negative sign

        // Calculate the force for self, including damping
        let force_self = direction * repulsive_force as f32 + damping_force; // Add damping force to repulsive force

        Some(force_self)
    }

    pub fn interact(&mut self, other: &mut Particle) {
        // Gravitational pull
        let distance = other.position.distance(self.position) as f64;
        let force = NEWTONIAN_CONSTANT_OF_GRAVITATION * self.mass * other.mass / distance.powi(2) as f64;
        let direction = (self.position - other.position).normalize();
        let gravity_force = direction * (force / distance) as f32;

        if !self.collides_with(other) {
            self.add_force(-1.0 * gravity_force);
            other.add_force(gravity_force);
            return;
        }

        // Calculate the overlap distance
        let overlap_distance = self.radius + other.radius - distance;
        if overlap_distance <= 0.0 {
            return; // No overlap, no need to move
        }

        // Move particles away based on overlap, taking into account masses
        let total_mass = self.mass + other.mass;
        let self_move_distance = direction * (overlap_distance * (other.mass / total_mass)) as f32;
        let other_move_distance = direction * -1.0 * (overlap_distance * (self.mass / total_mass)) as f32;

        // Move the particles so they don't overlap
        self.move_by(self_move_distance);
        other.move_by(other_move_distance);

        // Calculate and add collision force
        if let Some(force) = self.collision_force(other) {
            self.add_force(force);
            other.add_force(force * -1.0);
        }
    }

    pub fn move_by(&mut self, delta: Vec2) {
        if !self.is_static {
            self.position = self.position+delta;
        }
    }

    pub fn get_pos(&self) -> &Vec2 {
        &self.position
    }

    pub fn get_velocity(&self) -> &Vec2 {
        &self.velocity
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn add_force(&mut self, force: Vec2) {
        if !self.is_static {
            self.velocity = self.velocity + (force * (1.0 / self.mass) as f32);
        }
    }

    pub fn step(&mut self, dt: f64) {
        if !self.is_static {
            // Linear damping (proportional to velocity)
            let damping_coefficient = 0.01; // Adjust as needed

            let facing_wind_area_fraction = 0.5; // Adjust as needed
            let drag_coefficient = 0.47; // Adjust as needed
            let air_density = 1.2; // Adjust as needed

            // Wind direction
            let wind_direction = self.velocity.normalize_or_zero();

            // Damping force
            let damping_force_magnitude = -damping_coefficient * self.velocity.length();
            let damping_force = wind_direction * (damping_force_magnitude * self.mass as f32);

            // Drag force
            let drag_force_magnitude = -0.5 * drag_coefficient * air_density * self.velocity.length().powi(2) * self.radius as f32 * facing_wind_area_fraction;
            let drag_force = wind_direction * drag_force_magnitude;

            // Update velocity and position
            let net_force = damping_force + drag_force;
            self.velocity = self.velocity + net_force * (dt / self.mass) as f32;
            self.position = self.position + self.velocity * dt as f32;
        }
    }
}