use glam::{Vec2, Vec3};

use super::DT;
use crate::{
    generation_type::algos::{DataGenerator, DataGeneratorRow},
    simplex::simplex3,
};

#[derive(Debug, Clone)]
pub struct Row {
    pub timestamp: DT,
    pub latlong: Vec2,
    pub temperature: f32,
}

impl DataGeneratorRow for Row {
    fn header() -> &'static str {
        "timestamp,lat,long,temperature"
    }

    fn serialize(&self) -> String {
        format!(
            "{},{},{},{}",
            self.timestamp.to_rfc3339(),
            self.latlong.x,
            self.latlong.y,
            self.temperature
        )
    }
}

pub struct TemperatureDataGen {}

impl DataGenerator for TemperatureDataGen {
    type Row = Row;

    fn gen_single(timestamp: &DT, latlong: &Vec2) -> Row {
        // we'll be using simplex3 to produce this output, just need to translate
        let timems = (timestamp.timestamp() as f32) / 200000.0;
        let point = Vec3::new(latlong.x, latlong.y, timems);
        let noise = simplex3(&point);
        // re-scale to acceptable celsius range
        let temperature = 15.0 + ((noise + 1.0) * 0.5) * 10.0;
        Row {
            timestamp: timestamp.clone(),
            latlong: latlong.clone(),
            temperature: temperature,
        }
    }
}
