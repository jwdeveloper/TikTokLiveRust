use log::info;
use tiktoklive_client;
use tiktoklive_client::live_client::TikTokLiveClient;
use tiktoklive_client::live_client_builder::TikTokLiveSettings;
use tiktoklive_client::live_client_events::TikTokLiveEvent;

use tiktoklive_client::tiktok_live::TikTokLive;

fn main()
{
    let client = TikTokLive::new_client()
        .configure(configure)
        .configure(configure)
        .on_message(onGift)
        .on_message(onGift)
        .on_message(onGift)
        .build();

    client.connect();
}

fn configure(settings: &mut TikTokLiveSettings)
{

    settings.language = "dupa".to_string();
    println!("config! {}",settings.language);
}

fn onGift(client: &TikTokLiveClient, event: &TikTokLiveEvent)
{
    println!("siema!")
}