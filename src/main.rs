use clap::{Parser, Subcommand};

pub mod install;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct App {
    /// Main subcommand
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Install,
}

fn main() -> miette::Result<()> {
    let args = App::parse();

    match args.command {
        Commands::Install => {
            println!("it's install!");
            install::installer::install_hook()
        }
    }
}
