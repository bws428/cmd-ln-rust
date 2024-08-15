// Use the `clap` crate...
// ...a Command Line Argument Parser for Rust
// https://docs.rs/clap/latest/clap/

use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("bws428")
        .about("Rust version of `echo`")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();

    // let text = matches.values_of_lossy("text").unwrap();

    println!("{:#?}", matches)
}
