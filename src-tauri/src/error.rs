#[derive(Debug, thiserror::Error)]
pub enum TinkarosError {
    #[error("error while updating: {0}")]
    Update(String),

    #[error("unable to fetch mod versions")]
    FetchModVersions,

    #[error("could not request/parse external config: {0}")]
    DataInvalid(String),

    #[error("config file contents could not be parsed")]
    ConfigInvalid,

    #[error("config file not found")]
    ConfigNotFound,

    #[error("unable to parse mc launcher config")]
    InvalidLauncherConfig,

    #[error("Failed to fetch file: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("unable to extract zip file")]
    ZipResult(#[from] zip::result::ZipError),

    #[error("unable to emit event to frontend")]
    EmitEvent,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Unknown(#[from] Box<dyn std::error::Error>)
}

impl serde::Serialize for TinkarosError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

