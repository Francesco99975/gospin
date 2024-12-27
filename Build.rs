use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let dir_structure = process_dir("go_boilerplate");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_project.rs");
    let mut f = File::create(dest_path).unwrap();

    write!(f, "static PROJECT_DIR: ProjectDir = {};", dir_structure).unwrap();
}

// Recursively processes directories and files to create a string that can be compiled into the binary.
fn process_dir(dir_path: &str) -> String {
    let entries = fs::read_dir(dir_path).unwrap();
    let mut files = Vec::new();
    let mut dirs = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let content = fs::read_to_string(&path).unwrap();
            files.push(format!(
                "ProjectFile {{ filename: \"{}\".to_string(), content: \"{}\".to_string() }}",
                path.file_name().unwrap().to_str().unwrap(),
                content
            ));
        } else if path.is_dir() {
            dirs.push(process_dir(path.to_str().unwrap()));
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

    format!(
        "ProjectDir {{ dirname: \"{}\".to_string(), files: {}, dirs: {} }}",
        dir_path, files_str, dirs_str
    )
}
