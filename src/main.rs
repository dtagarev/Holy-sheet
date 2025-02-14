mod app;
mod pages;

use anyhow::Result;
use gdk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::{Application};
use gtk::glib::{Char, OptionArg, OptionFlags};
use std::env;
use std::fmt::Pointer;

fn main() -> Result<()> {
    let application = Application::new(
        Some("com.app.holy-sheet"),
        ApplicationFlags::HANDLES_COMMAND_LINE,
    );

    // Add the --show option
    application.add_main_option(
        "show",
    Char::from(b's'),
        OptionFlags::IN_MAIN,
        OptionArg::String,
        "Show a markdown file",
        None,
    );

    application.connect_activate(move |app| {
        // Default behavior when no command-line arguments are provided
        if let Err(e) = app::run_app_with_application(app) {
            eprintln!("Error running app: {:?}", e);
        }
    });

    application.connect_command_line(move |app, cmd_line| {
        
        // if args.contains(&std::ffi::OsString::from("--show")) {
            if let Some(file_name) = cmd_line.options_dict().lookup_value("show", None) {
                println!("Filename: {:?}", file_name.to_string().trim_matches('\''));
                if let Err(e) = pages::markdown_viewer::setup_markdown_viewer(app, 
                    &(file_name.to_string().trim_matches('\'').to_string() + ".md")) {
                    eprintln!("Error running markdown viewer: {:?}", e);
                }
                return 0;
            } else {
                eprintln!("Error: No filename provided for --show argument");
                return 1;
            }
        // }

        // app.activate();
        // 0
    });

    application.run();

    Ok(())
}