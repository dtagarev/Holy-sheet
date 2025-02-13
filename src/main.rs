mod app;
mod pages;
// mod web_view_mode;
use anyhow::Result;
use clap::{Arg, ArgAction, Command};

fn main() -> Result<()> {
    // Просто стартираме логиката от app.rs
    // let matches = Command::new("holysheet")
    //     .version("0.1.0")
    //     .about("Holy Sheet - GTK4 Cheatsheet helper")
    //     .arg(
    //         Arg::new("show")
    //             .long("show")
    //             .help("Name of the cheatsheet to load")
    //             .value_name("NAME")
    //             // Заявяваме, че този аргумент приема точно 1 стойност
    //             .num_args(1)
    //             // Искаме да сетне тази стойност (вместо да е true/false флаг)
    //             .action(ArgAction::Set)
    //     )
    //     .get_matches();

    // // Извличаме стойността на --show (ако е подадена)
    // let show_what = matches.get_one::<String>("show").map(|s| s.as_str());

    // web_view_mode::run_webview_mode("test.md");
    app::run_app()
}