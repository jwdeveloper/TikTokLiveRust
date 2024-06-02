<div align="center" >
<a target="blank" >
<img src="https://github.com/jwdeveloper/TikTokLiveRust/assets/79764581/5ae5385c-d31a-40d9-aa5c-9c8b06aae161" width="15%" >
</a>
</div>
<div align="center" >
<h1>TikTok Live Rust</h1>

❤️❤️🎁 *Connect to TikTok live in 3 lines* 🎁❤️❤️

<div align="center" >

<a href="https://crates.io/crates/tiktoklive" target="blank" >
<img src="https://img.shields.io/crates/v/tiktoklive.svg" >
</a>

<a href="https://discord.gg/e2XwPNTBBr" target="blank" >
<img src="https://img.shields.io/discord/977648006063091742.svg?color=7289da&logo=discord&style=flat-square" >
</a>

</div>
</div>

# Introduction
A Rust library. Use it to receive live stream events such as comments and gifts in realtime from [TikTok LIVE](https://www.tiktok.com/live) No credentials are required.

Join the support [discord](https://discord.gg/e2XwPNTBBr) and visit the `#rust-support` channel for questions, contributions and ideas. Feel free to make pull requests with missing/new features, fixes, etc

Do you prefer other programming languages?
- **Java**   [TikTokLiveJava](https://github.com/jwdeveloper/TikTokLiveJava)
- **Node**   [TikTok-Live-Connector](https://github.com/zerodytrash/TikTok-Live-Connector) by [@zerodytrash](https://github.com/zerodytrash)
- **Python** [TikTokLive](https://github.com/isaackogan/TikTokLive) by [@isaackogan](https://github.com/isaackogan)
- **C#**     [TikTokLiveSharp](https://github.com/frankvHoof93/TikTokLiveSharp) by [@frankvHoof93](https://github.com/frankvHoof93)

**NOTE:** This is not an official API. It's a reverse engineering project.


#### Overview
- [Getting started](#getting-started)
- [Documentation](https://docs.rs/tiktoklive/latest/tiktoklive/)
- [Contributing](#contributing)

## Getting started
```toml
[dependencies]
 tiktoklive = "0.0.11"
 tokio = { version = "1.35.1", features = ["full"] }
```
```rust
use std::time::Duration;
use tiktoklive::core::live_client::TikTokLiveClient;
use tiktoklive::data::live_common::TikTokLiveSettings;
use tiktoklive::generated::events::TikTokLiveEvent;
use tiktoklive::TikTokLive;
use tokio::signal;

#[tokio::main]
async fn main() {
    let user_name = "tragdate";
    let client = TikTokLive::new_client(user_name)
        .configure(configure)
        .on_event(handle_event)
        .build();

    let _ = tokio::spawn(async move {
        client.connect().await;
    });

    //Wait for Ctrl+C to exit
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
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

        TikTokLiveEvent::OnLike(like_event) => {
            let nick = &like_event.raw_data.user.nickname;
            println!("user: {} likes", nick);
        }
        _ => {}
    }
}

fn configure(settings: &mut TikTokLiveSettings) {
    settings.http_data.time_out = Duration::from_secs(12);
}
```

## For streams that are flagged adult/disturbing

#### You should use an authenticated account by injecting the cookie in the headers

```rust
fn configure(settings: &mut TikTokLiveSettings) {
    settings.http_data.time_out = Duration::from_secs(12);
    settings
        .http_data
        .headers
        .insert("Cookie".to_string(), "your-cookie".to_string());
}
```



## Contributing
Your improvements are welcome! Feel free to open an <a href="https://github.com/jwdeveloper/TikTok-Live-Java/issues">issue</a> or <a href="https://github.com/jwdeveloper/TikTok-Live-Java/pulls">pull request</a>.

## Contributors
[Zmole Cristian](https://github.com/ZmoleCristian)
