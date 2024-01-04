use tiktoklive_client;
use tiktoklive_client::common::live_common::TikTokLiveSettings;
use tiktoklive_client::common::live_events::{TikTokLiveEvent};
use tiktoklive_client::tiktok::live_client::TikTokLiveClient;
use tiktoklive_client::tiktok::TikTokLive;


#[tokio::main]
async fn main() {
    let client = TikTokLive::new_client("dash4214")
        .configure(configure)
        .configure(configure)
        .on_message(onEvent)
        .on_message(onEvent)
        .on_message(onEvent)
        .build();


    client.connect().await;
}


fn configure(settings: &mut TikTokLiveSettings)
{
    settings.language = "dupa".to_string();
}

fn onEvent(client: &TikTokLiveClient, event: &TikTokLiveEvent)
{
    match event
    {
        TikTokLiveEvent::TikTokGift(gift) =>
            {}
        TikTokLiveEvent::TikTokChest(chest) =>
            {}
    }
}