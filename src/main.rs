use std::io;
use std::time::Duration;

use tiktoklive::core::live_client::TikTokLiveClient;
use tiktoklive::data::live_common::TikTokLiveSettings;
use tiktoklive::generated::events::TikTokLiveEvent;

use tiktoklive::TikTokLive;

#[tokio::main]
async fn main() {
    let user_name = "dash4214";
    let client = TikTokLive::new_client(user_name)
        .configure(configure)
        .on_event(handle_event)
        .build();

    client.connect().await;

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() && input.trim() == "stop" {
        //client.disconnect();
    }
}

fn handle_event(_client: &TikTokLiveClient, event: &TikTokLiveEvent) {
    match event {
        TikTokLiveEvent::OnMember(join_event) => {
            println!("user: {}  joined", join_event.raw_data.user.nickname);
        }
        TikTokLiveEvent::OnChat(chat_event) => {
            println!(
                "user: {} -> {} ",
                chat_event.raw_data.user.nickname, chat_event.raw_data.content
            );
        }
        TikTokLiveEvent::OnGift(gift_event) => {
            let nick = &gift_event.raw_data.user.nickname;
            let gift_name = &gift_event.raw_data.gift.name;
            let gifts_amount = gift_event.raw_data.gift.combo;

            println!(
                "user: {} sends gift: {} x {}",
                nick, gift_name, gifts_amount
            );
        }
        _ => {}
    }
}

fn configure(settings: &mut TikTokLiveSettings) {
    settings.http_data.time_out = Duration::from_secs(12);
}
