use std::error::Error;

use yurl::cmd::{Commands, Execute};

fn main() -> Result<(), Box<dyn Error>> {
    let args = yurl::cmd::Cli::new();
    match args.command {
        Commands::Run(arg) => {
            match arg.run() {
                Ok(()) => {}
                Err(e) => {println!("{}",e.to_string())}
            }
        }
        Commands::Function(arg) => {
            match arg.run() {
                Ok(()) => {}
                Err(e) => {println!("{}",e.to_string())}
            }
        }
    }
    Ok(())
}
