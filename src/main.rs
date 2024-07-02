use std::error::Error;

use colored::Colorize;

use yurl::{
    cmd::{Commands, Execute},
    error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = yurl::cmd::Cli::new();
    match args.command {
        Commands::Run(arg) => match arg.run() {
            Ok(()) => {}
            Err(e) => {
                error!(e.to_string())
            }
        },
        Commands::Function(arg) => match arg.run() {
            Ok(()) => {}
            Err(e) => {
                error!(e.to_string())
            }
        },
        Commands::Generate(arg) => match arg.run() {
            Ok(()) => {}
            Err(e) => {
                error!(e.to_string())
            }
        },
    }
    Ok(())
}
