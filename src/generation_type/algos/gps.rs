
use glam::Vec2;
use rand::SeedableRng;
use uuid::Uuid;

use crate::{RNG, generation_type::algos::common::{latlong_from_basis, uuid_from_basis}, randomwalk::_randwalk2};

use super::{DataGenerator, DataGeneratorRow, DT};

#[derive(Debug, Clone)]
pub struct Row {
    pub timestamp: DT,
    pub device_id: Uuid,
    pub latlong: Vec2,
}

impl DataGeneratorRow for Row {
    fn header() -> &'static str {
        "timestamp,id,lat,long"
    }

    fn serialize(&self) -> String {
        format!(
            "{},{},{},{}",
            self.timestamp.to_rfc3339(),
            self.device_id.hyphenated().to_string(),
            self.latlong.x,
            self.latlong.y
        )
    }
}

pub struct GpsDataGen {}

impl DataGenerator for GpsDataGen {
    type Row = Row;

    fn gen_single(timestamp: &DT, basis: u128 ) -> Self::Row {
        // Random basis is on the initial latlong start point.
        let mut rng = RNG::from_seed(basis.to_le_bytes());

        // Generate device id.
        let device_id = uuid_from_basis(basis);

        // Random walk a few target points to create a cycle.
        let mut cycle : [Vec2; 3] = [Vec2::ZERO; 3];
        cycle[0] = latlong_from_basis(basis);
        cycle[1] = _randwalk2(&mut rng, &cycle[0], 20.0);
        cycle[2] = _randwalk2(&mut rng, &cycle[1], 20.0);

        // The position on the cycle depends on the timestamp.
        let t = (timestamp.timestamp() as f32 / 200.0).sin(); // very lazy but works
        let progress = t*2.0;
        let pos = 
        cycle[progress.trunc() as usize].lerp(
            cycle[progress.trunc() as usize + 1], 
            progress.fract()
        );

        Row {
            timestamp : *timestamp,
            device_id,
            latlong: pos
        }
    }
}