#[macro_use]
extern crate clap;

mod cli;
mod config;

use std::cmp;
use config::Config;

fn main() {
    let matches = cli::build().get_matches();
    let cfg = Config::from_matches(&matches);

    get_sets(cfg.bar, cfg.work_set, cfg.sets);
}

fn get_sets(min: u32, max: u32, sets: u32) {
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
        println!("{:3}x{}x{} # {:?}", weight, get_reps(set, sets), get_sub_sets(set, sets),
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

fn get_sub_sets(set: u32, sets: u32) -> u32 {
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
        let mut plate: f64 = 0.0;
        let sum = next_sum;

        // Eliminate (pop) available plates until we find one that doesn't exceed our desired
        // weight
        while !available_plates.is_empty() {
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

#[cfg(test)]
mod tests {
    mod get_plates {
        use super::super::*;

        #[test]
        fn min() {
            assert_eq!(get_plates(5), vec!(2.5));
        }

        #[test]
        fn max() {
            assert_eq!(get_plates(255), vec!(45.0, 35.0, 25.0, 10.0, 5.0, 5.0, 2.5));
        }

        #[test]
        fn mid() {
            assert_eq!(get_plates(90), vec!(45.0));
            assert_eq!(get_plates(30), vec!(10.0, 5.0));
        }

        #[test]
        #[should_panic(expected = "sum exceeds weight")]
        fn too_small() {
            get_plates(4);
        }

        #[test]
        #[should_panic(expected = "no solution found")]
        fn not_multiple_of_five() {
            get_plates(6);
        }

        #[test]
        #[should_panic(expected = "no solution found")]
        fn too_large() {
            get_plates(301);
        }
    }
}
