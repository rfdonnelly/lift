use std::cmp;
use std::fmt;

use structopt::StructOpt;

const MAX_SETS: u32 = 6;
const MAX_REPS: u32 = 5;

fn parse_sets(s: &str) -> Result<u32, String> {
    let value =
    match u32::from_str_radix(s, 10) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    if value > MAX_SETS {
        return Err(format!("Must be {} or less", MAX_SETS));
    }

    Ok(value)
}

#[derive(StructOpt, Debug)]
#[structopt(about, author)]
struct Options {
    /// The bar weight.
    #[structopt(short, long, default_value = "45")]
    bar: u32,

    /// The number of sets.
    #[structopt(short, long, default_value = "4", parse(try_from_str = parse_sets))]
    sets: u32,

    /// Sets the weight of the work set.  Must be great than or equal to the bar weight.
    work_set: u32,
}

struct Set {
    weight: u32,
    reps: u32,
    repeat: u32,
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}x{}", self.weight, self.reps, self.repeat)
    }
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}x{}", self.weight, self.reps, self.repeat)
    }
}

fn main() {
    let options = Options::from_args();
    let sets = get_sets(options.bar, options.work_set, options.sets);
    print_sets(options.bar, &sets);
}

fn print_sets(base: u32, sets: &[Set]) {
    for set in sets {
        println!("{:>7} {:?}", set.to_string(), get_plates(set.weight - base));
    }
}

fn get_set_weights(min: u32, max: u32, sets: u32) -> Vec<u32> {
    let mut rv = Vec::new();
    let delta = round_up_5((max - min) / (sets - 1));

    for set in 0..sets {
        // This ensures weight for second to last set != last set
        let set_max = match set {
            n if n == sets - 1 => max,
            _ => max - 5,
        };

        let weight = cmp::min(min + delta * set, set_max);

        rv.push(weight);
    }

    rv
}

fn get_sets(min: u32, max: u32, num_sets: u32) -> Vec<Set> {
    get_set_weights(min, max, num_sets)
        .iter()
        .enumerate()
        .map(|(set_idx, weight)| Set{
            weight: *weight,
            reps: get_reps(set_idx as u32, num_sets),
            repeat: get_set_repeats(set_idx as u32, num_sets),
        })
        .collect()
}

fn round_up_5(x: u32) -> u32 {
    (x + 4) / 5 * 5
}

fn get_reps(set: u32, sets: u32) -> u32 {
    let max = MAX_REPS;
    let upper_bound = sets - 1;

    match set {
        n if n == upper_bound => max,
        n => cmp::max(max - n, 1),
    }
}

fn get_set_repeats(set: u32, sets: u32) -> u32 {
    let lower_bound = 0;
    let upper_bound = sets - 1;

    match set {
        n if n == upper_bound => 3,
        n if n == lower_bound => 2,
        _ => 1,
    }
}

fn get_plates(weight: u32) -> Vec<f64> {
    if weight == 0 {
        return Vec::new();
    }

    let available_plates = vec![45.0, 35.0, 25.0, 10.0, 5.0, 5.0, 2.5];
    let mut required_plates: Vec<f64> = Vec::new();
    let mut available_plates_iter = available_plates.iter();

    let weight = weight as f64 / 2.0;
    let mut next_sum: f64 = 0.0;

    // Cap iterations to prevent infinite loop in case of no solution
    for _ in 0..10 {
        let sum = next_sum;

        // Eliminate available plates until we find one that doesn't exceed our desired weight
        while let Some(plate) = available_plates_iter.next() {
            next_sum = sum + plate;

            if next_sum <= weight {
                required_plates.push(*plate);
                break;
            }
        }

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

    mod get_sets {
        use super::super::*;

        #[test]
        fn typ() {
            assert_eq!(
                format!("{:?}", get_sets(45, 85, 5)),
                "[45x5x2, 55x4x1, 65x3x1, 75x2x1, 85x5x3]"
            );
            assert_eq!(
                format!("{:?}", get_sets(45, 105, 5)),
                "[45x5x2, 60x4x1, 75x3x1, 90x2x1, 105x5x3]"
            );
        }

        #[test]
        fn fractional_delta() {
            assert_eq!(
                format!("{:?}", get_sets(45, 90, 5)),
                "[45x5x2, 60x4x1, 75x3x1, 85x2x1, 90x5x3]"
            );
            assert_eq!(
                format!("{:?}", get_sets(45, 95, 5)),
                "[45x5x2, 60x4x1, 75x3x1, 90x2x1, 95x5x3]"
            );
            assert_eq!(
                format!("{:?}", get_sets(45, 100, 5)),
                "[45x5x2, 60x4x1, 75x3x1, 90x2x1, 100x5x3]"
            );
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

    mod get_set_repeats {
        use super::super::*;

        #[test]
        fn min() {
            assert_eq!(get_set_repeats(0, 5), 2);
        }

        #[test]
        fn max() {
            assert_eq!(get_set_repeats(4, 5), 3);
            assert_eq!(get_set_repeats(0, 1), 3);
        }

        #[test]
        fn mid() {
            assert_eq!(get_set_repeats(3, 5), 1);
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
