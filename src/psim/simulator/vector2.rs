#[derive(Clone,PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    pub fn zero() -> Self {
        Vector2::new(0.0,0.0)
    }
    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2 { x: self.x + other.x, y: self.y + other.y }
    }

    pub fn sub(&self, other: &Vector2) -> Vector2 {
        Vector2 { x: self.x - other.x, y: self.y - other.y }
    }

    pub fn scale(&self, factor: f64) -> Vector2 {
        Vector2 { x: self.x * factor, y: self.y * factor }
    }

    pub fn scale_in_place(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }

    pub fn inverse(&self) -> Vector2 {
        Vector2 { x: -self.x, y: -self.y }
    }

    pub fn normalize(&self) -> Vector2 {
        let length = self.magnitude();
        if length == 0.0 {
            Vector2::zero()
        }
        else {
            Vector2 { x: self.x / length, y: self.y / length }
        }
    }
    pub fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn distance(&self, other: &Vector2) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn magnitude(&self) -> f64 {
        self.distance(&Vector2::zero())
    }
}