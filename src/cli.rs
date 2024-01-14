use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Francesco Michele Barranca (kalairendev)", version = "0.1.5", about = "Gospin - GO + HTMX Boilerplate Initiator", long_about = None)]
#[command(propagate_version = true)]
pub struct Gospin {
    pub project: Option<String>,

    #[arg(short = 'p', long = "port", default_value = "8080")]
    pub port: String,
}
