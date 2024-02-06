use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Francesco Michele Barranca (kalairendev)", version = "0.1.5", about = "Gospin - GO + HTMX Boilerplate Initiator", long_about = None)]
#[command(propagate_version = true)]
pub struct Gospin {
    pub project: Option<String>,

    #[arg(short = 'p', long = "port", default_value = "8080")]
    pub port: String,
}

#[test]
fn test_gospin_default() {
    let app = Gospin::parse();
    assert_eq!(app.project, None);
    assert_eq!(app.port, "8080");
}

#[test]
fn test_gospin_with_args() {
    let app = Gospin::parse_from(["gospin", "prj", "--port", "3000"]);
    assert_eq!(app.project, Some("prj".to_string()));
    assert_eq!(app.port, "3000");
}
