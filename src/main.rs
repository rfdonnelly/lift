#[macro_use]
extern crate clap;

use clap::{App, Arg, AppSettings};

use std::cmp;

fn main() {
    let matches = App::new(crate_name!())
        .setting(AppSettings::NextLineHelp)
        .about("\nLift helps with barbell lift planning.")
        .version(concat!("v", crate_version!()))
        .author(crate_authors!())
        .arg(Arg::with_name("bar")
            .help("Sets the bar weight")
            .short("b")
            .long("bar")
            .default_value("45")
            .validator(validate_number)
            .takes_value(true))
        .arg(Arg::with_name("sets")
             .help("Sets the number of sets")
             .short("s")
             .long("sets")
             .default_value("5")
             .validator(validate_number)
             .takes_value(true))
        .arg(Arg::with_name("work-set")
             .help("Sets the weight of the work set.  Must be great than or equal to the bar weight.")
             .index(1)
             .required(true)
             .validator(validate_number))
        .get_matches();

    let bar_weight = value_t!(matches, "bar", u32).unwrap();
    let work_set_weight = value_t!(matches, "work-set", u32).unwrap();
    let sets = value_t!(matches, "sets", u32).unwrap();

    set_weights(bar_weight, work_set_weight, sets);
}

fn set_weights(min: u32, max: u32, sets: u32) {
    // Delta weight between sets
    let numerator = (max - min) as f64;
    let denominator = (sets - 1) as f64;
    let delta = (numerator / denominator).floor() as u32;

    // Round up to the nearest 5
    let remainder = delta % 5;
    let delta = if remainder > 0 {
        delta + 5 - remainder
    } else {
        delta
    };

    for set in 0..sets {
        let weight = min + delta * set;
        let weight = cmp::min(weight, max);

        let plates = get_plates(weight - min);
        println!("{:3}x{}x{} # {:?}", weight, get_reps(set, sets), get_sets(set, sets),
                 plates);
    }
}

fn get_reps(set: u32, sets: u32) -> u32 {
    let lower_bound = set == 0;
    let upper_bound = set == sets - 1;

    if lower_bound || upper_bound {
        5
    } else {
        sets - 1
    }
}

fn get_sets(set: u32, sets: u32) -> u32 {
    let lower_bound = set == 0;
    let upper_bound = set == sets - 1;

    if lower_bound {
        2
    } else if upper_bound {
        3
    } else {
        1
    }
}

fn get_plates(weight: u32) -> Vec<f64> {
    let mut available_plates = vec![2.5, 5.0, 5.0, 10.0, 25.0, 35.0, 45.0];
    let mut required_plates: Vec<f64> = Vec::new();

    if weight == 0 {
        return Vec::new();
    }

    let weight = weight as f64 / 2.0;

    let mut next_sum: f64 = 0.0;

    // Cap iterations to prevent infinite loop in case of no solution
    for _ in 0..10 {
        let mut plate: f64;
        let sum = next_sum;

        // Eliminate (pop) available plates until we find one that doesn't exceed our desired
        // weight
        loop {
            plate = available_plates.pop().unwrap();
            next_sum = sum + plate;

            if next_sum <= weight {
                break;
            }
        }

        required_plates.push(plate);

        // Are we done?
        if next_sum == weight {
            return required_plates;
        } else if next_sum > weight {
            panic!("sum exceeds weight");
        }
    }

    panic!("no solution found");
}

fn validate_number(s: String) -> Result<(), String> {
    s.parse::<usize>().map(|_|()).map_err(|err| err.to_string())
}
