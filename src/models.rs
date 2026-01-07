use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ProjectFile {
    pub filename: String,
    pub content: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProjectDir {
    pub dirname: String,
    pub files: Option<Vec<ProjectFile>>,
    pub dirs: Option<Vec<ProjectDir>>,
}

pub struct Injectables {
    pub project_name: String,
    pub username: String,
    pub port: u32,
    pub db: bool,
    pub ws: bool,
    pub doppler: bool,
}
