use crate::common::create_default_settings;
use crate::common::live_common::TikTokLiveSettings;

use crate::live_client::{TikTokLiveClient};
use crate::live_client_events::{TikTokEventHandler, TikTokLiveEventObserver};




pub struct TikTokLiveBuilder
{
    settings: TikTokLiveSettings,
    event_observer: TikTokLiveEventObserver,
}


impl TikTokLiveBuilder
{
    pub fn new() -> Self
    {
        Self
        {
            settings: create_default_settings(),
            event_observer: TikTokLiveEventObserver::new(),
        }
    }

    pub fn configure<F>(&mut self, on_configure: F) -> &mut Self
        where F: FnOnce(&mut TikTokLiveSettings),
    {
        on_configure(&mut self.settings);
        self
    }

    pub fn on_message(&mut self, fun: TikTokEventHandler) -> &mut Self
    {
        self.event_observer.attach(fun);
        self
    }

    pub fn build(&self) -> TikTokLiveClient {

        let observer: TikTokLiveEventObserver = self.event_observer.clone();
        return TikTokLiveClient::new(observer);
    }

    pub fn build_and_connect(&self) -> TikTokLiveClient {
        let client = self.build();
        client.connect();
        return client;
    }
}
