use crate::proto::messages::webcast::webcast_response::Message;
use crate::proto::messages::webcast::WebcastResponse;

pub struct TikTokGiftEvent
{
    pub gift_id: i32,
    pub gift_name: String,
}

pub struct TikTokChestEvent
{
    pub gift_id: i32,
    pub gift_name: String,
}

pub struct TikTokWebsocketResponseEvent
{
    pub websocket_response: WebcastResponse,
}

pub struct TikTokWebsocketMessageEvent
{
    pub websocket_message: Message,
}

pub enum TikTokLiveEvent
{
    OnGift(TikTokGiftEvent),
    OnChest(TikTokChestEvent),
    OnWebsocketResponseEvent(TikTokWebsocketResponseEvent),
    OnWebsocketMessageEvent(TikTokWebsocketMessageEvent),
}

