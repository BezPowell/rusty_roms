use easy_args::{arg_spec, ArgSpec};
use lib::files::check_files;
use sha1::digest::typenum::Le;

mod lib;

fn main() {
    let args = ArgSpec::build()
        .string("dat")
        .string("input")
        .string("output")
        .parse()
        .unwrap();
}
