use std::default;
use serde::{Serialize, Deserialize};

use crate::constvals::WINDOWS_UNITY_DEFAULT_BASE_PATH;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UnienvConfig {
    pub unity_hub_path: String,
}

impl default::Default for UnienvConfig {
    fn default() -> Self {
        Self { unity_hub_path: WINDOWS_UNITY_DEFAULT_BASE_PATH.to_string() }
    }
}