use glam::Vec2;
use rand::{RngExt, SeedableRng, distr::{StandardUniform, uniform::{UniformFloat, UniformSampler}}};
use uuid::Uuid;

use crate::RNG;

// Generate an array of basis values from a single seed.
pub fn gen_basis_array(seed : u128, count: usize) -> Vec<u128> {
    let mut o: Vec<u128> = vec![];
    let mut rng = RNG::from_seed(seed.to_le_bytes());
    for _ in 0..count {
        o.push(
            rng.sample(StandardUniform)
        );
    }
    o
}

pub fn latlong_from_basis(basis : u128) -> Vec2 {
    let mut rng = RNG::from_seed(basis.to_le_bytes());
    // Temporary range transform bounds- random location in peak district
    // src: https://www.openstreetmap.org
    const MAXIMA: Vec2 = Vec2::new(53.51755, -1.92553);
    const MINIMA: Vec2 = Vec2::new(53.45995, -1.80468);
    let distrubutionx = UniformFloat::<f32>::new(MINIMA.x, MAXIMA.x).unwrap();
    let distributiony = UniformFloat::<f32>::new(MAXIMA.y, MINIMA.y).unwrap();

    Vec2::new(
        distrubutionx.sample(&mut rng),
        distributiony.sample(&mut rng),
    )
}

pub fn uuid_from_basis(basis : u128) -> Uuid {
    // Literally translate the bits into a uuid and return
    Uuid::from_u128(basis)
}