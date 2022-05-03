use std::{
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

use easy_args::ArgSpec;
use lib::dat::Datafile;

use crate::lib::{files::File, set::CheckedSet};
mod lib;

fn main() {
    let args = ArgSpec::build()
        .string("dat")
        .string("input")
        .string("output")
        .parse()
        .unwrap();

    // Start timing
    let now = Instant::now();

    // Process dat
    let dat = match args.string("dat") {
        Some(src) => Datafile::from_file(src).unwrap(),
        None => panic!("No DAT file specified."),
    };

    // Load files
    let files = match args.string("input") {
        Some(src) => File::read_dir(src).unwrap(),
        None => panic!("No input directory specified."),
    };

    // Verify ROMs
    let results = CheckedSet::new(dat, files);

    // Display results
    for rom in results.results() {
        println!("{:?}", rom);
    }

    // Get elapsed time
    let elapsed = now.elapsed();

    println!("Checked roms in {:.2?}", elapsed);
}
