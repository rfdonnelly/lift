use lift::{get_plates, get_sets, Set};

use structopt::StructOpt;

const MAX_SETS: u32 = 6;

fn parse_sets(s: &str) -> Result<u32, String> {
    let value = match u32::from_str_radix(s, 10) {
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
    #[structopt(short, long, default_value = "5", parse(try_from_str = parse_sets))]
    sets: u32,

    /// Sets the weight of the work set.  Must be great than or equal to the bar weight.
    work_set: u32,
}

fn main() {
    // let matches = cli::build().get_matches();
    // let cfg = Config::from_matches(&matches);
    let options = Options::from_args();

    let sets = get_sets(options.bar, options.work_set, options.sets);
    print_sets(options.bar, &sets);
}

fn print_sets(base: u32, sets: &Vec<Set>) {
    for set in sets {
        println!("{:>7} {:?}", set.to_string(), get_plates(set.weight - base));
    }
}
