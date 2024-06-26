use lift::{plates, supersets, Distribution, SuperSet};

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
/// A calculator for strength training.
struct Cli {
    /// The bar weight.
    #[arg(short, long, default_value = "45")]
    bar: u32,

    /// The number of sets.
    #[arg(short, long, default_value = "4", value_parser = clap::value_parser!(u32).range(1..=6))]
    sets: u32,

    /// Sets the weight of the work set.  Must be great than or equal to the bar weight.
    work_set: u32,
}

fn main() {
    let cli = Cli::parse();
    let sets = supersets(cli.bar, cli.work_set, cli.sets, Distribution::Sin);
    print_sets(cli.bar, &sets);
}

fn print_sets(base: u32, sets: &[SuperSet]) {
    for set in sets {
        println!("{:>7} {:?}", set.to_string(), plates(set.weight - base));
    }
}
