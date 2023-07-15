use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

use crate::{resolve::structs::Modpack, error::TinkarosError};

static STATE: OnceCell<Arc<State>> = OnceCell::const_new();

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    pub modpack: Modpack,
}

impl State {
    pub async fn get() -> Result<Arc<Self>, TinkarosError> {
        STATE.get_or_try_init(|| async {
            let default_url = "https://gist.githubusercontent.com/Hbarniq/ec9d01d863083becd062b378ca01a3d8/raw/ahms.json";

            let modpack = reqwest::get(default_url).await?.json::<State>().await?.modpack;

            Ok(Arc::new(Self {
                modpack
            }))
            
        }).await.map(Arc::clone)
    }
}