use glam::Vec2;
use rand::{SeedableRng, distr::uniform::{UniformFloat, UniformSampler}};

use crate::{RNG, generation_type::algos::{DT, DataGenerator, DataGeneratorRow}, simplex::simplex2};



#[derive(Debug, Clone)]
pub struct Row {
	timestamp : DT,
	latlong : Vec2,
	windspeed : f32
}

impl DataGeneratorRow for Row {
    fn header() -> &'static str {
        "timestamp,lat,long,windspeed"
    }

    fn serialize(&self) -> String {
        format!("{},{},{},{}", self.timestamp.to_rfc3339(), self.latlong.x, self.latlong.y, self.windspeed)
    }
}

pub struct WindspeedDataGen {}

impl DataGenerator for WindspeedDataGen {
	type Row = Row;

	fn gen_single(timestamp: &DT, latlong: &Vec2) -> Self::Row {
		// In this generator, simplex is used to dictate the "exposure" of a particular point.
		let exposure = (simplex2(latlong) + 1.0)*0.5;
		// The most exposed points experience wind between 10-30mph, while less exposed points experience 0-20mph.
		let dist = UniformFloat::<f32>::new(
			exposure*10.0,
			10.0 + exposure*20.0 
		).unwrap();

		// The wind speed varies depending on the timestamp.
		let t = timestamp.timestamp();
		let mut rng = RNG::from_seed((t as i128).to_le_bytes());

		Row {
			timestamp : *timestamp,
			latlong : *latlong,
			windspeed : dist.sample(&mut rng)
		}
	}
}