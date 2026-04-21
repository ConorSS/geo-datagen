use std::fs::File;
use std::io::Write;

use chrono::{DateTime, TimeDelta, Utc};
use glam::Vec2;
use rand::{
    SeedableRng,
    distr::uniform::{UniformFloat, UniformSampler},
};

use crate::RNG;

pub mod temperature;
pub mod windspeed;

type DT = DateTime<Utc>;

// Enforce serializability of the Rows output by a Datagenerator (csv).
pub trait DataGeneratorRow: Clone {
    fn header() -> &'static str;

    fn serialize(&self) -> String;
}

// Primitive pattern for all singular data generators.
pub trait DataGenerator {
    type Row: DataGeneratorRow;

    fn gen_single(timestamp: &DT, latlong: &Vec2) -> Self::Row;

    fn gen_many(starttimestamp: &DT, latlongs: &Vec<Vec2>, totalentries: usize) -> Vec<Self::Row> {
        // get total entries for each latlong
        let rowcount = totalentries / latlongs.len();

        // generate many
        let mut o = vec![];

        for latlong in latlongs {
            let timestamp = *starttimestamp;

            for timeoffset in 0..rowcount {
                // offset is calculated in hours from input timestamp
                let thistime = timestamp + TimeDelta::new((timeoffset as i64) * 3600, 0).unwrap();
                let entry = Self::gen_single(&thistime, latlong);
                o.push(entry);
            }
        }

        o
    }

    fn write_rows(fp: &str, rows: &Vec<Self::Row>) {
        let Ok(mut file) = File::create(fp) else {
            panic!("Could not create/truncate file: {fp}");
        };
        if writeln!(file, "{}", temperature::Row::header()).is_err() {
            panic!("File write failure to {fp}")
        }
        for v in rows {
            if writeln!(file, "{}", v.serialize()).is_err() {
                panic!("File write failure to {fp}")
            }
        }
    }
}

pub fn gen_latlongs(seed: i128, count: usize) -> Vec<Vec2> {
    let mut o = vec![];
    let mut rng = RNG::from_seed(seed.to_le_bytes());
    // Temporary range transform bounds- random location in peak district
    // src: https://www.openstreetmap.org
    const MAXIMA: Vec2 = Vec2::new(53.51755, -1.92553);
    const MINIMA: Vec2 = Vec2::new(53.45995, -1.80468);
    let distrubutionx = UniformFloat::<f32>::new(MINIMA.x, MAXIMA.x).unwrap();
    let distributiony = UniformFloat::<f32>::new(MAXIMA.y, MINIMA.y).unwrap();

    for _ in 0..count {
        // Randomly generate vector in domain and append.
        let randpos = Vec2::new(
            distrubutionx.sample(&mut rng),
            distributiony.sample(&mut rng),
        );
        o.push(randpos);
    }

    o
}

// failed attempt at implementing a good (stream-like) iterator interface for data generation
// (this was miserable)
// for some semblance of an iterator interface, just do things with the output of gen_many

// pub struct BulkDataGenerator {
//     starttimestamp : DT,
//     latlongs : Vec<Vec2>,
//     totalentries : usize,

//     inner_currow : usize,
//     inner_rows : Box<dyn Iterator<Item = usize>>,
//     inner_latlongs : Box<dyn Iterator<Item = Vec2>>,
// }

// impl BulkDataGenerator {
//     pub fn new(
//         starttimestamp : &DT,
//         latlongs : &Vec<Vec2>,
//         totalentries : usize
//     ) -> Self {
//         let inner_maxima = totalentries / latlongs.len();
//         let inner_latlongs = latlongs.to_owned();
//         Self {
//             starttimestamp : starttimestamp.clone(),
//             latlongs : inner_latlongs,
//             totalentries : totalentries,

//             inner_currow : 0,
//             inner_rows : Box::new((0..inner_maxima).skip(1)),
//             inner_latlongs : Box::new(std::iter::empty())
//         }
//     }
// }

// impl<T : DataGenerator> Iterator for BulkDataGenerator {
//     type Item = T::Row;

//     fn next(&mut self) -> Option<Self::Item> {
//         let latlong = match self.inner_latlongs.next() {
//             Some(v) => v,
//             None => {
//                 self.inner_latlongs = Box::new(self.latlongs.iter());
//                 self.inner_currow = match self.inner_rows.next() {
//                     Some(v) => v,
//                     _ => {
//                         return None
//                     }
//                 };
//                 self.inner_latlongs.next().expect("latlongs is empty!")
//             }
//         };
//     }
// }
