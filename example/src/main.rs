use std::time::Duration;
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
    settings.http_data.time_out= Duration::from_secs(12)

}

fn onEvent(client: &TikTokLiveClient, event: &TikTokLiveEvent)
{
    match event
    {
        TikTokLiveEvent::onGift(gift) =>
            {
                let i = 0;
                let x = gift.gift_id;
            }
        TikTokLiveEvent::onChest(chest) =>
            {}
    };
}