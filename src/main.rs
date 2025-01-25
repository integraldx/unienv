use std::{env::current_dir, fs::read_to_string, io::{Error, ErrorKind}, path::{Path, PathBuf}, process::Command};
use config::{ProjectVersion, UnienvConfig};
use confy;
use constvals::{PROJECT_VERSION_PATH, WINDOWS_UNITY_EXECUTABLE_PATH};

mod constvals;
mod config;

fn main() -> Result<(), std::io::Error> {
    let Ok(config): Result<UnienvConfig, confy::ConfyError> = confy::load("unienv", None) else {
        return Err(Error::new(ErrorKind::Other, "Failed to load unienv config."));
    };


    let Ok(project_path) = current_dir() else {
        return Err(Error::new(ErrorKind::Other, "Failed to read current directory."));
    };

    let Ok(project_version) = get_project_version_string(&project_path) else {
        eprintln!("Failed to read project version from directory. Please check if target directory is valid unity project.");
        return Err(Error::new(ErrorKind::Other, "Failed to read project version from directory."))
    };

    let Ok(_) = confy::store("unienv", None, &config) else {
        return Err(Error::new(ErrorKind::Other, "Failed to store unienv config."));
    };

    let executable_path = Path::new(&config.unity_hub_path).join(&project_version).join(WINDOWS_UNITY_EXECUTABLE_PATH);

    let mut command = Command::new(executable_path);

    let output = command.output();

    return match output {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}

fn get_project_version_string(project_path: &PathBuf) -> Result<String, std::io::Error> {
    let version_file_str = read_to_string(project_path.join(PROJECT_VERSION_PATH))?;
    let Ok(version): Result<ProjectVersion, serde_yml::Error> = serde_yml::from_str(&version_file_str) else {
        eprintln!("Parse Error!");
        return Err(Error::new(ErrorKind::Other, "Failed to parse ProjectVersion.txt"))
    };

    Ok(version.editor_version)
}