use clap::Parser;
use cli::Gospin;
use scaffolder::scaffold;

mod cli;
mod errors;
mod models;
mod scaffolder;

fn main() {
    let args = Gospin::parse();

    match args.project {
        Some(project) => match scaffold(
            &project,
            args.ghu,
            args.port.parse::<u32>().unwrap_or(8080),
            args.db,
            args.ws,
        ) {
            Ok(_) => println!("GO Project Created"),
            Err(err) => eprintln!("{}", err.to_string()),
        },
        None => eprintln!("No Project name Specified"),
    }
}
