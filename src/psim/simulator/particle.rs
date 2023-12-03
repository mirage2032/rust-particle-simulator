use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;
use crate::psim::simulator::vector2::Vector2;

pub struct Particle {
    position: Vector2,
    velocity: Vector2,
    mass: f64,
    radius: f64,
    //TODO: Make particles able to have different shapes
    is_static: bool,
}

impl Particle {
    pub fn new(position: Vector2, velocity: Vector2, mass: f64, radius: f64) -> Self {
        Particle { position, velocity, mass, radius, is_static: false }
    }

    pub fn new_static(position: Vector2, velocity: Vector2, mass: f64, radius: f64) -> Self {
        Particle { position, velocity, mass, radius, is_static: true }
    }

    fn collides_with(&self, other: &Particle) -> bool {
        self.position.distance(&other.position) < self.radius + other.radius
    }

    pub fn collision_force(&self, other: &Particle) -> Option<Vector2> {
        // Calculate the direction vector between the particles
        let direction = self.position.sub(&other.position).normalize();

        // Calculate the relative velocity
        let relative_velocity = self.velocity.sub(&other.velocity);

        // Calculate the repulsive force, considering the masses
        let repulsive_force = (self.mass * other.mass) / self.position.distance(&other.position).powi(2);

        // Introduce damping to reduce extreme speeds
        let damping_factor = 0.1; // Adjust as needed

        // Limit the maximum force to avoid unrealistic behavior
        let max_force = 1000.0; // Adjust as needed
        let repulsive_force = repulsive_force.min(max_force);

        // Calculate the damping force
        let damping_force = relative_velocity.scale(-damping_factor); // Note the negative sign

        // Calculate the force for self, including damping
        let force_self = direction
            .scale(repulsive_force)
            .add(&damping_force); // Add damping force to repulsive force

        Some(force_self)
    }

    pub fn interact(&mut self, other: &mut Particle) {
        // Gravitational pull
        let distance = other.position.distance(&self.position);
        let force = NEWTONIAN_CONSTANT_OF_GRAVITATION * self.mass * other.mass / distance.powi(2);
        let direction = self.position.sub(&other.position).normalize();
        let gravity_force = direction.scale(force / distance);

        if !self.collides_with(other) {
            self.add_force(&gravity_force.inverse());
            other.add_force(&gravity_force);
            return;
        }

        // Calculate the overlap distance
        let overlap_distance = self.radius + other.radius - distance;
        if overlap_distance <= 0.0 {
            return; // No overlap, no need to move
        }

        // Move particles away based on overlap, taking into account masses
        let total_mass = self.mass + other.mass;
        let self_move_distance = direction.scale(overlap_distance * (other.mass / total_mass));
        let other_move_distance = direction.scale(overlap_distance * (self.mass / total_mass)).inverse();

        // Move the particles so they don't overlap
        self.move_by(&self_move_distance);
        other.move_by(&other_move_distance);

        // Calculate and add collision force
        if let Some(force) = self.collision_force(other) {
            self.add_force(&force);
            other.add_force(&force.inverse());
        }
    }

    pub fn move_by(&mut self, delta: &Vector2) {
        if !self.is_static {
            self.position = self.position.add(delta);
        }
    }

    pub fn get_pos(&self) -> &Vector2 {
        &self.position
    }

    pub fn get_velocity(&self) -> &Vector2 {
        &self.velocity
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn add_force(&mut self, force: &Vector2) {
        if !self.is_static {
            self.velocity = self.velocity.add(&force.scale(1.0 / self.mass));
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
            let wind_direction = self.velocity.normalize();

            // Damping force
            let damping_force_magnitude = -damping_coefficient * self.velocity.magnitude();
            let damping_force = wind_direction.scale(damping_force_magnitude * self.mass);

            // Drag force
            let drag_force_magnitude = -0.5 * drag_coefficient * air_density * self.velocity.magnitude().powi(2) * self.radius * facing_wind_area_fraction;
            let drag_force = wind_direction.scale(drag_force_magnitude);

            // Update velocity and position
            let net_force = damping_force.add(&drag_force);
            self.velocity = self.velocity.add(&net_force.scale(dt / self.mass));
            self.position = self.position.add(&self.velocity.scale(dt));
        }
    }
}