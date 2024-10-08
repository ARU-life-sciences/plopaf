pub mod paf;

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
        .get_matches();

    matches
}

pub fn run(matches: ArgMatches) {
    let paf_file = matches.get_one::<PathBuf>("PAF").unwrap().clone();
    let paf = paf::PAFSeqs::new(paf_file).unwrap();

    println!("{:#?}", paf);
}