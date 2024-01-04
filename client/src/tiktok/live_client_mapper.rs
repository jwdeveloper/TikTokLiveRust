use std::cell::RefCell;
use crate::common::live_events::{TikTokGiftEvent, TikTokLiveEvent};
use crate::proto::messages::webcast::WebcastResponse;
use crate::tiktok::live_client::TikTokLiveClient;
use crate::tiktok::live_client_events::TikTokLiveEventObserver;

pub struct TikTokLiveMessageMapper
{
    pub(crate) event_observer:  TikTokLiveEventObserver,
}

impl TikTokLiveMessageMapper
{
    pub fn handle_webcast_response(&self, webcastResponse: &WebcastResponse, client: &TikTokLiveClient)
    {
        let event = TikTokLiveEvent::onGift(TikTokGiftEvent
        {
            gift_id: 0,
            gift_name: "dupa".to_string(),
        });
        self.event_observer.publish(client, &event);
    }
}