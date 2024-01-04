use std::time::Duration;

use tiktoklive::data::live_common::TikTokLiveSettings;
use tiktoklive::data::live_events::{TikTokLiveEvent};
use tiktoklive::core::live_client::TikTokLiveClient;
use tiktoklive::TikTokLive;

#[tokio::main]
async fn main() {
    let user_name = "dash4214";
    let client = TikTokLive::new_client(user_name)
        .configure(configure)
        .on_event(on_event)
        .build();

  ;
    client.connect().await;
}


fn configure(settings: &mut TikTokLiveSettings)
{
    settings.http_data.time_out= Duration::from_secs(12)

}

fn on_event(client: &TikTokLiveClient, event: &TikTokLiveEvent)
{
    match event
    {
        TikTokLiveEvent::OnWebsocketMessageEvent(event)=>
            {
                println!("Hello! {}",event.websocket_message.method)
            }
        _ => {}
    }
}