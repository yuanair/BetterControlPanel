use num::{Complex, Rational64};
use num::complex::ComplexFloat;
use num::traits::Inv;
use num::traits::real::Real;
use serde::{Deserialize, Serialize};

pub mod particle_effects;
pub struct Game {}
/// 一元二次方程
/// ax^2 + bx + c = 0
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct QuadraticEquation {
    a: Rational64,
    b: Rational64,
    c: Rational64,
}

impl QuadraticEquation {
    pub fn new(a: Rational64, b: Rational64, c: Rational64) -> Self {
        Self {
            a,
            b,
            c,
        }
    }

    pub fn from_x_and_a(x1: Complex<Rational64>, x2: Complex<Rational64>, a: Rational64) -> Self {
        let delta = (x2.powi(2) - x1.powi(2)) * a;
        let b = delta / (x1 - x2);
        let c = -b * x1;
        Self {
            a,
            b: b.re,
            c: c.re,
        }
    }

    // pub fn calculate(&self) -> (Complex<Rational64>, Complex<Rational64>) {
    //     let sqrt_delta = Complex::new(self.b.pow(2) - 4.into() * self.a * self.c, 0.into()).sqrt();
    //     let double_a_inv = (Rational64::from(2) * self.a).inv();
    //     let b = <Complex<Rational64>>::from(-self.b);
    //     let x1 = (b + sqrt_delta) * double_a_inv;
    //     let x2 = (b - sqrt_delta) * double_a_inv;
    //     (x1, x2)
    // }
}

impl Game {
    pub fn new() -> Self {
        Self {}
    }
}

