use serde::{Deserialize, Serialize};
use std::default;

use crate::constvals::WINDOWS_UNITY_DEFAULT_BASE_PATH;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UnienvConfig {
    pub unity_hub_path: String,
}

impl default::Default for UnienvConfig {
    fn default() -> Self {
        Self {
            unity_hub_path: WINDOWS_UNITY_DEFAULT_BASE_PATH.to_string(),
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
