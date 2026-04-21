use std::{env::args, time::SystemTime};

use crate::generation_type::{self, parse_gentype};

// Function collecting unix time from standard library, found online. Fairly straightforward.
// Composed by github user jweinst1's gist. Thank you, github user.
// src: https://gist.github.com/jweinst1/0f0f2e9e31e487469e5367d42ad29253
fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AppArguments {
    pub types: u8,
    pub outputfp: String,
    pub entries: usize,
    pub seed: isize,
}

impl Default for AppArguments {
    fn default() -> Self {
        AppArguments {
            types: generation_type::ALL,
            outputfp: "datagen".to_owned(),
            entries: 100,
            seed: get_sys_time_in_secs() as isize,
        }
    }
}

impl AppArguments {
    // Collect arguments to application from terminal arguments.
    // Returns "None" when user has sent invalid arguments or wishes to open the help menu.
    pub fn collect() -> Option<AppArguments> {
        Self::from_arguments_array(args().collect())
    }

    pub fn from_arguments_array(arguments: Vec<String>) -> Option<AppArguments> {
        let mut o = AppArguments::default();

        let mut iter = arguments.iter().peekable();
        // ignore first entry because it is filepath
        iter.next();
        while let Some(v) = iter.next() {
            match v.as_str() {
                "-h" | "--help" => {
                    // shows helptext
                    return None;
                }
                "-s" | "--seed" => {
                    // we want to parse some inner data...
                    let Some(seed_raw) = iter.next() else {
                        println!("Invalid syntax: --seed <number> has no value afterward!");
                        return None;
                    };
                    let Ok(seed) = seed_raw.parse::<isize>() else {
                        println!("Could not parse seed '{}' as integer!", seed_raw);
                        return None;
                    };
                    o.seed = seed;
                }
                "-t" | "--types" => {
                    // collect types from following input
                    o.types = generation_type::NONE;
                    let Some(todo) = iter.next() else {
                        println!("Invalid syntax: --types <types> has no value afterward!");
                        return None;
                    };
                    // try and parse
                    for typetodo in todo.split(',') {
                        let Some(typetodo_u8) = parse_gentype(typetodo) else {
                            println!(
                                "Unknown table type: '{}'. Check helptext for all table types.",
                                typetodo
                            );
                            return None;
                        };
                        o.types |= typetodo_u8;
                    }
                }
                "-e" | "--entries" => {
                    // we want to parse some inner data...
                    let Some(entriescount_raw) = iter.next() else {
                        println!("Invalid syntax: --entries <count> has no value afterward!");
                        return None;
                    };
                    let Ok(entriescount) = entriescount_raw.parse::<usize>() else {
                        println!(
                            "Could not parse entries count '{}' as integer!",
                            entriescount_raw
                        );
                        return None;
                    };
                    o.entries = entriescount;
                }
                _ => {
                    // if this is the last argument...
                    if iter.peek() == None {
                        // then this is outputfp
                        o.outputfp = v.to_owned();
                    } else {
                        // otherwise, this is some unaccounted for
                        println!("Invalid argument: {}", v);
                        return None;
                    }
                }
            }
        }

        Some(o)
    }

    // Shows help text for program.
    pub fn show_help_text() {
        println!(
            "geo_datagen\n(c) ConorSS 2026\n---\nUsage:\ngeo_datagen.exe <option...> <outputfp>"
        );

        println!("---\nOptions:");
        for set in [
            ("Option(s)", "Description", "Default value"),
            (
                "<outputfp>",
                "Output filepath. Will be automatically prepended with output data type.",
                "datagen",
            ),
            ("-h", "Show helptext", ""),
            ("--help", "", ""),
            ("-s <number>", "Seed randomization.", "Random"),
            ("--seed <number>", "", "All"),
            (
                "-t <types>",
                "Set types of output data (Comma-seperated)",
                "All",
            ),
            ("--types <types>", "", "All"),
            (
                "-e <count>",
                "Set amount of entries in each set of output data",
                "100",
            ),
            ("--entries <count>", "", "100"),
        ] {
            println!("{:18}| {:90}| {}", set.0, set.1, set.2)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // validates a config without options defaults to the right default
    fn arguments_default() {
        assert_eq!(
            AppArguments::from_arguments_array(vec!["blah".to_owned()]),
            Some(AppArguments::default())
        );
    }

    #[test]
    // validates you can manually trigger helptext
    fn arguments_helptext() {
        assert_eq!(
            AppArguments::from_arguments_array(vec!["blah".to_owned(), "-h".to_owned()]),
            None
        );
        assert_eq!(
            AppArguments::from_arguments_array(vec!["blah".to_owned(), "--help".to_owned()]),
            None
        );
    }

    #[test]
    // validates the entries argument works right
    fn arguments_entries() {
        for set in [
            (
                vec!["blah".into(), "-e".into(), "369".into()],
                Some(AppArguments {
                    entries: 369,
                    ..Default::default()
                }),
            ),
            (
                vec!["blah".into(), "--entries".into(), "29032".into()],
                Some(AppArguments {
                    entries: 29032,
                    ..Default::default()
                }),
            ),
            (vec!["blah".into(), "--entries".into(), "0.06".into()], None),
            (vec!["blah".into(), "-e".into()], None),
        ] {
            assert_eq!(AppArguments::from_arguments_array(set.0), set.1)
        }
    }

    // practical testing shows all the others *should* work- don't have time for comprehensive right now
}
