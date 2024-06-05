use crate::errors::LibError;
use crate::http::http_data::LiveStatus::{HostNotFound, HostOffline, HostOnline};
use crate::http::http_data::UserStatus::{Live, LivePaused, NotFound, Offline};
use crate::http::http_data::{LiveDataResponse, LiveUserDataResponse, SignServerResponse};
use serde_json::Value;

pub fn map_live_user_data_response(json: String) -> Result<LiveUserDataResponse, LibError> {
    let json_value: Value =
        serde_json::from_str(json.as_str()).map_err(|_| LibError::JsonParseError)?;

    let message = json_value["message"]
        .as_str()
        .ok_or(LibError::UserMessageFieldMissing)?;
    match message {
        "params_error" => return Err(LibError::ParamsError),
        "user_not_found" => return Err(LibError::UserNotFound),
        _ => {}
    }

    let option_data = json_value["data"]
        .as_object()
        .ok_or(LibError::UserDataFieldMissing)?;
    let user = option_data["user"]
        .as_object()
        .ok_or(LibError::UserFieldMissing)?;
    let room_id = user["roomId"]
        .as_str()
        .ok_or(LibError::RoomIDFieldMissing)?;
    let status = user["status"]
        .as_i64()
        .ok_or(LibError::UserStatusFieldMissing)?;

    let user_status = match status {
        2 => Live,
        3 => LivePaused,
        4 => Offline,
        _ => NotFound,
    };

    let live_room = option_data["liveRoom"]
        .as_object()
        .ok_or(LibError::LiveRoomFieldMissing)?;
    let start_time = live_room["startTime"]
        .as_i64()
        .ok_or(LibError::StartTimeFieldMissing)?;

    Ok(LiveUserDataResponse {
        user_status,
        json,
        room_id: room_id.to_string(),
        started_at_timestamp: start_time,
    })
}

pub fn map_live_data_response(json: String) -> Result<LiveDataResponse, LibError> {
    let json_value: Value = serde_json::from_str(&json).map_err(|_| LibError::JsonParseError)?;

    let data = json_value["data"]
        .as_object()
        .ok_or(LibError::LiveDataFieldMissing)?;

    let status = data
        .get("status")
        .and_then(|v| v.as_i64())
        .ok_or(LibError::LiveStatusFieldMissing)?;
    let title = data
        .get("title")
        .and_then(|v| v.as_str())
        .ok_or(LibError::TitleFieldMissing)?;
    let viewers = data
        .get("user_count")
        .and_then(|v| v.as_i64())
        .ok_or(LibError::UserCountFieldMissing)?;
    let live_status = match status {
        2 => HostOnline,
        4 => HostOffline,
        _ => HostNotFound,
    };

    let stats = data
        .get("stats")
        .and_then(|v| v.as_object())
        .ok_or(LibError::StatsFieldMissing)?;
    let likes = stats
        .get("like_count")
        .and_then(|v| v.as_i64())
        .ok_or(LibError::LikeCountFieldMissing)?;
    let total_viewers = stats
        .get("total_user")
        .and_then(|v| v.as_i64())
        .ok_or(LibError::TotalUserFieldMissing)?;

    Ok(LiveDataResponse {
        json,
        live_status,
        total_viewers,
        viewers,
        likes,
        title: title.to_string(),
    })
}

pub fn map_sign_server_response(json: String) -> SignServerResponse {
    let json_value: Value = serde_json::from_str(json.as_str()).unwrap();
    let signed_url = json_value["signedUrl"].as_str().unwrap();
    let user_agent = json_value["User-Agent"].as_str().unwrap();

    SignServerResponse {
        signed_url: signed_url.to_string(),
        user_agent: user_agent.to_string(),
    }
}
