use crate::proto::messages::webcast::webcast_response::Message;
use crate::proto::messages::webcast::{WebcastChatMessage, WebcastGiftMessage, WebcastMemberMessage, WebcastResponse};


pub struct TikTokJoinEvent
{
    pub raw_data: WebcastMemberMessage,
}

pub struct TikTokChatEvent
{
    pub raw_data: WebcastChatMessage,
}

pub struct TikTokGiftEvent
{
    pub raw_data: WebcastGiftMessage

}


pub struct TikTokWebsocketResponseEvent
{
    pub websocket_response: WebcastResponse,
}

pub struct TikTokWebsocketMessageEvent
{
    pub websocket_message: Message,
}

pub struct TikTokWebsocketUnknownMessageEvent
{
    pub websocket_message: Message,
}

pub enum TikTokLiveEvent
{
    onJoin(TikTokJoinEvent),
    OnGift(TikTokGiftEvent),
    onChat(WebcastChatMessage),
    OnWebsocketResponseEvent(TikTokWebsocketResponseEvent),
    OnWebsocketMessageEvent(TikTokWebsocketMessageEvent),
    onWebsocketUnknownMessageEvent(TikTokWebsocketUnknownMessageEvent)
}

