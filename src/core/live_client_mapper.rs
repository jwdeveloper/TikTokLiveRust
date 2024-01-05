use std::collections::HashMap;
use std::rc::Rc;
use protobuf::reflect::FileDescriptor;


use crate::core::live_client::TikTokLiveClient;
use crate::core::live_client_events::TikTokLiveEventObserver;
use crate::data::live_events::{TikTokGiftEvent, TikTokLiveEvent, TikTokWebsocketMessageEvent, TikTokWebsocketResponseEvent, TikTokWebsocketUnknownMessageEvent};
use crate::proto::messages::webcast::{file_descriptor, WebcastGiftMessage, WebcastMemberMessage, WebcastResponse};
use crate::proto::messages::webcast::webcast_response::Message;


pub struct TikTokLiveMessageMapper
{}


impl TikTokLiveMessageMapper
{
    pub fn new() -> Self
    {
        TikTokLiveMessageMapper
        {}
    }

    pub fn handle_webcast_response(&self, webcast_response: &WebcastResponse, client: &TikTokLiveClient)
    {
        client.publish_event(TikTokLiveEvent::OnWebsocketResponseEvent(TikTokWebsocketResponseEvent
        {
            websocket_response: webcast_response.clone()
        }));
        for message in &webcast_response.messages
        {
            self.handle_single_message(message.clone(), client);
        }
    }


    pub fn handle_single_message(&self, message: Message, client: &TikTokLiveClient)
    {
        let proto_message_name = &message.method;
        let option = file_descriptor().message_by_package_relative_name(proto_message_name);
        if (option.is_none())
        {
            client.publish_event(TikTokLiveEvent::onWebsocketUnknownMessageEvent(TikTokWebsocketUnknownMessageEvent
            {
                websocket_message: message,
            }));
            return;
        }
        println!("Incoming massage! {}", proto_message_name);
        let proto_message_content = &message.payload;
        let proto_message = option.unwrap();
        let webcast_object = proto_message.parse_from_bytes(proto_message_content).unwrap();


       // let event  = webcast_object as WebcastGiftMessage;





        //  client.publish_event(event_enum);
    }
}

