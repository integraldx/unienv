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

        let mut args = config.default_editor_options.clone();

        let mut passargs: Vec<String> = matches
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
                args.push(String::from_str("-projectPath").unwrap());
                args.push(String::from_str(pwd.to_str().unwrap()).unwrap());

                pwd
            }
        };

        args.append(&mut passargs);

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

        launch_unity_editor(&executable_path, args.into_iter().collect())
    } else if matches.subcommand_name().unwrap() == "hub" {
        let matches = matches.subcommand_matches("hub").unwrap();
        let mut passargs: Vec<String> = matches
            .get_many::<String>("passargs")
            .unwrap_or_default()
            .map(|sref| String::from(sref))
            .collect();

        let executable_path = Path::new(&config.unity_hub_path);

        let mut args = config.default_hub_options.clone();
        args.append(&mut passargs);

        launch_unity_hub(&executable_path.to_path_buf(), args.into_iter().collect())
    } else if matches.subcommand_name().unwrap() == "build" {
        let matches = matches.subcommand_matches("build").unwrap();

        let build_profile = matches.get_one::<String>("buildProfile").unwrap();
        let output_path = matches.get_one::<String>("output").unwrap();
        let log_path = matches.get_one::<String>("logPath").unwrap();
        let current_dir_str = String::from_str(current_dir().unwrap().to_str().unwrap()).unwrap();
        let project_path = matches
            .get_one::<String>("projectPath")
            .unwrap_or(&current_dir_str);

        let Ok(project_version) =
            get_project_version_string(&PathBuf::from_str(project_path.as_str()).unwrap())
        else {
            eprintln!("Failed to read project version from directory. Please check if target directory is valid unity project.");
            return Err(Err(Error::new(
                ErrorKind::Other,
                "Failed to read project version from directory.",
            )));
        };

        let executable_path = Path::new(&config.unity_installation_base_path)
            .join(&project_version)
            .join(WINDOWS_UNITY_EXECUTABLE_PATH);

        let mut args = config.default_editor_options.clone();

        args.push(String::from_str("-projectPath").unwrap());
        args.push(project_path.clone());

        args.push(String::from("-logFile"));
        args.push(log_path.clone());

        args.push(String::from("-activeBuildProfile"));
        args.push(build_profile.clone());

        args.push(String::from("-build"));
        args.push(output_path.clone());

        launch_unity_editor(&executable_path.to_path_buf(), args.into_iter().collect())
    } else {
        // Unexpected due to command requiring subcommand
        panic!("Subcommand unprovided");
    };
    Ok(result)
}

fn launch_unity_editor(
    unity_editor_path: &PathBuf,
    args: VecDeque<String>,
) -> Result<Output, Error> {
    let mut unity_command = Command::new(unity_editor_path);
    unity_command.args(args);
    unity_command.stdout(Stdio::inherit());
    unity_command.stderr(Stdio::inherit());

    let Ok(process) = unity_command.spawn() else {
        println!("Failed to launch unity editor.");
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to launch unity editor.",
        ));
    };

    process.wait_with_output()
}

fn launch_unity_hub(unity_hub_path: &PathBuf, args: VecDeque<String>) -> Result<Output, Error> {
    let mut unity_hub_command = Command::new(unity_hub_path);
    unity_hub_command.args(args);
    unity_hub_command.stdout(Stdio::inherit());
    unity_hub_command.stderr(Stdio::inherit());

    let Ok(process) = unity_hub_command.spawn() else {
        println!("Failed to launch unity hub.");
        return Err(Error::new(ErrorKind::Other, "Failed to launch unity hub."));
    };

    process.wait_with_output()
}
