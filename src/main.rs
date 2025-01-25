use std::{io::{Error, ErrorKind}, path::Path, process::Command};
use config::UnienvConfig;
use confy;
use constvals::WINDOWS_UNITY_EXECUTABLE_PATH;

mod constvals;
mod config;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");

    let Ok(config): Result<UnienvConfig, confy::ConfyError> = confy::load("unienv", None) else {
        return Err(Error::new(ErrorKind::Other, "Failed to load unienv config."));
    };

    let version = "2022.3.47f1";

    let executable_path = Path::new(&config.unity_hub_path).join(version).join(WINDOWS_UNITY_EXECUTABLE_PATH);


    let Ok(_) = confy::store("unienv", None, config) else {
        return Err(Error::new(ErrorKind::Other, "Failed to store unienv config."));
    };

    let mut command = Command::new(executable_path);

    let output = command.output();

    return match output {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}
