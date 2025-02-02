use std::{
    fs::read_to_string,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use crate::{config::ProjectVersion, constvals::PROJECT_VERSION_PATH};

pub(crate) fn get_project_version_string(project_path: &PathBuf) -> Result<String, std::io::Error> {
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
