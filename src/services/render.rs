use crate::types::cache::ExpiringCache;
use crate::types::error::MinecraftError;
use crate::types::ui::player_render::{create_url, url_to_buffer, PlayerOptions, PlayerType};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

pub struct PlayerRenderService {
    cache: ExpiringCache<String, Vec<u8>>,
}

impl PlayerRenderService {
    /// Creates a new PlayerDBService with a cache TTL of 60 minutes.
    fn new() -> Self {
        Self {
            cache: ExpiringCache::new(Duration::from_secs(3600)), // 5 minutes TTL
        }
    }

    /// Returns a singleton instance of PlayerDBService.
    pub fn instance() -> &'static Arc<Self> {
        static INSTANCE: OnceLock<Arc<PlayerRenderService>> = OnceLock::new();
        
        INSTANCE.get_or_init(|| {
            Arc::new(PlayerRenderService::new())
        })
    }

    /// Retrieves player render for the given Minecraft uuid.
    /// It first checks the cache, and if not found or expired, fetches fresh data from the API.
    pub async fn get_player_render(&self, username: &str) -> Result<Vec<u8>, MinecraftError> {
        if let Some(response) = self.cache.get(&username.to_string()).await {
            return Ok(response.clone());
        }

        let response = self.get_player_render_api(username).await;
        self.cache.insert(username.to_string(), response.clone()).await;

        Ok(response)
    }

    async fn get_player_render_api(&self, id: &str) -> Vec<u8> {
        let url = create_url(id, PlayerOptions {
            player_type: PlayerType::Full,
            size: Some(384.0),
            ..Default::default()
        });
        
        url_to_buffer(url.as_str()).await
    }
}