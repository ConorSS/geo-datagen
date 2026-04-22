pub mod algos;

// Bitflag definitions and implementations for generation types.
pub const NONE: u8 = 0;
pub const TEMPERATURE: u8 = 0b0001;
pub const WINDSPEED: u8 = 0b0010;
pub const GPS: u8 = 0b0100;
pub const ALL: u8 = TEMPERATURE | WINDSPEED | GPS;

// Iterator type for iterating through all bitflags conveniently.
#[derive(Debug)]
pub struct GenTypeIterator {
    prev: u8,
    full: u8,
}

impl Iterator for GenTypeIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.prev = if self.prev == NONE {
                0x1
            } else {
                self.prev << 0x1
            };

            if (self.prev & self.full) > 0 {
                return Some(self.prev);
            }
            if self.prev >= self.full {
                return None;
            }
        }
    }
}

// Converts bitflags into iterator for convenient iteration.
pub fn gentype_into_iterator(flags: u8) -> GenTypeIterator {
    GenTypeIterator {
        prev: NONE,
        full: flags,
    }
}

// Translation table converting string labels into their associated generation type, or None if it does not parse.
pub fn parse_gentype(value: &str) -> Option<u8> {
    Some(match value.to_lowercase().as_str() {
        "temperature" => TEMPERATURE,
        "windspeed" => WINDSPEED,
        "gps" => GPS,
        "all" => ALL,
        _ => { return None },
    })
}

pub fn gentype_to_string(value: u8) -> &'static str {
    match value {
        TEMPERATURE => "temperature",
        WINDSPEED => "windspeed",
        GPS => "gps",
        ALL => "all",
        _ => "unknown",
    }
}
