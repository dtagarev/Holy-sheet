mod app;
mod pages;

use anyhow::Result;
use clap::{Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the cheatsheet to load
    #[arg(long)]
    show: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Check if the --show argument is provided
    if let Some(file_name) = args.show {
        println!("Filename: {:?}", file_name);
        if let Err(e) = pages::markdown_viewer::run_markdown_viewer(&file_name) {
            eprintln!("Error running markdown viewer: {:?}", e);
        }
    } else {
        app::run_app()?;
    }

    Ok(())
}