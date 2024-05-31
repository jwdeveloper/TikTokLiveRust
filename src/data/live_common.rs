use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Clone)]
pub struct TikTokLiveSettings {
    pub host_name: String,
    pub language: String,
    pub reconnect_on_fail: bool,
    pub print_logs: bool,
    pub http_data: HttpData,
}

#[derive(Clone, Default)]
pub struct HttpData {
    pub time_out: Duration,
    pub params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
}

#[derive(Default)]
pub struct TikTokLiveInfo {
    pub room_id: String,
    pub likes: i32,
    pub viewers: i32,
    pub total_viewers: i32,
    pub host_name: String,
    pub title: String,
    pub language: String,
    pub connection_state: Mutex<ConnectionState>,
}

#[derive(PartialEq, Debug)]
pub enum ConnectionState {
    CONNECTING,
    CONNECTED,
    DISCONNECTED,
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::DISCONNECTED
    }
}
