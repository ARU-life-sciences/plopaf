pub mod paf;
pub mod plot;

use std::path::PathBuf;

// a basic clap cli
use clap::{crate_version, value_parser, Arg, ArgMatches, Command};

// entry point to the cli
pub fn parse_args() -> ArgMatches {
    // define the cli
    let matches = Command::new("plopaf")
        .version(crate_version!())
        .next_line_help(true)
        .help_expected(true)
        .max_term_width(80)
        .arg(
            Arg::new("PAF")
                .help("The input file in PAF format.")
                .id("PAF")
                .value_parser(value_parser!(PathBuf))
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("Output file path.")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .num_args(1)
                .value_parser(value_parser!(PathBuf))
                .default_value("./paf.png"),
        )
        .get_matches();

    matches
}

pub fn run(matches: ArgMatches) {
    let paf_file = matches.get_one::<PathBuf>("PAF").unwrap().clone();
    let out = matches.get_one::<PathBuf>("output").unwrap().clone();

    let coords = paf::generate_alignment_coords(paf_file).unwrap();

    plot::plot(coords, out).unwrap();
}
