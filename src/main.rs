mod converters;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use converters::{Format, convert};

#[derive(Parser, Debug)]
#[command(name="filevert", about="Convert files between formats")]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Convert {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        format: Format,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Prompt,
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Convert { input, format, output } => {
            run_convert(input, format, output, cli.verbose);
        }
        Cmd::Prompt => {
            run_prompt(cli.verbose);
        }
    }
}

fn run_convert(input: PathBuf, out_fmt: Format, output: Option<PathBuf>, verbose: bool) {
    println!("fn: run_convert");
    println!("  verbose: {}", verbose);

    if !input.is_file() {
        eprintln!("✗ Input file not found: {}", input.display());
        std::process::exit(1);
    }

    // default output path = same name with new extension
    let out_path = output.unwrap_or_else(|| {
        let mut p = input.clone();
        p.set_extension(format!("{:?}", out_fmt).to_lowercase());
        p
    });

    // infer input format from extension (very naive for now)
    let in_fmt = match input.extension().and_then(|e| e.to_str()) {
        Some("csv") => Format::Csv,
        Some("json") => Format::Json,
        Some("xml") => Format::Xml,
        _ => {
            eprintln!("✗ Could not infer input format from {}", input.display());
            std::process::exit(2);
        }
    };

    // call the dispatcher in converters.rs
    match convert(in_fmt, out_fmt, &input, &out_path) {
        Ok(()) => println!("✔ Conversion complete: {}", out_path.display()),
        Err(e) => {
            eprintln!("✗ Conversion failed: {e}");
            std::process::exit(1);
        }
    }
}

fn run_prompt(verbose: bool) {
    println!("fn: run_prompt");
    println!("  verbose: {}", verbose);
}
