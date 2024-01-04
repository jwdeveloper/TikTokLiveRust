use std::cell::RefCell;
use std::rc::Rc;
use crate::data::live_events::{TikTokGiftEvent, TikTokLiveEvent, TikTokWebsocketMessageEvent, TikTokWebsocketResponseEvent};
use crate::proto::messages::webcast::webcast_response::Message;
use crate::proto::messages::webcast::WebcastResponse;
use crate::core::live_client::TikTokLiveClient;
use crate::core::live_client_events::TikTokLiveEventObserver;

pub struct TikTokLiveMessageMapper
{
    pub(crate) event_observer: Rc<TikTokLiveEventObserver>,
}

impl TikTokLiveMessageMapper
{
    pub fn handle_webcast_response(&self, webcast_response: &WebcastResponse, client: &TikTokLiveClient)
    {
        self.event_observer.publish(&client, TikTokLiveEvent::OnWebsocketResponseEvent(TikTokWebsocketResponseEvent
        {
            websocket_response: webcast_response.clone()
        }));
        for message in &webcast_response.messages
        {
            self.handle_single_message(message, client);
        }
    }


    pub fn handle_single_message(&self, message: &Message, client: &TikTokLiveClient)
    {
        self.event_observer.publish(&client, TikTokLiveEvent::OnWebsocketMessageEvent(TikTokWebsocketMessageEvent
        {
            websocket_message: message.clone()
        }))
    }
}