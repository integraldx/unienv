use serde::{Deserialize, Serialize};
use std::{default, str::FromStr};

use crate::constvals::{WINDOWS_UNITY_DEFAULT_BASE_PATH, WINDOWS_UNITY_HUB_DEFAULT_PATH};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UnienvConfig {
    pub unity_hub_path: String,
    pub unity_installation_base_path: String,
    pub default_editor_options: Vec<String>,
    pub default_hub_options: Vec<String>,
}

impl default::Default for UnienvConfig {
    fn default() -> Self {
        Self {
            unity_hub_path: WINDOWS_UNITY_HUB_DEFAULT_PATH.to_string(),
            unity_installation_base_path: WINDOWS_UNITY_DEFAULT_BASE_PATH.to_string(),
            default_editor_options: ["-batchmode", "-quit"]
                .map(|s| String::from_str(s).unwrap())
                .to_vec(),
            default_hub_options: ["--", "--headless"]
                .map(|s| String::from_str(s).unwrap())
                .to_vec(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProjectVersion {
    #[serde(rename = "m_EditorVersion")]
    pub editor_version: String,

    #[serde(rename = "m_EditorVersionWithRevision")]
    pub editor_version_with_revision: String,
}
