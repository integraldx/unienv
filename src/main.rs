use std::process::Command;

mod constvals;

fn main() -> Result<(), std::io::Error>{
    println!("Hello, world!");

    let mut command = Command::new(constvals::WINDOWS_UNITY_DEFAULT_BASE_PATH);

    let output = command.output();

    return match output {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}
