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

### Dependencies

```toml
[dependencies]
tiktoklive = "0.0.13"
tokio = { version = "1.35.1", features = ["full"] }
serde_json = "1.0"
log = "0.4"
env_logger = "0.10.1"
```

### Usage example

```rust
use env_logger::{Builder, Env}; // Importing the logger builder and environment configuration
use log::LevelFilter; // Importing log level filter
use log::{error, warn};
use std::time::Duration; // Importing Duration for timeout settings
use tiktoklive::{
    // Importing necessary modules and structs from tiktoklive crate
    core::live_client::TikTokLiveClient,
    data::live_common::{ClientData, StreamData, TikTokLiveSettings},
    errors::LibError,
    generated::events::TikTokLiveEvent,
    TikTokLive,
};
use tokio::signal; // Importing signal handling from tokio

#[tokio::main] // Main function is asynchronous and uses tokio runtime
async fn main() {
    init_logger("info"); // Initialize logger with "info" level

    let user_name = "tragdate"; // Define the TikTok username

    let client = create_client(user_name); // Create a client for the given username

    // Spawn a new asynchronous task to connect the client
    let handle = tokio::spawn(async move {
        // Attempt to connect the client
        if let Err(e) = client.connect().await {
            match e {
                // Match on the error type
                LibError::LiveStatusFieldMissing => {
                    // Specific error case
                    warn!(
                        "Failed to get live status (probably needs authenticated client): {}",
                        e
                    );
                    let auth_client = create_client_with_cookies(user_name); // Create an authenticated client
                    if let Err(e) = auth_client.connect().await {
                        // Attempt to connect the authenticated client
                        error!("Error connecting to TikTok Live after retry: {}", e);
                    }
                }
                _ => {
                    // General error case
                    error!("Error connecting to TikTok Live: {}", e);
                }
            }
        }
    });

    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C"); // Wait for Ctrl+C signal to gracefully shut down

    handle.await.expect("The spawned task has panicked"); // Await the spawned task to ensure it completes
}

fn handle_event(client: &TikTokLiveClient, event: &TikTokLiveEvent) {
    match event {
        TikTokLiveEvent::OnConnected(..) => {
            // This is an experimental and unstable feature
            // Get room info from the client
            let room_info = client.get_room_info();
            // Parse the room info
            let client_data: ClientData = serde_json::from_str(room_info).unwrap();
            // Parse the stream data
            let stream_data: StreamData = serde_json::from_str(
                &client_data
                    .data
                    .stream_url
                    .live_core_sdk_data
                    .pull_data
                    .stream_data,
            )
            .unwrap();
            // Get the video URL for the low definition stream with fallback to the high definition stream in a flv format
            let video_url = stream_data
                .data
                .ld
                .map(|ld| ld.main.flv)
                .or_else(|| stream_data.data.sd.map(|sd| sd.main.flv))
                .or_else(|| stream_data.data.origin.map(|origin| origin.main.flv))
                .expect("None of the stream types set");
            println!("room info: {}", video_url);
        }

        // Match on the event type
        TikTokLiveEvent::OnMember(join_event) => {
            // Handle member join event
            println!("user: {} joined", join_event.raw_data.user.nickname);
        }
        TikTokLiveEvent::OnChat(chat_event) => {
            // Handle chat event
            println!(
                "user: {} -> {}",
                chat_event.raw_data.user.nickname, chat_event.raw_data.content
            );
        }
        TikTokLiveEvent::OnGift(gift_event) => {
            // Handle gift event
            let nick = &gift_event.raw_data.user.nickname;
            let gift_name = &gift_event.raw_data.gift.name;
            let gifts_amount = gift_event.raw_data.gift.combo;
            println!(
                "user: {} sends gift: {} x {}",
                nick, gift_name, gifts_amount
            );
        }
        TikTokLiveEvent::OnLike(like_event) => {
            // Handle like event
            let nick = &like_event.raw_data.user.nickname;
            println!("user: {} likes", nick);
        }
        _ => {} // Ignore other events
    }
}

// Function to initialize the logger with a default log level
fn init_logger(default_level: &str) {
    let env = Env::default().filter_or("LOG_LEVEL", default_level); // Set default log level from environment or use provided level
    Builder::from_env(env) // Build the logger from environment settings
        .filter_module("tiktoklive", LevelFilter::Debug) // Set log level for tiktoklive module
        .init(); // Initialize the logger
}

// Function to configure the TikTok live settings
fn configure(settings: &mut TikTokLiveSettings) {
    settings.http_data.time_out = Duration::from_secs(12); // Set HTTP timeout to 12 seconds
}

// Function to configure the TikTok live settings with cookies for authentication
fn configure_with_cookies(settings: &mut TikTokLiveSettings) {
    settings.http_data.time_out = Duration::from_secs(12); // Set HTTP timeout to 12 seconds
    let contents = ""; // Placeholder for cookies
    settings
        .http_data
        .headers
        .insert("Cookie".to_string(), contents.to_string());
    // Insert cookies into HTTP headers
}

// Function to create a TikTok live client for the given username
fn create_client(user_name: &str) -> TikTokLiveClient {
    TikTokLive::new_client(user_name) // Create a new client
        .configure(configure) // Configure the client
        .on_event(handle_event) // Set the event handler
        .build() // Build the client
}

// Function to create a TikTok live client with cookies for the given username
fn create_client_with_cookies(user_name: &str) -> TikTokLiveClient {
    TikTokLive::new_client(user_name) // Create a new client
        .configure(configure_with_cookies) // Configure the client with cookies
        .on_event(handle_event) // Set the event handler
        .build() // Build the client
}
```

## Library errors table 

You can catch errors on events with 

```rust
use tiktoklive::LibError;

if let Err(e) = client.connect().await {
    match e {
        LibError::UserFieldMissing => {
            println!("User field is missing");
        }
        _ => {
            eprintln!("Error connecting to TikTok Live: {}", e);
        }
    }
}
```

| Error type | Description |
| --- | --- |
| RoomIDFieldMissing | Room ID field is missing, contact developer |
| UserFieldMissing | User field is missing |
| UserDataFieldMissing | User data field is missing |
| LiveDataFieldMissing | Live data field is missing |
| JsonParseError | Error parsing JSON |
| UserMessageFieldMissing | User message field is missing |
| ParamsError | Params error |
| UserStatusFieldMissing | User status field is missing |
| LiveStatusFieldMissing | Live status field is missing |
| TitleFieldMissing | Title field is missing |
| UserCountFieldMissing | User count field is missing |
| StatsFieldMissing | Stats field is missing |
| LikeCountFieldMissing | Like count is missing |
| TotalUserFieldMissing | Total user field is missing |
| LiveRoomFieldMissing | Live room field is missing |
| StartTimeFieldMissing | Start time field is missing |
| UserNotFound | User not found |
| HostNotOnline | Live stream for host is not online!, current status HostOffline |
| InvalidHost | Invalid host in WebSocket URL |
| WebSocketConnectFailed | Failed to connect to WebSocket |
| PushFrameParseError | Unable to read push frame |
| WebcastResponseParseError | Unable to read webcast response |
| AckPacketSendError | Unable to send ack packet |
| HttpRequestFailed | HTTP request failed |
| UrlSigningFailed | URL signing failed |
| HeaderNotReceived | Header was not received |
| BytesParseError | Unable to parse bytes to Push Frame |



## Contributing
Your improvements are welcome! Feel free to open an <a href="https://github.com/jwdeveloper/TikTok-Live-Java/issues">issue</a> or <a href="https://github.com/jwdeveloper/TikTok-Live-Java/pulls">pull request</a>.

## Contributors
[Zmole Cristian](https://github.com/ZmoleCristian)
