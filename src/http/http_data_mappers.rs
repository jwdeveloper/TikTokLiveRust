use serde_json::Value;
use crate::http::http_data::{LiveDataResponse, LiveStatus, LiveUserDataResponse, SignServerResponse, UserStatus};
use crate::http::http_data::LiveStatus::{HostNotFound, HostOffline, HostOnline};
use crate::http::http_data::UserStatus::{Live, LivePaused, NotFound, Offline};

pub fn map_live_user_data_response(json: String) -> LiveUserDataResponse
{
    let json_value: Value = serde_json::from_str(json.as_str()).unwrap();


    let message = json_value["message"].as_str().unwrap();
    if (message.eq("params_error"))
    {
        panic!("fetchRoomIdFromTiktokApi -> Unable to fetch roomID, contact the developer");
    }
    if (message.eq("user_not_found")) {
        panic!("TikTokUserInfo.UserStatus.NotFound");
    }

    let option_data = json_value["data"].as_object();
    if (option_data.is_none())
    {
        panic!("TikTokUserInfo.UserStatus.NotFound");
    }
    let option = option_data.unwrap();
    let user = option["user"].as_object().unwrap();
    let roomId = user["roomId"].as_str().unwrap();
    let status = user["status"].as_i64().unwrap();

    let userStatus = match status {
        2 => Live,
        3 => LivePaused,
        4 => Offline,
        _ => NotFound,
    };

    let liveRoom = option["liveRoom"].as_object().unwrap();
    let startTime = liveRoom["startTime"].as_i64().unwrap();

    return LiveUserDataResponse
    {
        user_status: userStatus,
        json: json.to_string(),
        room_id: roomId.to_string(),
        started_at_timestamp: startTime,
    };
}

pub fn map_live_data_response(json: String) -> LiveDataResponse
{
    let json_value: Value = serde_json::from_str(json.as_str()).unwrap();

    let data = json_value["data"].as_object().unwrap();

    let status = data["status"].as_i64().unwrap();
    let title = data["title"].as_str().unwrap();
    let viewers = data["user_count"].as_i64().unwrap();
    let live_status = match status {
        2 => HostOnline,
        4 => HostOffline,
        _ => HostNotFound,
    };

    let stats = data["stats"].as_object().unwrap();
    let likes = stats["like_count"].as_i64().unwrap();
    let total_viewers = stats["total_user"].as_i64().unwrap();

    return LiveDataResponse
    {
        json,
        live_status,
        total_viewers,
        viewers,
        likes,
        title: title.to_string(),
    };
}

pub fn map_sign_server_response(json: String) -> SignServerResponse
{
    let json_value: Value = serde_json::from_str(json.as_str()).unwrap();
    let signed_url = json_value["signedUrl"].as_str().unwrap();
    let user_agent = json_value["User-Agent"].as_str().unwrap();

    return SignServerResponse
    {
        signed_url: signed_url.to_string(),
        user_agent: user_agent.to_string(),
    }
}