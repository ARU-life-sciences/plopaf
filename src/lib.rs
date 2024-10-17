pub mod paf;
pub mod plot;

use std::path::PathBuf;

// a basic clap cli
use clap::{crate_version, value_parser, Arg, ArgAction, ArgMatches, Command};
use paf::{CigarCoords, CigarCoordsIter};

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
        .arg(
            Arg::new("filter-primary-alignments")
                .help("Remove secondary alignments from the plot.")
                .short('f')
                .long("filter-primary-alignments")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    matches
}

pub fn run(matches: ArgMatches) {
    let paf_file = matches.get_one::<PathBuf>("PAF").unwrap().clone();
    let out = matches.get_one::<PathBuf>("output").unwrap().clone();
    // i.e. remove secondary alignments
    let filter_primary_alignments = matches.get_flag("filter-primary-alignments");

    let coords = paf::generate_alignment_coords(paf_file, filter_primary_alignments).unwrap();
    // kind of annoying - borrow the records.
    let coords: Vec<&CigarCoords> = coords.iter().collect();
    // create the new iterator
    let coordsiter = CigarCoordsIter::new(&coords);

    plot::plot(coordsiter, out, filter_primary_alignments).unwrap();
}
