use crate::config::UnienvConfig;
use crate::constants::WINDOWS_UNITY_EXECUTABLE_PATH;
use crate::unity_parser::get_project_version_string;
use clap::ArgMatches;
use std::collections::VecDeque;
use std::env::current_dir;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::str::FromStr;

pub fn launch_unity(
    matches: ArgMatches,
    config: &UnienvConfig,
) -> Result<Result<Output, Error>, Result<(), Error>> {
    let result = if matches.subcommand_name().unwrap() == "editor" {
        let matches = matches.subcommand_matches("editor").unwrap();

        let mut passargs: VecDeque<String> = matches
            .get_many::<String>("passargs")
            .unwrap_or_default()
            .map(|sref| String::from(sref))
            .collect();

        let project_path = match passargs
            .iter()
            .skip_while(|&arg| arg.as_str() != "-projectPath")
            .skip(1)
            .next()
        {
            Some(path_ref) => PathBuf::from_str(path_ref).unwrap(),
            None => {
                println!("No project path provided, assuming PWD.");

                let pwd = current_dir().unwrap();
                passargs.push_front(String::from_str(pwd.to_str().unwrap()).unwrap());
                passargs.push_front(String::from_str("-projectPath").unwrap());

                pwd
            }
        };

        println!("{}", project_path.to_str().unwrap());

        let Ok(project_version) = get_project_version_string(&project_path) else {
            eprintln!("Failed to read project version from directory. Please check if target directory is valid unity project.");
            return Err(Err(Error::new(
                ErrorKind::Other,
                "Failed to read project version from directory.",
            )));
        };

        let Ok(_) = confy::store("unienv", None, &config) else {
            return Err(Err(Error::new(
                ErrorKind::Other,
                "Failed to store unienv config.",
            )));
        };

        let executable_path = Path::new(&config.unity_installation_base_path)
            .join(&project_version)
            .join(WINDOWS_UNITY_EXECUTABLE_PATH);

        let mut unity_command = Command::new(executable_path);
        unity_command.args(config.default_editor_options.clone());
        unity_command.arg("-projectPath").arg(project_path);
        unity_command.args(passargs);

        unity_command.stdout(Stdio::inherit());
        unity_command.stderr(Stdio::inherit());

        let process = unity_command.spawn().unwrap();
        process.wait_with_output()
    } else if matches.subcommand_name().unwrap() == "hub" {
        let matches = matches.subcommand_matches("hub").unwrap();
        let passargs: VecDeque<String> = matches
            .get_many::<String>("passargs")
            .unwrap_or_default()
            .map(|sref| String::from(sref))
            .collect();

        let executable_path = Path::new(&config.unity_hub_path);

        let mut unity_hub_command = Command::new(executable_path);
        unity_hub_command.args(config.default_hub_options.clone());
        unity_hub_command.args(passargs);

        unity_hub_command.stdout(Stdio::inherit());
        unity_hub_command.stderr(Stdio::inherit());

        let process = unity_hub_command.spawn().unwrap();

        process.wait_with_output()
    } else {
        // Unexpected due to command requiring subcommand
        panic!("Subcommand unprovided");
    };
    Ok(result)
}
