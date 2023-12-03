use crate::psim::simulator::forcefield::ForceField;
use crate::psim::simulator::particle::Particle;

pub struct PSim {
    pub particles: Vec<Particle>,
    pub force_fields: Vec<ForceField>,
}

impl PSim {
    pub fn new() -> Self {
        PSim { particles: vec![], force_fields: vec![] }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn add_force_field(&mut self, force_field: ForceField) {
        self.force_fields.push(force_field);
    }

    pub fn get_particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    pub fn get_force_fields(&self) -> &Vec<ForceField> {
        &self.force_fields
    }

    fn add_forces(&mut self) {
        for particle in &mut self.particles {
            for force_field in &self.force_fields {
                if force_field.affects_particle(particle) {
                    let force = force_field.calculate_force(particle);
                    particle.add_force(&force);
                }
            }
        }
        let particle_count = self.particles.len();

        for i in 0..particle_count {
            for j in i + 1..particle_count {
                // Borrow two mutable references from the vector at indices i and j
                let (particles_left, particles_right) = self.particles.split_at_mut(j);
                let particle_i = &mut particles_left[i];
                let mut particle_j = &mut particles_right[0];
                particle_i.interact(&mut particle_j);
            }
        }
    }

    pub fn step(&mut self, dt: f64) {
        // if self.particles.len()>=2 {
        //     println!("Velocity:{} {}", self.particles[1].get_velocity().x, self.particles[0].get_velocity().y);
        // }
        self.add_forces();
        for particle in &mut self.particles {
            particle.step(dt);
        }
    }
}