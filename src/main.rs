use std::{fs, path::Path, time::Instant};

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
    let roms = match args.string("dat") {
        Some(src) => Datafile::from_file(src).unwrap().roms(),
        None => panic!("No DAT file specified."),
    };

    // Load files
    let files = match args.string("input") {
        Some(src) => File::read_dir(src).unwrap(),
        None => panic!("No input directory specified."),
    };

    // Verify ROMs
    let results = CheckedSet::new(&roms, &files);

    // Display results
    for file in results.results() {
        println!("{} {}", file.0.name().unwrap(), file.1.pretty_print());
    }

    // Copy files if directory specified
    if let Some(output) = args.string("output") {
        println!("Copying Files");
        for item in results.results() {
            if let Some(name) = item.1.output_path() {
                let target = Path::new(output).join(name);
                fs::copy(item.0.path(), target).unwrap();
            }
        }
    }

    // Get elapsed time
    let elapsed = now.elapsed();
    let counts = results.counts();

    println!("Checked {} roms in {:.2?}", results.len(), elapsed);
    println!(
        "{} matched exactly. {} matched with wrong filename. {} could not be matched",
        counts.0, counts.1, counts.2
    );
}
