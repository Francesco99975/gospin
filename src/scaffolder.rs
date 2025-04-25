use std::{
    env,
    fs::File,
    io::{self, Write},
    path::Path,
    process::Command,
};

use crate::{
    errors::ScaffError,
    models::{Injectables, ProjectDir},
};

use console::style;
use regex::Regex;

mod generated_project {
    include!(concat!(env!("OUT_DIR"), "/generated_project.rs"));
}

pub fn scaffold(
    project: &str,
    ghu: Option<String>,
    port: u32,
    db: bool,
    ws: bool,
) -> Result<(), ScaffError> {
    let username = match ghu {
        Some(u) => u,
        None => {
            // Create a mutable String to store the user input
            let mut input_string = String::new();

            // Print a prompt to the user
            print!("Please enter you Github username: ");

            // Flush the output to ensure the prompt is displayed immediately
            io::stdout().flush().unwrap();

            // Read the user input into the String
            io::stdin()
                .read_line(&mut input_string)
                .map_err(|err| ScaffError {
                    message: "go init error -> ".to_owned() + &err.to_string(),
                })?;

            input_string.trim().replace("\n", "")
        }
    };

    let import_str = format!("github.com/{}/{}", username, project);

    let root = generated_project::PROJECT_DIR.clone();

    let injects = Injectables {
        project_name: project.to_string(),
        username,
        port,
        db,
        ws,
    };

    dir_builder(root, format!("./{}", project), &injects)?;

    env::set_current_dir(Path::new(&project)).expect("Could not set dir project");
    println!("{} Initializing Project...", style("[2/5]").bold().dim());
    Command::new("go")
        .arg("mod")
        .arg("init")
        .arg(import_str.clone())
        .output()
        .map_err(|err| ScaffError {
            message: "go init error -> ".to_owned() + &err.to_string(),
        })?;

    println!("{} Installing Go Packages...", style("[3/5]").bold().dim());
    Command::new("go")
        .arg("get")
        .args(vec![
            "github.com/labstack/echo/v4",
            "github.com/labstack/echo/v4/middleware",
            "github.com/a-h/templ",
            "github.com/joho/godotenv",
            "github.com/google/uuid",
            "github.com/jmoiron/sqlx",
            "github.com/lib/pq",
        ])
        .output()
        .map_err(|err| ScaffError {
            message: "go init error -> ".to_owned() + &err.to_string(),
        })?;

    println!(
        "{} Compiling Boilerplate Templ...",
        style("[4/5]").bold().dim()
    );
    Command::new("templ")
        .arg("generate")
        .output()
        .map_err(|err| ScaffError {
            message: "templ error -> ".to_owned() + &err.to_string(),
        })?;

    println!("{} Tidying Up...", style("[5/5]").bold().dim());
    Command::new("go")
        .arg("mod")
        .arg("tidy")
        .output()
        .map_err(|err| ScaffError {
            message: "go init error -> ".to_owned() + &err.to_string(),
        })?;

    Ok(())
}

fn dir_builder(dir: ProjectDir, depth: String, injects: &Injectables) -> Result<(), ScaffError> {
    if dir.dirname == "connections" && !injects.ws
        || (dir.dirname == "database" || dir.dirname == "sql") && !injects.db
    {
        return Ok(());
    }

    Command::new("mkdir")
        .arg("-p")
        .arg(depth.clone())
        .output()
        .map_err(|err| ScaffError {
            message: err.to_string(),
        })?;

    for mut prj_file in dir.files.unwrap_or(vec![]) {
        prj_file.content = prj_file
            .content
            .replace("go_boilerplate", injects.project_name.as_str());
        prj_file.content = prj_file
            .content
            .replace("__username__", injects.username.as_str());
        prj_file.content = prj_file
            .content
            .replace("__port__", injects.port.to_string().as_str());

        if injects.ws {
            prj_file.content = prj_file.content.replace("//--", "");
            prj_file.content = prj_file.content.replace("#--", "");
        } else {
            let re = Regex::new(r"(//|#)--[^\n]*\n").unwrap();
            prj_file.content = re.replace_all(&prj_file.content, "").to_string();
        }

        if injects.db {
            prj_file.content = prj_file.content.replace("//==", "");
            prj_file.content = prj_file.content.replace("#==", "");
        } else {
            let re = Regex::new(r"(//|#)==[^\n]*\n").unwrap();
            prj_file.content = re.replace_all(&prj_file.content, "").to_string();
        }

        let mut file =
            File::create(depth.clone() + "/" + &prj_file.filename).map_err(|err| ScaffError {
                message: err.to_string(),
            })?;
        file.write_all(prj_file.content.as_bytes())
            .map_err(|err| ScaffError {
                message: err.to_string(),
            })?;
    }

    if dir.dirname == "client" {
        env::set_current_dir(Path::new(&depth)).expect("Could not set dir");
        println!("{} Running npm install...", style("[1/5]").bold().dim());
        Command::new("npm")
            .arg("install")
            .output()
            .map_err(|err| ScaffError {
                message: "npm error -> ".to_owned() + &err.to_string(),
            })?;

        env::set_current_dir(Path::new("../..")).expect("Could not set dir root");
    }

    for prj_dir in dir.dirs.unwrap_or(vec![]) {
        let new_depth = prj_dir.dirname.clone();
        dir_builder(prj_dir, depth.clone() + "/" + &new_depth, injects)?;
    }

    Ok(())
}
