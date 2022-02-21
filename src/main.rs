use clap::Parser;
use std::{fs, io::Write, path::Path};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Text of note
    #[clap(short, long)]
    note: String,
    /// Category for note to be saved under
    #[clap(short, long)]
    category: String,
}

fn main() {
    let args = Args::parse();
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(
            Path::new(&dirs_next::home_dir().unwrap())
                .join("notes")
                .join(args.category)
                .join("quick_notes.txt"),
        )
        .unwrap();
    if let Err(e) = writeln!(file, "{}", args.note) {
        eprintln!("Could not write to file: {}", e);
    }
}
