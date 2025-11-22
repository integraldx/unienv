use clap::{self, command, Arg};
use config::UnienvConfig;
use confy;
use std::io::{Error, ErrorKind};
mod config;
mod constants;
mod unity_launcher;
mod unity_parser;

fn build_command() -> clap::Command {
    // let test_command = command!().name("test").arg(
    //     Arg::new("passargs")
    //         .allow_hyphen_values(true)
    //         .num_args(0..)
    //         .trailing_var_arg(true),
    // );

    let build_command = command!().name("build")
        .arg(Arg::new("logPath")
            .required(false)
            .short('l')
            .long("logPath")
            .help("log file path")
            .default_value("./unity.log")
        )
        .arg(Arg::new("projectPath")
            .required(false)
            .short('p')
            .long("projectPath")
            .help("project path")
        )
        .arg(Arg::new("buildProfile")
            .required(true)
            .short('b')
            .long("buildProfile")
            .help("Build profile to use")
        )
        .arg(Arg::new("output")
            .required(true)
            .short('o')
            .long("output")
            .help("Target output directory or executable.\nRefer to unity documentation for more information.")
        );

    let hub_command = command!().name("hub").arg(
        Arg::new("passargs")
            .allow_hyphen_values(true)
            .num_args(0..)
            .trailing_var_arg(true),
    );

    let editor_command = command!().name("editor").arg(
        Arg::new("passargs")
            .allow_hyphen_values(true)
            .num_args(0..)
            .trailing_var_arg(true),
    );

    command!()
        .subcommand_required(true)
        // .subcommand(test_command)
        .subcommand(build_command)
        .subcommand(editor_command)
        .subcommand(hub_command)
}

fn main() -> Result<(), Error> {
    let command = build_command();

    let matches = command.get_matches();

    let Ok(config): Result<UnienvConfig, confy::ConfyError> = confy::load("unienv", None) else {
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to load unienv config.",
        ));
    };

    let result = match unity_launcher::launch_unity(matches, &config) {
        Ok(value) => value,
        Err(value) => return value,
    };

    match result {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(Error::new(
                    ErrorKind::Other,
                    "Process returned non-zero value.",
                ))
            }
        }
        Err(e) => Err(e),
    }
}
