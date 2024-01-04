use std::cell::RefCell;
use env_logger::{Builder, Env};
use log::LevelFilter;
use crate::common::create_default_settings;
use crate::common::live_common::{TikTokLiveInfo, TikTokLiveSettings};
use crate::http::http_request_builder::HttpRequestFactory;
use crate::tiktok::live_client::TikTokLiveClient;
use crate::tiktok::live_client_events::{TikTokEventHandler, TikTokLiveEventObserver};
use crate::tiktok::live_client_http::TikTokLiveHttpClient;
use crate::tiktok::live_client_mapper::TikTokLiveMessageMapper;
use crate::tiktok::live_client_websocket::TikTokLiveWebsocketClient;


pub struct TikTokLiveBuilder
{
    settings: TikTokLiveSettings,
    event_observer: TikTokLiveEventObserver,
}


impl TikTokLiveBuilder
{
    pub fn new(host_name: &str) -> Self
    {
        let env = Env::default()
            .filter_or("MY_LOG_LEVEL", "info");
        Builder::from_env(env)
            .filter_module("my_crate::module", LevelFilter::Debug)
            .init();
        Self
        {
            settings: create_default_settings(host_name),
            event_observer: TikTokLiveEventObserver::new(),
        }
    }

    pub fn configure<F>(&mut self, on_configure: F) -> &mut Self
        where F: FnOnce(&mut TikTokLiveSettings),
    {
        on_configure(&mut self.settings);
        self
    }

    pub fn on_message(&mut self, on_event: TikTokEventHandler) -> &mut Self
    {
        self.event_observer.attach(on_event);
        self
    }

    pub fn build(&self) -> TikTokLiveClient {
        let settings = &self.settings;
        let observer: &TikTokLiveEventObserver = &self.event_observer;
        let mapper = TikTokLiveMessageMapper
        {
            event_observer: observer
        };
        let websocket_client = TikTokLiveWebsocketClient
        {
            message_mapper: mapper
        };
        let http_factory = HttpRequestFactory {
            settings: settings.clone()
        };
        let http_client = TikTokLiveHttpClient
        {
            settings: settings.clone(),
            factory: http_factory,
        };

        return TikTokLiveClient
        {
            settings: settings.clone(),
            http_client,
            event_observer,
            websocket_client,
            room_info: TikTokLiveInfo::default(),
        };
    }

    pub fn build_and_connect(&self) -> TikTokLiveClient
    {
        let client = self.build();
        client.connect();
        return client;
    }
}
