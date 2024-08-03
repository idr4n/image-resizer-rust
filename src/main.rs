mod cli;

use std::path::PathBuf;

fn main() {
    let matches = cli::cli().get_matches();

    let input = matches.get_one::<PathBuf>("input").unwrap();
    println!("The input given was: {:?}", input);
}
