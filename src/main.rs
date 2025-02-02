use clap::{self, command, Arg};
use config::UnienvConfig;
use confy;
use constvals::WINDOWS_UNITY_EXECUTABLE_PATH;
use std::{
    collections::VecDeque,
    env::current_dir,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    str::FromStr,
};
use unity_parser::get_project_version_string;

mod config;
mod constvals;
mod unity_parser;

fn build_command() -> clap::Command {
    let editor_command = command!().name("editor").arg(
        Arg::new("passargs")
            .allow_hyphen_values(true)
            .num_args(0..)
            .trailing_var_arg(true),
    );

    let hub_command = command!().name("hub").arg(
        Arg::new("passargs")
            .allow_hyphen_values(true)
            .num_args(0..)
            .trailing_var_arg(true),
    );

    command!()
        .subcommand_required(true)
        .subcommand(editor_command)
        .subcommand(hub_command)
}

fn main() -> Result<(), std::io::Error> {
    let command = build_command();

    let matches = command.get_matches();

    let Ok(config): Result<UnienvConfig, confy::ConfyError> = confy::load("unienv", None) else {
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to load unienv config.",
        ));
    };

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

        let executable_path = Path::new(&config.unity_installation_base_path)
            .join(&project_version)
            .join(WINDOWS_UNITY_EXECUTABLE_PATH);

        let mut unity_command = Command::new(executable_path);
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
        unity_hub_command.arg("--");
        unity_hub_command.arg("--headless");
        unity_hub_command.args(passargs);

        unity_hub_command.stdout(Stdio::inherit());
        unity_hub_command.stderr(Stdio::inherit());

        let process = unity_hub_command.spawn().unwrap();

        process.wait_with_output()
    } else {
        // Unexpected due to command requiring subcommand
        panic!("Subcommand unprovided");
    };

    return match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}
