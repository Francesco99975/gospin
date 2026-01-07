use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, Write},
    path::Path,
    process::Command,
};

use crate::{
    cli::run_command,
    errors::ScaffError,
    models::{Injectables, ProjectDir},
};

use console::style;
use regex::Regex;

mod generated_project {
    include!(concat!(env!("OUT_DIR"), "/generated_project.rs"));
}

fn extract_imports(input: &str, username: &str) -> Vec<String> {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut imports = Vec::new();
    let mut in_import = false;

    for line in lines {
        let trimmed = line.trim();
        if trimmed.contains("import (") {
            in_import = true;
            continue;
        }
        if in_import {
            if trimmed.contains(")") {
                in_import = false;
                continue;
            }
            if !trimmed.is_empty() && !trimmed.starts_with("//") {
                if trimmed.starts_with('"') && trimmed.ends_with('"') {
                    let import_path = &trimmed[1..trimmed.len() - 1];
                    if import_path.contains(".")
                        && !import_path.contains("__username__")
                        && !import_path.contains(username)
                    {
                        imports.push(import_path.replace(" ", "").replace("\n", ""));
                    }
                }
            }
        }
    }

    imports
}

pub fn scaffold(
    project: &str,
    ghu: Option<String>,
    port: u32,
    db: bool,
    ws: bool,
    doppler: bool,
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
        doppler,
    };

    let imports_set = dir_builder(root, format!("./{}", project), &injects)?;

    env::set_current_dir(Path::new(&project).join("client")).expect("Could not set dir client");

    run_command("npm", &["run", "generate-assets"]).map_err(|err| ScaffError {
        message: "npm assets error -> ".to_owned() + &err.to_string(),
    })?;

    run_command("npm", &["run", "build"]).map_err(|err| ScaffError {
        message: "npm build error -> ".to_owned() + &err.to_string(),
    })?;

    env::set_current_dir("..").expect("Could not set dir project");
    println!("{} Initializing Project...", style("[2/5]").bold().dim());

    run_command("go", &["mod", "init", &import_str]).map_err(|err| ScaffError {
        message: "go init error -> ".to_owned() + &err.to_string(),
    })?;

    println!("{} Installing Go Packages...", style("[3/5]").bold().dim());

    let mut args = vec!["get"];
    args.extend(imports_set.iter().map(|s| s.as_str()));

    run_command("go", &args).map_err(|err| ScaffError {
        message: "go get error -> ".to_owned() + &err.to_string(),
    })?;

    if db {
        //Development External Comand line tools
        let go_tools = vec![
            "github.com/golang-migrate/migrate/v4/cmd/migrate@latest".to_string(),
            "github.com/sqlc-dev/sqlc/cmd/sqlc@latest".to_string(),
            "-tags 'postgres' github.com/golang-migrate/migrate/v4/cmd/migrate@latest".to_string(),
        ];

        for tool in go_tools {
            Command::new("go")
                .arg("install")
                .arg(tool)
                .output()
                .map_err(|err| ScaffError {
                    message: "go install error <-- ".to_owned() + &err.to_string(),
                })?;
        }
    }

    println!(
        "{} Compiling Boilerplate Templ...",
        style("[4/5]").bold().dim()
    );

    run_command("templ", &["generate"]).map_err(|err| ScaffError {
        message: "templ error -> ".to_owned() + &err.to_string(),
    })?;

    println!("{} Tidying Up...", style("[5/5]").bold().dim());

    run_command("go", &["mod", "tidy"]).map_err(|err| ScaffError {
        message: "tidy error -> ".to_owned() + &err.to_string(),
    })?;

    run_command("make", &["fmt"]).map_err(|err| ScaffError {
        message: "linting error -> ".to_owned() + &err.to_string(),
    })?;

    run_command("make", &["lint"]).map_err(|err| ScaffError {
        message: "linting error -> ".to_owned() + &err.to_string(),
    })?;

    run_command("make", &["vet"]).map_err(|err| ScaffError {
        message: "vetting error -> ".to_owned() + &err.to_string(),
    })?;

    Ok(())
}

fn dir_builder(
    dir: ProjectDir,
    depth: String,
    injects: &Injectables,
) -> Result<HashSet<String>, ScaffError> {
    if dir.dirname == "connections" && !injects.ws
        || (dir.dirname == "database" || dir.dirname == "sql" || dir.dirname == "repository")
            && !injects.db
    {
        return Ok(HashSet::new());
    }

    Command::new("mkdir")
        .arg("-p")
        .arg(depth.clone())
        .output()
        .map_err(|err| ScaffError {
            message: err.to_string(),
        })?;

    let mut imports_set = HashSet::new();

    for mut prj_file in dir.files.unwrap_or(vec![]) {
        if (prj_file.filename == "sqlc.yml".to_string()
            || prj_file.filename == "user-item.templ".to_string()
            || prj_file.filename == "user-list.templ".to_string())
            && !injects.db
        {
            continue;
        }

        if prj_file.filename.ends_with(".env") && injects.doppler {
            continue;
        }

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
            prj_file.content = prj_file.content.replace("==//", "");
            prj_file.content = prj_file.content.replace("#==", "");
            prj_file.content = prj_file.content.replace("==#", "");
        } else {
            let re2 = Regex::new(r"(?s)(//===|#==).*?(===//|==#)").unwrap();
            prj_file.content = re2.replace_all(&prj_file.content, "").to_string();

            let re = Regex::new(r"(//|#)==[^\n]*\n").unwrap();
            prj_file.content = re.replace_all(&prj_file.content, "").to_string();
        }

        if injects.doppler {
            prj_file.content = prj_file.content.replace("//%%", "");
            prj_file.content = prj_file.content.replace("#%%", "");

            let re = Regex::new(r"(//|#)%-[^\n]*\n").unwrap();
            prj_file.content = re.replace_all(&prj_file.content, "").to_string();
        } else {
            prj_file.content = prj_file.content.replace("//%-", "");
            prj_file.content = prj_file.content.replace("#%-", "");

            let re = Regex::new(r"(//|#)%%[^\n]*\n").unwrap();
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

        let extracted_strings = extract_imports(&prj_file.content, injects.username.as_str());

        imports_set.extend(extracted_strings);
    }

    if dir.dirname == "client" {
        env::set_current_dir(Path::new(&depth)).expect("Could not set dir");
        println!("{} Running npm install...", style("[1/5]").bold().dim());
        run_command("npm", &["install"]).map_err(|err| ScaffError {
            message: "npm install error -> ".to_owned() + &err.to_string(),
        })?;

        env::set_current_dir(Path::new("../..")).expect("Could not set dir root");
    }

    for prj_dir in dir.dirs.unwrap_or(vec![]) {
        let new_depth = prj_dir.dirname.clone();
        imports_set.extend(dir_builder(
            prj_dir,
            depth.clone() + "/" + &new_depth,
            injects,
        )?);
    }

    Ok(imports_set)
}
