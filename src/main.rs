extern crate clap;
extern crate num;

mod lib;
mod testrunner;
mod tests;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Practical Astronomy")
        .arg(
            Arg::with_name("tests")
                .short("t")
                .long("tests")
                .takes_value(false)
                .help("Run unit tests"),
        )
        .get_matches();

    if matches.is_present("tests") {
        testrunner::run_tests();
    }
}
