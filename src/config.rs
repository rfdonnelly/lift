use clap::ArgMatches;

pub struct Config {
    pub bar: u32,
    pub sets: u32,
    pub work_set: u32,
}

impl Config {
    pub fn from_matches(matches: &ArgMatches) -> Config {
        Config {
            bar: value_t!(matches, "bar", u32).unwrap(),
            sets: value_t!(matches, "sets", u32).unwrap(),
            work_set: value_t!(matches, "work-set", u32).unwrap(),
        }
    }
}
