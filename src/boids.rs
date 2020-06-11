use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64
}

impl Vec2 {
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

    fn abs(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl std::ops::Sub<&Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, other: &Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl std::ops::Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other
        }
    }
}

#[wasm_bindgen]
pub struct BoidConfig {
    separation: f64,
    alignment: f64,
    cohesion: f64
}

impl BoidConfig {
    pub fn new(separation: f64, alignment: f64, cohesion: f64) -> Self {
        Self {
            separation,
            alignment,
            cohesion
        }
    }
}

impl Default for BoidConfig {
    fn default() -> Self {
        Self {
            separation: 100.0,
            alignment: 100.0,
            cohesion: 8.0
        }
    }
}

#[wasm_bindgen]
pub struct Flock {
    positions: Vec<Vec2>,
    velocity: Vec<Vec2>,
    config: BoidConfig,
}

impl Flock {
     // Cohesion
     fn rule1(&self, b: &Vec2 ) -> Vec2 {
        self.positions.iter().fold(Vec2::zero(), |acc, c| {
            if b != c {
                acc + *b
            } else {
                acc
            }
        }) / self.config.cohesion
    }

    // Alignment

    fn rule2(&self, b: &Vec2) -> Vec2 {
        self.positions.iter().fold(Vec2::zero(), |acc, c| {
            if (*b-c).abs() < self.config.separation {
                acc - (*b-c)
            } else {
                acc
            }
        })
    }

    fn rule3(&self, velocity: &Vec<Vec2>, e: usize) -> Vec2 {
        velocity.iter().enumerate().fold(Vec2::zero(), |acc, (i, v)| if i != e {acc + *v} else {acc}) / self.config.alignment
    }
}

#[wasm_bindgen]
impl Flock {
    pub fn new_random() -> Self {
        unimplemented!();
    }

    pub fn with_config(self, config: BoidConfig) -> Self {
        Self {
            config,
            ..self
        }
    }
    
    pub fn step(&mut self) {
        for (element, position) in self.positions.iter().enumerate() {
            let velocity = self.rule1(position) + self.rule2(position) + self.rule3(&self.velocity, element);
            self.velocity[element] = velocity;
        }
    }
}