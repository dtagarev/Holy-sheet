mod app;
mod pages;

use anyhow::Result;
use gtk::prelude::*;
use gtk::Application;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let application = Application::new(
        Some("com.app.holy-sheet"),
        Default::default(),
    );

    application.connect_activate(move |app| {
        // Check if the --show argument is provided
        // if args.len() > 1 && args[1] == "--show" {
        //     if args.len() > 2 {
        //         let file_name = &args[2];
        //         println!("Filename: {:?}", file_name);
                // if let Err(e) = pages::markdown_viewer::run_markdown_viewer("test.md") {
                //     eprintln!("Error running markdown viewer: {:?}", e);
                // }
        //     } else {
        //         eprintln!("Error: No filename provided for --show argument");
        //     }
        // } else {
            // if let Err(e) = app::run_app_with_application(app) {
            //     eprintln!("Error running app: {:?}", e);
            // }
        // }
    });

    if let Err(e) = pages::markdown_viewer::run_markdown_viewer("test.md") {
        eprintln!("Error running markdown viewer: {:?}", e);
    }
    
    // application.run();

    Ok(())
}