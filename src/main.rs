use structopt::StructOpt;
use std::{io, fs::File};

#[derive(StructOpt)]
struct Args {
    files: Vec<String>,
    out_file: String,
}

fn main() {
    let args = Args::from_args();
    let mut out_file = File::create(args.out_file).unwrap();
    args.files.iter()
        .map(File::open)
        .map(Result::unwrap)
        .map(|mut in_file| io::copy(&mut in_file, &mut out_file))
        .map(Result::unwrap)
        .for_each(|_|());
}
