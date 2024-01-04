use protobuf::Message;
use tokio_tungstenite::tungstenite::handshake::client::{Request};
use tokio_tungstenite::tungstenite::{connect};
use crate::http::http_data::LiveConnectionDataResponse;
use crate::proto::messages::webcast::{WebcastPushFrame, WebcastResponse};
use crate::tiktok::live_client::TikTokLiveClient;
use crate::tiktok::live_client_mapper::TikTokLiveMessageMapper;

pub struct TikTokLiveWebsocketClient
{
    pub(crate) message_mapper: TikTokLiveMessageMapper,
}

impl TikTokLiveWebsocketClient
{
    pub async fn start(&self, response: LiveConnectionDataResponse, client: &TikTokLiveClient)
    {
        let host = response.web_socket_url.host_str().expect("Invalid host in WebSocket URL");

        let request = Request::builder()
            .method("GET")
            .uri(response.web_socket_url.to_string())
            .header("Host", host)
            .header("Upgrade", "websocket")
            .header("Connection", "upgrade")
            .header("Sec-Websocket-Key", "asd")
            .header("Cookie", response.web_socket_cookies)
            .header("Sec-Websocket-Version", "13")
            .body(())
            .unwrap();

        let (mut socket, connectionResponse) = connect(request).expect("Failed to connect");
        loop {
            let msg = socket.read_message().expect("Error reading message");
            let buffer = msg.into_data();

            let mut push_frame = WebcastPushFrame::parse_from_bytes(buffer.as_slice()).expect("Unable to read push frame");
            let mut webcast_response = WebcastResponse::parse_from_bytes(push_frame.Payload.as_mut_slice()).expect("Unable to read webcast response");

            if (webcast_response.needsAck)
            {
                let mut push_frame_ack = WebcastPushFrame::new();
                push_frame_ack.PayloadType = "ack".to_string();
                push_frame_ack.LogId = push_frame.LogId;
                push_frame_ack.Payload = webcast_response.internalExt.clone().into_bytes();

                let binary = push_frame_ack.write_to_bytes().unwrap();
                let message = tungstenite::protocol::Message::binary(binary);
                socket.write_message(message).expect("Unable to send ack packet");
                println!("send ack packet!")
            }

            self.message_mapper.handle_webcast_response(&webcast_response, &client);
        }
    }


    pub fn stop(&self)
    {}
}