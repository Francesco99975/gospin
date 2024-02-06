use std::{
    env,
    fs::File,
    io::{self, Write},
    path::Path,
    process::Command,
};

use crate::{
    errors::ScaffError,
    models::{generate_project_structure, ProjectDir},
};

pub fn scaffold(project: &str, port: u32) -> Result<(), ScaffError> {
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

    let username = input_string.trim();

    let import_str = format!("github.com/{}/{}", username, project);

    let root = generate_project_structure(project, port, &import_str);

    dir_builder(root, format!("./{}", project))?;

    env::set_current_dir(Path::new(&project)).expect("Could not set dir project");
    println!("Running go init");
    Command::new("go")
        .arg("mod")
        .arg("init")
        .arg(import_str.clone())
        .output()
        .map_err(|err| ScaffError {
            message: "go init error -> ".to_owned() + &err.to_string(),
        })?;

    Command::new("go")
        .arg("get")
        .args(vec![
            "github.com/labstack/echo/v4",
            "github.com/a-h/templ",
            "github.com/joho/godotenv",
        ])
        .output()
        .map_err(|err| ScaffError {
            message: "go init error -> ".to_owned() + &err.to_string(),
        })?;

    Command::new("go")
        .arg("mod")
        .arg("tidy")
        .output()
        .map_err(|err| ScaffError {
            message: "go init error -> ".to_owned() + &err.to_string(),
        })?;

    Ok(())
}

fn dir_builder(dir: ProjectDir, depth: String) -> Result<(), ScaffError> {
    Command::new("mkdir")
        .arg("-p")
        .arg(depth.clone())
        .output()
        .map_err(|err| ScaffError {
            message: err.to_string(),
        })?;

    for prj_file in dir.files.unwrap_or(vec![]) {
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
        println!("Running npm");
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
        dir_builder(prj_dir, depth.clone() + "/" + &new_depth)?;
    }

    Ok(())
}

#[test]
fn test_generate_project_structure() {
    let proj_dir = generate_project_structure("testproj", 8080, "github.com/test");

    assert!(proj_dir.dirs.unwrap().len() > 0);
    assert!(proj_dir.files.unwrap().len() > 0);
}
