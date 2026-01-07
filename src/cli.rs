use clap::Parser;
use std::ffi::OsStr;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(author = "Francesco Michele Barranca (kalairendev)", version = "0.2.1", about = "Gospin - GO Boilerplate Initiator", long_about = None)]
#[command(propagate_version = true)]
pub struct Gospin {
    pub project: Option<String>,

    #[arg(short = 'u', long = "username")]
    pub ghu: Option<String>,

    #[arg(short = 'p', long = "port", default_value = "8080")]
    pub port: String,

    #[arg(short = 'd', long = "database", default_value = "false")]
    pub db: bool,

    #[arg(short = 'w', long = "websockets", default_value = "false")]
    pub ws: bool,

    #[arg(long = "doppler", default_value = "false")]
    pub doppler: bool,
}

#[derive(Debug)]
pub struct ScaffError {
    pub message: String,
}

impl std::fmt::Display for ScaffError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ScaffError {}

pub fn run_command<I, S>(cmd: &str, args: I) -> Result<(), ScaffError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    // 1. Spawn the command
    let mut child = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| ScaffError {
            message: format!("Failed to spawn `{}`: {}", cmd, e),
        })?;

    // 2. Take stdout/stderr
    let stdout = child.stdout.take().ok_or_else(|| ScaffError {
        message: "Failed to capture stdout".into(),
    })?;
    let stderr = child.stderr.take().ok_or_else(|| ScaffError {
        message: "Failed to capture stderr".into(),
    })?;

    let out_reader = BufReader::new(stdout);
    let err_reader = BufReader::new(stderr);

    // 3. Stream output in separate threads (interleaved + colored)
    let out_handle = std::thread::spawn(move || {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        for line in out_reader.lines().flatten() {
            let _ = writeln!(handle, "{}", line);
        }
    });

    let err_handle = std::thread::spawn(move || {
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        for line in err_reader.lines().flatten() {
            let _ = writeln!(handle, "{}", line);
        }
    });

    // 4. Wait for process to finish
    let status = child.wait().map_err(|e| ScaffError {
        message: format!("Failed to wait on `{}`: {}", cmd, e),
    })?;

    // 5. Wait for streaming threads
    out_handle.join().unwrap();
    err_handle.join().unwrap();

    // 6. Check exit status
    if !status.success() {
        return Err(ScaffError {
            message: format!("Command `{}` failed with status: {}", cmd, status),
        });
    }

    Ok(())
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
