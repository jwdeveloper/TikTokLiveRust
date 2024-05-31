use crate::core::live_client::TikTokLiveClient;
use crate::generated::events::{TikTokLiveEvent, TikTokWebsocketResponseEvent};
use crate::generated::messages::webcast::WebcastResponse;

#[derive(Clone)]
pub struct TikTokLiveMessageMapper {}

impl TikTokLiveMessageMapper {
    pub fn handle_webcast_response(
        &self,
        webcast_response: WebcastResponse,
        client: &TikTokLiveClient,
    ) {
        client.publish_event(TikTokLiveEvent::OnWebsocketResponse(
            TikTokWebsocketResponseEvent {
                raw_data: webcast_response.clone(),
            },
        ));
        for message in &webcast_response.messages {
            self.handle_single_message(message.clone(), client);
        }
    }

    pub fn dupa(&self, _webcast_response: WebcastResponse, _client: &TikTokLiveClient) {
        println!("XD")
    }
}
