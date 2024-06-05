use std::cmp;
use std::f32::consts::PI;
use std::fmt;

const MAX_REPS: u32 = 5;

pub enum Distribution {
    Linear,
    Sin,
}

type DistributionFn = fn(f32, f32) -> f32;

pub struct Set {
    /// Total weight of the set
    pub weight: u32,
    /// The number of repititions
    reps: u32,
    /// The number of times the set is repeated
    sets: u32,
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}x{}", self.weight, self.reps, self.sets)
    }
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}x{}", self.weight, self.reps, self.sets)
    }
}

fn distribution_linear(x: f32, _delta_normalized: f32) -> f32 {
    dbg!(&(x, x.ceil(), _delta_normalized));
    x.ceil()
}

fn distribution_sin(x: f32, delta_normalized: f32) -> f32 {
    delta_normalized * (PI * x / delta_normalized / 2.0).sin()
}

fn weights(
    min: u32,
    max: u32,
    num_sets: u32,
    distribution: DistributionFn,
) -> impl Iterator<Item = u32> {
    let delta = max - min;
    let delta_normalized = delta as f32 / 5.0;
    let increment = delta_normalized / (num_sets - 1) as f32;

    (0..num_sets)
        // Create even spread
        .map(move |x| x as f32 * increment)
        // Distribute
        .map(move |x| distribution(x, delta_normalized))
        // Convert
        .map(|x| x as u32)
        // Denormalize
        .map(|x| x * 5)
        // Offset
        .map(move |x| x + min)
}

pub fn get_sets(min: u32, max: u32, sets: u32, distribution: Distribution) -> Vec<Set> {
    let distribution = match distribution {
        Distribution::Linear => distribution_linear,
        Distribution::Sin => distribution_sin,
    };

    weights(min, max, sets, distribution)
        .enumerate()
        .map(|(set_idx, weight)| Set {
            weight,
            reps: get_reps(set_idx as u32, sets),
            sets: get_sub_sets(set_idx as u32, sets),
        })
        .collect()
}

fn get_reps(set: u32, sets: u32) -> u32 {
    let max = MAX_REPS;
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

pub fn get_plates(weight: u32) -> Vec<f64> {
    if weight == 0 {
        return Vec::new();
    }

    let available_plates = [45.0, 35.0, 25.0, 10.0, 5.0, 5.0, 2.5];
    let mut required_plates: Vec<f64> = Vec::new();
    let mut available_plates_iter = available_plates.iter();

    let weight = weight as f64 / 2.0;
    let mut next_sum: f64 = 0.0;

    // Cap iterations to prevent infinite loop in case of no solution
    for _ in 0..10 {
        let sum = next_sum;

        // Eliminate available plates until we find one that doesn't exceed our desired weight
        for plate in available_plates_iter.by_ref() {
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
    mod get_sets_linear {
        use super::super::*;

        #[test]
        fn typ() {
            assert_eq!(
                format!("{:?}", get_sets(45, 85, 5, Distribution::Sin)),
                "[45x5x2, 60x4x1, 70x3x1, 80x2x1, 85x5x3]"
            );
            assert_eq!(
                format!("{:?}", get_sets(45, 105, 5, Distribution::Sin)),
                "[45x5x2, 65x4x1, 85x3x1, 100x2x1, 105x5x3]"
            );
        }

        #[test]
        fn fractional_delta() {
            assert_eq!(
                format!("{:?}", get_sets(45, 90, 5, Distribution::Sin)),
                "[45x5x2, 60x4x1, 75x3x1, 85x2x1, 90x5x3]"
            );
            assert_eq!(
                format!("{:?}", get_sets(45, 95, 5, Distribution::Sin)),
                "[45x5x2, 60x4x1, 80x3x1, 90x2x1, 95x5x3]"
            );
            assert_eq!(
                format!("{:?}", get_sets(45, 100, 5, Distribution::Sin)),
                "[45x5x2, 65x4x1, 80x3x1, 95x2x1, 100x5x3]"
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
