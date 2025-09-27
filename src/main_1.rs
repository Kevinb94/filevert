use clap::Parser;
use std::{fs, path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(name = "filevert")]
#[command(about = "Convert files between formats", version, author)]
struct Args {
    /// Path to input file (required)
    #[arg(short, long, value_name = "PATH", required = true)]
    input: PathBuf,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    format: Option<String>,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    // Validate early
    match fs::metadata(&args.input) {
        Ok(md) if md.is_file() => {}            // ok, fall through
        Ok(_) => { eprintln!("Not a file: {}", args.input.display()); process::exit(2); }
        Err(e) => { eprintln!("Cannot access {}: {e}", args.input.display()); process::exit(2); }
    }

    println!("Input file: {}", args.input.display());
}
