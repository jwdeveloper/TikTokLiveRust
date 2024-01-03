use std::cell::RefCell;
use crate::live_client::{TikTokLiveClient};

pub struct TikTokEvent
{
    pub created_at: i32,
    pub name: String,
}

pub struct TikTokLiveSettings
{}


pub struct TikTokLiveBuilder
{
    settings: RefCell<TikTokLiveSettings>,
}


impl TikTokLiveBuilder
{
    pub fn new() -> Self
    {
        TikTokLiveBuilder
        {
            settings: RefCell::new(TikTokLiveSettings{})
        }
    }

    pub fn configure(&mut self, on_configure: fn(& mut TikTokLiveSettings)) -> &Self
    {
        on_configure(self.settings.get_mut());
        self
    }

    pub fn on_message(&self, on_event: fn(TikTokEvent)) -> &Self
    {


        self
    }

    pub fn build(&self) -> TikTokLiveClient {
        TikTokLiveClient {}
    }

    pub fn build_and_connect(&self) -> TikTokLiveClient {
        TikTokLiveClient {}
    }
}
