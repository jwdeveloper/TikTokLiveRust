use std::time::Duration;

use url::Url;

pub struct LiveUserDataRequest
{
    pub user_name: String,
}

pub struct LiveUserDataResponse
{
    pub room_id: String,
    pub started_at_timestamp: i64,
    pub user_status: UserStatus,
    pub json: String,
}

pub struct LiveDataRequest
{
    pub room_id: String,
}

pub struct LiveDataResponse
{
    pub json: String,
    pub live_status: LiveStatus,
    pub title: String,
    pub likes: i64,
    pub viewers: i64,
    pub total_viewers: i64,
}


pub struct LiveConnectionDataRequest
{
    pub room_id: String,
}

pub struct LiveConnectionDataResponse
{
    pub web_socket_timeout: Duration,
    pub web_socket_cookies: String,
    pub web_socket_url: Url,
}


pub struct SignServerResponse
{
    pub signed_url: String,
    pub user_agent: String,
}


pub enum UserStatus
{
    NotFound,
    Offline,
    LivePaused,
    Live,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum LiveStatus
{
    HostNotFound,
    HostOnline,
    HostOffline,
}