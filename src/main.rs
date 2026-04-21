use core::f32;
use std::{fs::File, io::Write, time::Instant};

use chrono::Utc;
use glam::Vec2;
use rand_xoshiro::Xoroshiro128PlusPlus;

use crate::{
    argmaster::AppArguments,
    generation_type::{
        algos::{
            DataGenerator, DataGeneratorRow, gen_latlongs,
            temperature::{self, TemperatureDataGen},
        },
        gentype_into_iterator, gentype_to_string,
    },
};

mod argmaster;
mod generation_type;
mod randomwalk;
mod simplex;

pub type RNG = Xoroshiro128PlusPlus;

fn tonemap(value: f32) -> char {
    if value < -0.5 {
        '░'
    } else if value < 0.0 {
        '▒'
    } else if value < 0.5 {
        '▓'
    } else {
        '█'
    }
}

fn simplex_grid(w: i32, h: i32, scale: f32) {
    for i in 0..h {
        for j in 0..w {
            let mut point = Vec2::new(i as f32, j as f32);
            point *= scale;
            print!("{}", tonemap(simplex::simplex2(&point)));
        }
        print!("\n");
    }
}

fn main() {
    let args: AppArguments = match AppArguments::collect() {
        Some(v) => v,
        None => {
            AppArguments::show_help_text();
            return;
        }
    };

    println!("Generating with seed ({})...", args.seed);

    // start timer
    let timer = Instant::now();
    // collect statics between generators
    let now = Utc::now();
    let latlongs = gen_latlongs(
        args.seed as i128,
        // The amount of latlong points is determined by the total.
        (args.entries.isqrt()).max(1),
    );

    let data_types = gentype_into_iterator(args.types);
    for data_type in data_types {
        match data_type {
            generation_type::TEMPERATURE => {
                println!("Generating temperature...");

                // collect values
                let values = TemperatureDataGen::gen_many(&now, &latlongs, args.entries);

                // serialize to file
                let fp = format!("{}_{}.csv", gentype_to_string(data_type), args.outputfp);
                let Ok(mut file) = File::create(&fp) else {
                    panic!("Could not create/truncate file: {fp}");
                };
                if writeln!(file, "{}", temperature::Row::header()).is_err() {
                    panic!("File write failure to {fp}")
                }
                for v in values {
                    if writeln!(file, "{}", v.serialize()).is_err() {
                        panic!("File write failure to {fp}")
                    }
                }
                println!("Written to {fp}.");
            }
            generation_type::WINDSPEED => {
                println!("Generating windspeed...");
                todo!("yeah no way buddy");
            }
            _ => panic!("Unexpected data_type!: {}", data_type),
        }
    }

    println!(
        "Completed in {} s.",
        Instant::now().duration_since(timer).as_secs()
    );

    simplex_grid(50, 5, 0.1);
}
