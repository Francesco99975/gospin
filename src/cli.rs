use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Francesco Michele Barranca (kalairendev)", version = "1.0", about = "Gospin - Project Initiator", long_about = None)]
#[command(propagate_version = true)]
pub struct Gospin {
    pub project: Option<String>,

    #[arg(short = 'p', long = "port", default_value = "8080")]
    pub port: String,
}
