use std::{error::Error, fmt, fs::File, io::Write, process::Command};

use crate::models::{generate_project_structure, ProjectDir};

pub fn scaffold(project: &str) -> Result<(), ScaffError> {
    let root = generate_project_structure(project, 8080);

    dir_builder(root, format!("./"))?;

    Ok(())
}

fn dir_builder(dir: ProjectDir, depth: String) -> Result<(), ScaffError> {
    Command::new("mkdir")
        .arg(dir.dirname)
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

    for prj_dir in dir.dirs.unwrap_or(vec![]) {
        let new_depth = prj_dir.dirname.clone();
        dir_builder(prj_dir, depth.clone() + &new_depth)?;
    }

    Ok(())
}

#[derive(Debug)]
pub struct ScaffError {
    pub message: String,
}
impl Error for ScaffError {}

impl fmt::Display for ScaffError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Could not get device id - Error: {}", self.message)
    }
}
