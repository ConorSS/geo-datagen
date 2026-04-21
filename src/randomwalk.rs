use std::f32::consts::PI;

use glam::Vec2;
use rand::distr::uniform::{UniformFloat, UniformSampler};

use crate::RNG;

// 2d high-fidelity random walk algorithm.
// Collects next random walk point based on
pub fn randwalk2(rng: &mut RNG, point: &Vec2, maxdist: f32) -> Vec2 {
    if maxdist < 0.0 {
        panic!("Cannot randwalk with maxdist < 0!");
    }

    let angledist: UniformFloat<f32> = UniformFloat::new(0.0, 2.0 * PI).unwrap();
    let angle = angledist.sample(rng);
    let distdist: UniformFloat<f32> = UniformFloat::new(0.0, maxdist).unwrap();
    let dist = distdist.sample(rng);
    // Produce angled vector from parts.
    let displacement = Vec2::from_angle(angle) * dist;

    point + displacement
}
