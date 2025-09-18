// Use the `clap` crate (with 'derive' pattern)...
// ...a Command Line Argument Parser for Rust
// https://docs.rs/clap/latest/clap/
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]

/// A Rust version of the `echo` utility
// is this even working right yet?
struct Args {
    /// Input text
    #[arg(required(true))]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short('n'))]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}
