use std::path::PathBuf;
use tokio::fs;

pub struct HorizonAssetManager {
    pub game_dir: PathBuf,
}

impl HorizonAssetManager {
    pub fn new(game_dir: PathBuf) -> Self {
        Self { game_dir }
    }

    pub async fn prepare_assets(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.game_dir.join("assets")).await?;
        fs::create_dir_all(&self.game_dir.join("libraries")).await?;
        Ok(())
    }
}
