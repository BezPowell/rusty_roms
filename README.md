# rusty_roms
Rusty Roms is a simple command-line application to check game roms against no-intro dat files. It can both verify roms, and optionally copy and rename those that match.

Rusty Roms is not complete yet, but is already usable. The automated test will fail on your system, as they rely on actual ROMS, which almost certainly do not allow redistribution; any suggestions for test roms that could be shipped with the source-code are most appreciated!

## Features
- Single binary - no dependencies.
- Comparison on SHA1 - most accurate format currently provided by no-intro.
- NES rom support via header detection.
- Cross-platform - developed and tested on Linux, but should work across Mac and Windows too.
- Reasonably fast - Verifies and copies all the roms from the Sega Mega Drive & Genesis Classics on my machine in under 200ms.

## Usage
To check all roms in a folder and report on verification status:
```
rusty_roms --dat <path_to_dat_file> --input <path_to_input_directory>
```
To verify roms and copy all matches renamed to another directory:
```
rusty_roms --dat <path_to_dat_file> --input <path_to_input_directory> --output <path_to_output_directory>
```
## Compiling
Assuming you already have Rust and Cargo installed, compilation is simple.
```
git clone https://github.com/BezPowell/rusty_roms.git
cd rusty_roms
cargo build --release
```
The binary file will then be located in the `target/release` folder.

## TODO
- Improve error handling - will currently crash if output directory does not already exist.
- Add option to move files, instead of copy.
- Better documentation.
- Find roms that allow redistribution for tests.