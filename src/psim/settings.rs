use ggez::glam::Vec2;
use rand::random;

pub struct Settings{
    size:Vec2,
    scale:f64,
    dt: f64,
    realtime:bool,
    active_particle_id:u64,
    running: bool,
}

impl Settings {
    pub fn new(size:Vec2,scale:f64,dt:f64,realtime:bool) -> Self {
        Settings {
            size,
            scale,
            dt,
            realtime,
            active_particle_id: random::<u64>(),
            running: false,
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
    pub fn get_running(&self) -> bool {
        self.running
    }
    pub fn set_size(&mut self, size:Vec2) {
        self.size = size;
    }
    pub fn set_scale(&mut self, scale:f64) {
        self.scale = scale;
    }
    pub fn set_dt(&mut self, dt:f64) {
        self.dt = dt;
    }
    pub fn set_realtime(&mut self, realtime:bool) {
        self.realtime = realtime;
    }
    pub fn set_active_particle_id(&mut self, active_particle:u64) {
        self.active_particle_id = active_particle;
    }
    pub fn set_running(&mut self, running:bool) {
        self.running = running;
    }
}