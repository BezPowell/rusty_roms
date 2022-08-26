use crate::lib::App;
use easy_args::ArgSpec;
use std::{error::Error, time::Instant};
mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    let args = ArgSpec::build()
        .string("dat")
        .string("input")
        .string("output")
        .parse()
        .unwrap();

    // Start timing
    let now = Instant::now();

    // Check arguments
    let dat_path = args.string("dat").expect("No DAT file specified.");
    let file_dir = args.string("input").expect("No input directory specified.");

    // Build App
    let app = App::new(dat_path, file_dir)?;

    // Verify files
    let results = app.verify();

    // Print results
    app.report(&results);

    let elapsed = now.elapsed();
    println!("Finished verifying files in {:.2?}", elapsed);

    Ok(())
}
