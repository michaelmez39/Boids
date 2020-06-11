mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
    #[wasm_bindgen(js_namespace = Math)]
    fn floor(s: f64) -> f64;
}

#[wasm_bindgen]
pub fn greet() {
    let a = format!("{}", random());
    alert(&a);
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub struct Vec2 {
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

    fn new(x: f64, y: f64) -> Self {
        Self {x, y}
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
impl std::ops::Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other
        }
    }
}

#[wasm_bindgen]
pub struct BoidConfig {
    separation: f64,
    alignment: f64,
    cohesion: f64,
    limit: f64,
    width: f64,
    height: f64
}

impl BoidConfig {
    pub fn new(separation: f64, alignment: f64, cohesion: f64, limit: f64, width: f64, height: f64) -> Self {
        Self {
            separation,
            alignment,
            cohesion,
            limit,
            width,
            height
        }
    }
}

impl Default for BoidConfig {
    fn default() -> Self {
        Self {
            separation: 100.0,
            alignment: 100.0,
            cohesion: 8.0,
            limit: 4.0,
            width: 500.0,
            height: 300.0
        }
    }
}

fn rand_range(rng: std::ops::Range<usize>) -> f64 {
    floor(rng.start as f64 + (rng.end - rng.start) as f64 * random())
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
    pub fn new_random(n: usize, width: usize, height: usize, separation: f64, alignment: f64, cohesion: f64, limit: f64) -> Self {
        let positions: Vec<Vec2> = (0..n).map(|_| Vec2::new(rand_range(0..width), rand_range(0..height))).collect();
        Self {
            positions,
            velocity: vec!(Vec2::zero(); n),
            config: BoidConfig {
                separation,
                alignment,
                cohesion,
                limit,
                width: width as f64,
                height: height as f64
            }
        }
    }

    pub fn with_config(self, config: BoidConfig) -> Self {
        Self {
            config,
            ..self
        }
    }
    
    pub fn step(&mut self) {
        for (element, position) in self.positions.iter().enumerate() {
            let mut velocity = self.rule1(position) + self.rule2(position) + self.rule3(&self.velocity, element);
            if velocity.abs() > self.config.limit {
                velocity = (velocity / (velocity.abs())) * self.config.limit;
            }

            if position.x > self.config.width {
                velocity.x = -3.0;
            } else if position.x < 0.0 {
                velocity.x = 3.0;
            }

            if position.y > self.config.height {
                velocity.y = -3.0;
            } else if position.y < 0.0 {
                velocity.y = 3.0;
            }
            self.velocity[element] = velocity;
        }
        for i in 0..self.positions.len() {
            let position = self.positions[i] + self.velocity[i];
            
            self.positions[i] = position;
        }
    }

    pub fn positions(&self) -> *const Vec2 {
        self.positions.as_ptr()
    }
}