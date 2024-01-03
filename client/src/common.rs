
pub mod live_common;

use crate::common::live_common::{TikTokHttpData, TikTokLiveSettings};



pub const TIKTOK_URL_WEB: &str = "https://www.tiktok.com/";
pub const TIKTOK_URL_WEBCAST: &str = "https://webcast.tiktok.com/webcast/";
pub const TIKTOK_SIGN_API: &str = "https://tiktok.eulerstream.com/webcast/sign_url";

pub fn create_default_settings() -> TikTokLiveSettings
{
    return TikTokLiveSettings
    {
        language: "en-US".to_string(),
        print_logs: true,
        reconnect_on_fail: true,
        http_data: TikTokHttpData
        {
            http_cookies: create_default_cookies(),
            http_headers: create_default_headers(),
            http_params: create_default_params(),
        },
    };
}


fn create_default_params() -> Vec<(String, String)>
{
    let mut params: Vec<(String, String)> = Vec::new();
    return params;
}

fn create_default_headers() -> Vec<(String, String)>
{
    let mut headers: Vec<(String, String)> = Vec::new();
    return headers;
}


fn create_default_cookies() -> Vec<(String, String)>
{
    return Vec::new();
}