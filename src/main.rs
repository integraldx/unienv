use clap::{self, command, Arg};
use config::{ProjectVersion, UnienvConfig};
use confy;
use constvals::{PROJECT_VERSION_PATH, WINDOWS_UNITY_EXECUTABLE_PATH};
use std::{
    fs::read_to_string,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

mod config;
mod constvals;

fn main() -> Result<(), std::io::Error> {
    let command = command!()
        .arg(
            Arg::new("projectPath")
                .long("projectPath")
                .value_name("[PATH]")
                .required(true),
        )
        .arg(
            Arg::new("passargs")
                .allow_hyphen_values(true)
                .num_args(0..)
                .trailing_var_arg(true),
        );

    let matches = command.get_matches();

    let project_path_str: &String = matches.get_one("projectPath").unwrap();
    let passargs: Vec<_> = matches
        .get_many::<String>("passargs")
        .unwrap_or_default()
        .collect();

    let Ok(config): Result<UnienvConfig, confy::ConfyError> = confy::load("unienv", None) else {
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to load unienv config.",
        ));
    };

    let Ok(project_path) = PathBuf::from_str(project_path_str);

    let Ok(project_version) = get_project_version_string(&project_path) else {
        eprintln!("Failed to read project version from directory. Please check if target directory is valid unity project.");
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to read project version from directory.",
        ));
    };

    let Ok(_) = confy::store("unienv", None, &config) else {
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to store unienv config.",
        ));
    };

    let executable_path = Path::new(&config.unity_hub_path)
        .join(&project_version)
        .join(WINDOWS_UNITY_EXECUTABLE_PATH);

    let mut unity_command = Command::new(executable_path);
    unity_command.arg("-projectPath").arg(project_path_str);
    unity_command.args(passargs);

    let output = unity_command.output();

    return match output {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}

fn get_project_version_string(project_path: &PathBuf) -> Result<String, std::io::Error> {
    let version_file_str = read_to_string(project_path.join(PROJECT_VERSION_PATH))?;
    let Ok(version): Result<ProjectVersion, serde_yml::Error> =
        serde_yml::from_str(&version_file_str)
    else {
        eprintln!("Parse Error!");
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to parse ProjectVersion.txt",
        ));
    };

    Ok(version.editor_version)
}
