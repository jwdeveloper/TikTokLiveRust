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


<div align="center">
  <a href="https://www.youtube.com/watch?v=eerWGgUKc6c" align="right" target="blank"><img src="https://img.youtube.com/vi/eerWGgUKc6c/hqdefault.jpg" alt="IMAGE ALT TEXT" width="38%" align="right"></a>
</div>

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
 tiktoklive = "0.0.1"
```
```rust

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

```




## Contributing
Your improvements are welcome! Feel free to open an <a href="https://github.com/jwdeveloper/TikTok-Live-Java/issues">issue</a> or <a href="https://github.com/jwdeveloper/TikTok-Live-Java/pulls">pull request</a>.
