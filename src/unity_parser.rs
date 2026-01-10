use std::{
    fs::read_to_string,
    io::Error,
    path::PathBuf,
};

use crate::{config::ProjectVersion, constants::PROJECT_VERSION_PATH};

pub(crate) fn get_project_version_string(project_path: &PathBuf) -> Result<String, Error> {
    let version_file_str = read_to_string(project_path.join(PROJECT_VERSION_PATH))?;
    let Ok(version): Result<ProjectVersion, serde_yml::Error> =
        serde_yml::from_str(&version_file_str)
    else {
        eprintln!("Parse Error!");
        return Err(Error::other(
            "Failed to parse ProjectVersion.txt",
        ));
    };

    Ok(version.editor_version)
}
