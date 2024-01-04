use log::info;
use protobuf::Message;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Url;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use crate::http::http_data::LiveConnectionDataResponse;
use crate::tiktok::live_client::TikTokLiveClient;

pub struct TikTokLiveWebsocketClient
{}

impl TikTokLiveWebsocketClient
{
    pub async fn start(&self, response: LiveConnectionDataResponse, client: &TikTokLiveClient)
    {
        let url = Url::parse(response.web_socket_url.as_str()).expect("Invalid URL");

        let mut headers = HeaderMap::new();
        for header in response.web_socket_headers
        {
            headers.insert("Cookie", header);
        }
        let (mut socket, _) = timeout(response.web_socket_timeout, connect_async(url))
            .await
            .unwrap().unwrap();


        println!("WebSocket connected");

/*
        socket.send(Message::Text("Hello WebSocket".into())).await?;

        // Listen for messages from the server
        while let Some(msg) = socket.next().await {
            let msg = msg?;

            match msg {
                Message::Text(text) => println!("Received text: {}", text),
                Message::Binary(bin) => println!("Received binary data: {:?}", bin),
                _ => (),
            }
        }*/


    }


    pub fn stop(&self)
    {}
}