use std::{fs, path::Path, time::Instant};

use easy_args::ArgSpec;
use lib::dat::Datafile;

use crate::lib::set::CheckedSet;
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

    // Set input directory
    let input = match args.string("input") {
        Some(src) => src,
        None => panic!("No input directory specified."),
    };

    // Verify ROMs
    let matches = CheckedSet::new(dat.check_directory(input).unwrap());

    // Display results
    for rom in matches.results() {
        println!("{}", rom.pretty_print());
    }

    // Copy files if directory specified
    if let Some(output) = args.string("output") {
        println!("Copying Files");
        for item in matches.results() {
            if let Some(name) = item.output_path() {
                let target = Path::new(output).join(name);
                fs::copy(item.file(), target).unwrap();
            }
        }
    }

    // Get elapsed time
    let elapsed = now.elapsed();
    let counts = matches.counts();

    println!("Checked {} roms in {:.2?}", matches.len(), elapsed);
    println!(
        "{} matched exactly. {} matched with wrong filename. {} could not be matched",
        counts.0, counts.1, counts.2
    );
}
