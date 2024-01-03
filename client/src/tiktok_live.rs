
use crate::live_client_builder::{TikTokLiveBuilder};


pub struct TikTokLive {}
impl TikTokLive
{
    pub fn new_client() -> TikTokLiveBuilder {
        TikTokLiveBuilder::new()
    }
}