use std::fmt;

#[derive(Debug)]
pub enum LibError {
    RoomIDFieldMissing,
    UserFieldMissing,
    UserDataFieldMissing,
    LiveDataFieldMissing,
    JsonParseError,
    UserMessageFieldMissing,
    ParamsError,
    UserStatusFieldMissing,
    LiveStatusFieldMissing,
    TitleFieldMissing,
    UserCountFieldMissing,
    StatsFieldMissing,
    LikeCountFieldMissing,
    TotalUserFieldMissing,
    LiveRoomFieldMissing,
    StartTimeFieldMissing,
    UserNotFound,
    HostNotOnline,
    InvalidHost,
    WebSocketConnectFailed,
    PushFrameParseError,
    WebcastResponseParseError,
    AckPacketSendError,
    HttpRequestFailed,
    UrlSigningFailed,
    HeaderNotReceived,
    BytesParseError,
}

impl fmt::Display for LibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LibError::RoomIDFieldMissing => {
                write!(f, "Room ID field is missing, contact developer")
            }
            LibError::UserFieldMissing => write!(f, "User field is missing"),
            LibError::UserDataFieldMissing => write!(f, "User data field is missing"),
            LibError::LiveDataFieldMissing => write!(f, "Live data field is missing"),
            LibError::JsonParseError => write!(f, "Error parsing JSON"),
            LibError::UserMessageFieldMissing => write!(f, "User message field is missing"),
            LibError::ParamsError => write!(f, "Params error"),
            LibError::UserStatusFieldMissing => write!(f, "User status field is missing"),
            LibError::LiveStatusFieldMissing => write!(f, "Live status field is missing"),
            LibError::TitleFieldMissing => write!(f, "Title field is missing"),
            LibError::UserCountFieldMissing => write!(f, "User count field is missing"),
            LibError::StatsFieldMissing => write!(f, "Stats field is missing"),
            LibError::LikeCountFieldMissing => write!(f, "Like count is missing"),
            LibError::TotalUserFieldMissing => write!(f, "Total user field is missing"),
            LibError::LiveRoomFieldMissing => write!(f, "Live room field is missing"),
            LibError::StartTimeFieldMissing => write!(f, "Start time field is missing"),
            LibError::UserNotFound => write!(f, "User not found"),
            LibError::HostNotOnline => write!(
                f,
                "Live stream for host is not online!, current status HostOffline"
            ),
            LibError::InvalidHost => write!(f, "Invalid host in WebSocket URL"),
            LibError::WebSocketConnectFailed => write!(f, "Failed to connect to WebSocket"),
            LibError::PushFrameParseError => write!(f, "Unable to read push frame"),
            LibError::WebcastResponseParseError => write!(f, "Unable to read webcast response"),
            LibError::AckPacketSendError => write!(f, "Unable to send ack packet"),
            LibError::HttpRequestFailed => write!(f, "HTTP request failed"),
            LibError::UrlSigningFailed => write!(f, "URL signing failed"),
            LibError::HeaderNotReceived => write!(f, "Header was not received"),
            LibError::BytesParseError => write!(f, "Unable to parse bytes to Push Frame!"),
        }
    }
}

impl std::error::Error for LibError {}
