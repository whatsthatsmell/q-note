use clap::Parser;
use std::{fs, io::Write};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Text of note
    #[clap(short, long)]
    note: String,
    // todo: add category
    // todo: new line
    // todo: etc...
}

fn main() {
    let args = Args::parse();
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("notes.txt")
        .unwrap();
    file.write_all(args.note.as_bytes());
}
