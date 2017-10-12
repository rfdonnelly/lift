use clap::App;
use clap::Arg;
use clap::AppSettings;

pub fn build<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
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
}

fn validate_number(s: String) -> Result<(), String> {
    s.parse::<usize>().map(|_|()).map_err(|err| err.to_string())
}
