pub mod live_common;
pub mod live_events;

use std::any::Any;
use std::collections::HashMap;
use std::time::Duration;
use crate::common::live_common::{HttpData, TikTokLiveSettings};


pub fn create_default_settings(host_name: &str) -> TikTokLiveSettings
{
    return TikTokLiveSettings
    {
        language: "en-US".to_string(),
        print_logs: true,
        reconnect_on_fail: true,
        host_name: host_name.to_string(),
        http_data: HttpData
        {
            time_out: Duration::from_secs(3),
            cookies: create_default_cookies(),
            headers: create_default_headers(),
            params: create_default_params(),
        },
    };
}


fn create_default_params()  -> HashMap<String,String>
{
    let mut params: Vec<(&str, &str)> = Vec::new();
    params.push(("aid", "1988"));
    params.push(("app_language", "en-US"));
    params.push(("app_name", "tiktok_web"));
    params.push(("browser_language", "en"));
    params.push(("browser_name", "Mozilla"));
    params.push(("browser_online", "true"));
    params.push(("browser_platform", "Win32"));
    params.push(("browser_version", "5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36"));
    params.push(("cookie_enabled", "true"));
    params.push(("cursor", ""));
    params.push(("internal_ext", ""));
    params.push(("device_platform", "web"));
    params.push(("focus_state", "true"));
    params.push(("from_page", "user"));
    params.push(("history_len", "4"));
    params.push(("is_fullscreen", "false"));
    params.push(("is_page_visible", "true"));
    params.push(("did_rule", "3"));
    params.push(("fetch_rule", "1"));
    params.push(("identity", "audience"));
    params.push(("last_rtt", "0"));
    params.push(("live_id", "12"));
    params.push(("resp_content_type", "protobuf"));
    params.push(("screen_height", "1152"));
    params.push(("screen_width", "2048"));
    params.push(("tz_name", "Europe/Berlin"));
    params.push(("referer", "https, //www.tiktok.com/"));
    params.push(("root_referer", "https, //www.tiktok.com/"));
    params.push(("msToken", ""));
    params.push(("version_code", "180800"));
    params.push(("webcast_sdk_version", "1.3.0"));
    params.push(("update_version_code", "1.3.0"));

    return params.iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
}

fn create_default_headers()  -> HashMap<String,String>
{
    let mut headers: Vec<(&str, &str)> = Vec::new();

    headers.push(("authority", "www.tiktok.com"));

    headers.push(("Cache-Control", "max-age=0"));
    headers.push(("Accept", "text/html,application/json,application/protobuf"));
    headers.push(("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36"));
    headers.push(("Referer", "https://www.tiktok.com/"));
    headers.push(("Origin", "https://www.tiktok.com"));
    headers.push(("Accept-Language", "en-US,en; q=0.9"));


    return headers.iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
}


fn create_default_cookies() -> HashMap<String,String>
{
    return HashMap::new();
}