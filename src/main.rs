#[macro_use]
extern crate clap;

mod cli;
mod config;
mod set;

use std::cmp;

use config::Config;
use set::Set;

fn main() {
    let matches = cli::build().get_matches();
    let cfg = Config::from_matches(&matches);

    let sets = get_sets(cfg.bar, cfg.work_set, cfg.sets);
    print_sets(cfg.bar, &sets);
}

fn print_sets(base: u32, sets: &Vec<Set>) {
    for set in sets {
        println!("{:>7} {:?}", set.to_string(), get_plates(set.weight - base));
    }
}

fn get_sets(min: u32, max: u32, sets: u32) -> Vec<Set> {
    let mut rv = Vec::new();
    let delta = round_up_5((max - min) / (sets - 1));

    for set in 0..sets {
        let weight = cmp::min(
            min + delta * set,
            max
            );

        rv.push(Set {
            weight: weight,
            reps: get_reps(set, sets),
            sets: get_sub_sets(set, sets),
        });
    }

    rv
}

fn round_up_5(x: u32) -> u32 {
    (x + 4) / 5 * 5
}

fn get_reps(set: u32, sets: u32) -> u32 {
    let max = 5;
    let upper_bound = sets - 1;

    match set {
        n if n == upper_bound => max,
        n => cmp::max(max - n, 1),
    }
}

fn get_sub_sets(set: u32, sets: u32) -> u32 {
    let lower_bound = 0;
    let upper_bound = sets - 1;

    match set {
        n if n == upper_bound => 3,
        n if n == lower_bound => 2,
        _ => 1,
    }
}

fn get_plates(weight: u32) -> Vec<f64> {
    let available_plates = vec![2.5, 5.0, 5.0, 10.0, 25.0, 35.0, 45.0];
    let mut idx: i32 = (available_plates.len() as i32) - 1;
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
        while idx >= 0 {
            plate = available_plates[idx as usize];
            idx -= 1;
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
    mod round_up_5 {
        use super::super::*;

        #[test]
        fn compare() {
            assert_eq!(round_up_5(0), 0);
            assert_eq!(round_up_5(1), 5);
            assert_eq!(round_up_5(2), 5);
            assert_eq!(round_up_5(3), 5);
            assert_eq!(round_up_5(4), 5);
            assert_eq!(round_up_5(5), 5);
            assert_eq!(round_up_5(6), 10);
        }
    }

    mod get_reps {
        use super::super::*;

        #[test]
        fn min() {
            assert_eq!(get_reps(0, 5), 5);
        }

        #[test]
        fn mid_nominal() {
            assert_eq!(get_reps(1, 5), 4);
            assert_eq!(get_reps(2, 5), 3);
            assert_eq!(get_reps(3, 5), 2);
            assert_eq!(get_reps(4, 6), 1);
        }

        #[test]
        fn mid_min() {
            assert_eq!(get_reps(5, 7), 1);
            assert_eq!(get_reps(5, 9), 1);
        }

        #[test]
        fn max() {
            assert_eq!(get_reps(4, 5), 5);
        }
    }

    mod get_sub_sets {
        use super::super::*;

        #[test]
        fn min() {
            assert_eq!(get_sub_sets(0, 5), 2);
        }

        #[test]
        fn max() {
            assert_eq!(get_sub_sets(4, 5), 3);
            assert_eq!(get_sub_sets(0, 1), 3);
        }

        #[test]
        fn mid() {
            assert_eq!(get_sub_sets(3, 5), 1);
        }
    }

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
