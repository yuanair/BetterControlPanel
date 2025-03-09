use num::integer::Roots;
use num::traits::Inv;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

pub mod particle_effects;
pub struct Game {
    pub rng: StdRng,
    current_question: Option<QuadraticEquation>,
}
/// 一元二次方程
/// ax^2 + bx + c = 0
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct QuadraticEquation {
    pub a: i64,
    pub b: i64,
    pub c: i64,
}

impl QuadraticEquation {
    pub fn new(a: i64, b: i64, c: i64) -> Option<Self> {
        if a == 0 {
            return None;
        }
        Some(Self {
            a,
            b,
            c,
        })
    }

    pub fn from_x_and_a(x1: i64, x2: i64, a: i64) -> Option<Self> {
        Self::new(a, -a * (x1 + x2), a * x1 * x2)
    }

    pub fn calculate(&self) -> Option<(i64, i64)> {
        let delta = self.b.pow(2) - 4 * self.a * self.c;
        match delta {
            0 => {
                let x = -self.b / (2 * self.a);
                Some((x, x))
            }
            i64::MIN..0 => {
                None
            }
            _ => {
                let sqrt_delta = delta.sqrt();
                let double_a = 2 * self.a;
                let x1 = (-self.b + sqrt_delta) / double_a;
                let x2 = (-self.b - sqrt_delta) / double_a;
                Some((x1, x2))
            }
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            rng: StdRng::from_os_rng(),
            current_question: None,
        }
    }
    pub fn create_quadratic_equation(&mut self) -> QuadraticEquation {
        let x1 = self.rng.random_range(-100..=100);
        let x2 = self.rng.random_range(-100..=100);
        let a = self.rng.random_range(1..=10);
        let result = QuadraticEquation::from_x_and_a(x1, x2, a).unwrap();
        self.current_question = Some(result.clone());
        result
    }

    pub fn current_question(&self) -> Option<&QuadraticEquation> {
        self.current_question.as_ref()
    }
}

