use crate::core::live_client_builder::TikTokLiveBuilder;
use crate::core::live_client_http::TikTokLiveHttpClient;

///  # Data structures
pub mod data;

///  # Core functionality of library
pub mod core;

///  # Http client and request handling
pub mod http;

///  # ProtocolBuffer structures
pub mod generated;



///
/// # Entry point of library used to create new instance of TikTokLive client
///
/// ```
/// use tiktoklive::TikTokLive;
///
/// let client = TikTokLive::new_client("some-user");
//         .configure(configure)
//         .on_event(on_event)
//         .build();
///   client.connect().await
/// ```
///
pub struct TikTokLive {}

impl TikTokLive
{
    ///
    /// Returns builder for creating new TikTokLiveClient
    ///
    pub fn new_client(user_name: &str) -> TikTokLiveBuilder
    {
        TikTokLiveBuilder::new(user_name)
    }


}