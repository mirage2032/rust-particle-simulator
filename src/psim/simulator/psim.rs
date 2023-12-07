use std::collections::HashMap;

use multi_mut::HashMapMultiMut;
use rand::random;

use crate::psim::simulator::forcefield::ForceField;
use crate::psim::simulator::particle::Particle;

pub struct PSim {
    pub particles: HashMap<u64,Particle>,
    pub force_fields: Vec<ForceField>
}

impl PSim {
    pub fn new() -> Self {
        PSim { particles: HashMap::new() , force_fields: vec![]}
    }

    pub fn add_particle(&mut self, particle: Particle) {
        let id = random::<u64>();
        self.particles.insert(id,particle);
    }

    pub fn add_force_field(&mut self, force_field: ForceField) {
        self.force_fields.push(force_field);
    }

    pub fn get_particles(&self) -> &HashMap<u64,Particle> {
        &self.particles
    }

    pub fn get_force_fields(&self) -> &Vec<ForceField> {
        &self.force_fields
    }

    fn add_forces(&mut self) {
        for (id,particle) in &mut self.particles {
            for force_field in &self.force_fields {
                if force_field.affects_particle(particle) {
                    let force = force_field.calculate_force(particle);
                    particle.apply_force(force);
                }
            }
        }
        let ids: Vec<u64> = self.particles.keys().cloned().collect();
        let particle_count = ids.len();

        for i in 0..particle_count {
            for j in i + 1..particle_count {
                let id_i = ids[i];
                let id_j = ids[j];
                let (particle_i,particle_j) = self.particles.get_pair_mut(&id_i, &id_j).unwrap();
                particle_i.interact(particle_j);
            }
        }
    }

    pub fn step(&mut self, dt: f64) {
        // if self.particles.len()>=2 {
        //     println!("Velocity:{} {}", self.particles[1].get_velocity().x, self.particles[0].get_velocity().y);
        // }
        self.add_forces();
        for (_, particle) in &mut self.particles {
            particle.step(dt);
        }
    }
    }