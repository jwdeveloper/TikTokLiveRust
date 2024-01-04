use log::info;
use protobuf::Message;
use protobuf::well_known_types::struct_::Value;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Url;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::handshake::client::{Request};
use tokio_tungstenite::tungstenite::http;
use crate::http::http_data::LiveConnectionDataResponse;
use crate::tiktok::live_client::TikTokLiveClient;

pub struct TikTokLiveWebsocketClient
{}

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

        let (mut socket, _) = connect_async(request).await.expect("Failed to connect");
        socket.send(Message::Text("Hello, world!".into())).await?;
        while let Some(msg) = socket.next().await {
            let msg = msg?;

            match msg {
                Message::Text(text) => println!("Received text: {}", text),
                Message::Binary(bin) => println!("Received binary data: {:?}", bin),
                _ => (),
            }
        }
    }

    pub fn stop(&self)
    {}
}