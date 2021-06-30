use clap::{Clap};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

// コマンドライン設定
#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "KamuiRoeru",
    about = "Super awesome sample RPN calculator",
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,
    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
    /// Number. Must be 0 <= num <= 255
    #[clap(short, long, default_value = "0")]
    num: u8,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

fn run<R: BufRead>(reader: R, _verbose: bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}