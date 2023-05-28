use std::collections::HashMap;

use chrono::{Utc, DateTime};
use ferinth::structures::project::Project;
use furse::structures::mod_structs::Mod;
use serde::{Serialize, Deserialize};
use serde_json::{Map, Value};

#[derive(Clone, Serialize)]
pub struct StatusUpdate {
  pub status: String,
}

#[derive(Clone, Serialize)]
pub struct ProgressUpdate {
  pub progress: i32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LauncherProfiles {
    pub profiles: HashMap<String, Profile>,
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub name: String,
    #[serde(rename = "type")]
    pub profile_type: String,
    #[serde(default)]
    pub created: DateTime<Utc>,
    pub last_version_id: String,
    pub icon: String,
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CombinedProjects {
  ModrinthProject(Project),
  CurseForgeMod(Mod),
}