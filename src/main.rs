use clap::Parser;
use cli::Gospin;
use scaffolder::scaffold;

mod cli;
mod models;
mod scaffolder;

fn main() {
    let args = Gospin::parse();

    match args.project {
        Some(project) => match scaffold(&project) {
            Ok(()) => println!("Created"),
            Err(err) => eprint!("{}", err.message),
        },
        None => eprintln!("No Project name Specified"),
    }
}
