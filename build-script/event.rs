struct TikTokGiftEvent {
    raw_data: WebcastGiftMessage,
}
struct TikTokChatEvent {
    raw_data: WebcastChatMessage,
}
struct TikTokLikeEvent {
    raw_data: WebcastLikeMessage,
}
pub enum TikTokLiveEvent
{
    OnGift(TikTokGiftEvent),
    OnChat(TikTokChatEvent),
    OnLike(TikTokLikeEvent),
}
pub fn handle_single_message(
    message: crate::proto::messages::webcast::webcast_response::Message,
    client: &TikTokLiveClient,
) {
    let proto_message_name = &message.method;
    let proto_message_content = &message.payload;
    match proto_message_name.as_str() {
        "WebcastGiftMessage" => {
            let raw_data = WebcastGiftMessage::parse_from_bytes(proto_message_content)
                .unwrap();
            let event = TikTokGiftEvent { raw_data };
            client.publish_event(TikTokLiveEvent::OnGift(event));
        }
        "WebcastChatMessage" => {
            let raw_data = WebcastChatMessage::parse_from_bytes(proto_message_content)
                .unwrap();
            let event = TikTokChatEvent { raw_data };
            client.publish_event(TikTokLiveEvent::OnChat(event));
        }
        "WebcastLikeMessage" => {
            let raw_data = WebcastLikeMessage::parse_from_bytes(proto_message_content)
                .unwrap();
            let event = TikTokLikeEvent { raw_data };
            client.publish_event(TikTokLiveEvent::OnLike(event));
        }
        _ => {
            client
                .publish_event(
                    TikTokLiveEvent::OnWebsocketUnknownMessageEvent(TikTokWebsocketUnknownMessageEvent {
                        message_name: message.method.clone(),
                        raw_data: message,
                    }),
                );
        }
    }
}
