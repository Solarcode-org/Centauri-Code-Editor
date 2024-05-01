use std::path::PathBuf;

use directories::ProjectDirs;

#[derive(Debug, Clone, Copy)]
pub struct Config<S: AsRef<str>, P: Into<PathBuf> + std::convert::AsRef<std::ffi::OsStr>> {
    pub project_name: S,
    pub data_folder: Option<P>,
    pub log_env: S,
    pub log_file: S,
}

pub fn get_data_dir<S: AsRef<str>, P: Into<PathBuf> + std::convert::AsRef<std::ffi::OsStr>>(
    config: &Config<S, P>,
) -> PathBuf {
    let directory = if let Some(s) = &config.data_folder {
        s.into()
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".data")
    };
    directory
}

pub fn init_config<'a>() -> Config<String, PathBuf> {
    let project_name = env!("CARGO_CRATE_NAME").to_uppercase().to_string();

    Config {
        project_name: project_name.clone(),
        data_folder: std::env::var(format!("{}_DATA", project_name))
            .ok()
            .map(PathBuf::from),
        log_env: format!("{}_LOGLEVEL", project_name),
        log_file: format!("{}.log", env!("CARGO_PKG_NAME")),
    }
}

fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("org", "solarcode", env!("CARGO_PKG_NAME"))
}
