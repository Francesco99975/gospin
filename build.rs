use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    // Process the directory to generate the structure
    let dir_structure = match process_dir("go_boilerplate") {
        Ok(structure) => structure,
        Err(e) => {
            eprintln!("Error processing directory: {}", e);
            return;
        }
    };

    // Get the output directory from the environment
    let out_dir = match std::env::var("OUT_DIR") {
        Ok(out_dir) => out_dir,
        Err(e) => {
            eprintln!("Error getting OUT_DIR: {}", e);
            return;
        }
    };

    let dest_path = Path::new(&out_dir).join("generated_project.rs");
    let mut f = match File::create(&dest_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return;
        }
    };

    // Write the generated content to the file
    if let Err(e) = write!(f, "use crate::models::{{ProjectDir, ProjectFile}};\n") {
        eprintln!("Error writing to file: {}", e);
        return;
    }

    if let Err(e) = write!(f, "use once_cell::sync::Lazy;\n") {
        eprintln!("Error writing to file: {}", e);
        return;
    }

    if let Err(e) = write!(
        f,
        "pub static PROJECT_DIR: Lazy<ProjectDir> = Lazy::new(|| {{ {} }});\n",
        dir_structure
    ) {
        eprintln!("Error writing to file: {}", e);
        return;
    }
}

// Recursively processes directories and files to create a string that can be compiled into the binary.
fn process_dir(dir_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let entries = fs::read_dir(dir_path)?;

    let mut files = Vec::new();
    let mut dirs = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let mut filename = path.file_name().unwrap().to_str().unwrap();
            let content = fs::read_to_string(&path)?;

            if filename.starts_with("_.") {
                let mut chars = filename.chars();
                chars.next();
                filename = chars.as_str();
            }

            files.push(format!(
                "ProjectFile {{ filename: \"{}\".to_string(), content: r#\"{}\"#.to_string() }}",
                filename, content
            ));
        } else if path.is_dir() {
            dirs.push(process_dir(path.to_str().unwrap())?);
        }
    }

    let files_str = if !files.is_empty() {
        format!("Some(vec![{}])", files.join(", "))
    } else {
        "None".to_string()
    };

    let dirs_str = if !dirs.is_empty() {
        format!("Some(vec![{}])", dirs.join(", "))
    } else {
        "None".to_string()
    };

    Ok(format!(
        "ProjectDir {{ dirname: \"{}\".to_string(), files: {}, dirs: {} }}",
        dir_path.split("/").last().unwrap().replace("_.", "."),
        files_str,
        dirs_str
    ))
}
