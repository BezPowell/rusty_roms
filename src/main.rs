use std::{fs, path::Path, time::Instant};

use easy_args::ArgSpec;
use lib::dat::Datafile;
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
    let matches = dat.check_directory(input).unwrap();

    // Copy files if directory specified
    if let Some(output) = args.string("output") {
        for item in &matches {
            if let Some(name) = item.output_path() {
                let target = Path::new(output).join(name);
                fs::copy(item.file(), target).unwrap();
            }
        }
    }

    // Get elapsed time
    let elapsed = now.elapsed();
    let total = matches.len();

    // Just print out matches for now
    for rom in matches {
        println!("{:?}", rom);
    }
    println!("Checked {} roms in {:.2?}", total, elapsed);
}
