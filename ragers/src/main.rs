use clap::Parser;

use ragers::cli::Cli;
use ragers::run;

fn main() {
    let cli = Cli::parse();
    match run(cli) {
        Ok(_) => {
            std::process::exit(0);
        } // Execution is done
        Err(err) => {
            println!("error: {err}");
            std::process::exit(1);
        }
    }
}
