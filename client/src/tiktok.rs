use crate::tiktok::live_client_builder::TikTokLiveBuilder;

pub mod live_client_builder;
pub mod live_client;
pub mod live_client_events;
pub mod live_client_http;
mod live_client_websocket;

pub struct TikTokLive {}

impl TikTokLive
{
    pub fn new_client(user: &str) -> TikTokLiveBuilder {
        TikTokLiveBuilder::new(user)
    }
}