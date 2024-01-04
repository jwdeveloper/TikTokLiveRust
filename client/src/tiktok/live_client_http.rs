use std::str::Utf8Error;
use std::time::Duration;
use protobuf::Message;
use reqwest::header::HeaderValue;
use crate::common::live_common::TikTokLiveSettings;
use crate::http::http_data::{LiveDataRequest, LiveDataResponse, LiveStatus, LiveConnectionDataRequest, LiveUserDataResponse, UserStatus, LiveUserDataRequest, LiveConnectionDataResponse};
use crate::http::http_data_mappers::{map_live_data_response, map_live_user_data_response, map_sign_server_response};
use crate::http::http_request_builder::HttpRequestFactory;
use crate::proto::messages::webcast::WebcastResponse;

pub struct TikTokLiveHttpClient
{
    pub(crate) settings: TikTokLiveSettings,
    pub(crate) factory: HttpRequestFactory,
}

pub const TIKTOK_URL_WEB: &str = "https://www.tiktok.com/";
pub const TIKTOK_URL_WEBCAST: &str = "https://webcast.tiktok.com/webcast/";
pub const TIKTOK_SIGN_API: &str = "https://tiktok.eulerstream.com/webcast/sign_url";

impl TikTokLiveHttpClient
{
    pub async fn fetch_live_user_data(&self, request: LiveUserDataRequest) -> LiveUserDataResponse
    {
        let url = format!("{}{}", TIKTOK_URL_WEB, "api-live/user/room");
        let option = self.factory
            .request()
            .withUrl(url.as_str())
            .withParam("uniqueId", request.user_name.as_str())
            .withParam("sourceType", "54")
            .asJson()
            .await;

        if (option.is_none())
        {
            panic!("Unable to get info about user ")
        }
        let json = option.unwrap();
        return map_live_user_data_response(json);
    }
    pub async fn fetch_live_data(&self, request: LiveDataRequest) -> LiveDataResponse
    {
        let url = format!("{}{}", TIKTOK_URL_WEBCAST, "room/info");
        let option = self.factory
            .request()
            .withUrl(url.as_str())
            .withParam("room_id", request.room_id.as_str())
            .asJson()
            .await;


        if (option.is_none())
        {
            panic!("Unable to get info about live room")
        }
        let json = option.unwrap();
        return map_live_data_response(json);
    }

    pub async fn fetch_live_connection_data(&self, request: LiveConnectionDataRequest) -> LiveConnectionDataResponse
    {
        //Preparing URL to sign
        let url_to_sign = self.factory
            .request()
            .withUrl(format!("{}{}", TIKTOK_URL_WEBCAST, "im/fetch").as_str())
            .withParam("room_id", request.room_id.as_str())
            .asUrl();

        //Signing URL
        let option = self.factory
            .request()
            .withUrl(TIKTOK_SIGN_API)
            .withParam("client", "ttlive-rust")
            .withParam("uuc", "1")
            .withParam("url", url_to_sign.as_str())
            .asJson()
            .await;

        if (option.is_none())
        {
            panic!("Unable sign url {}", url_to_sign.as_str())
        }
        let json = option.unwrap();
        let sign_server_response = map_sign_server_response(json);

        //Getting credentials for connection to websocket
        let mut response = self.factory
            .request()
            .withReset()
            .withTimeOut(self.settings.http_data.time_out)
            .withUrl(sign_server_response.signed_url.as_str())
            .build_get_request()
            .send()
            .await
            .unwrap();


        let optionalHeader = response.headers().get("set-cookie");

        if (optionalHeader.is_none())
        {
            panic!("Header was not received not provided")
        }
        let headerValue = optionalHeader.unwrap().to_str().unwrap().to_string();


        let protocol_buffer_message = response.bytes().await.unwrap();
        let webcast_response = WebcastResponse::parse_from_bytes(protocol_buffer_message.as_ref())
            .expect("Unable to parse bytes to Push Frame!");


        //preparing websocket url
        let web_socket_url = self.factory
            .request()
            .withUrl(webcast_response.pushServer.as_str())
            .withParam("room_id", request.room_id.as_str())
            .withParam("cursor", webcast_response.cursor.as_str())
            .withParam("resp_content_type", "protobuf")
            .withParam("internal_ext", webcast_response.internalExt.as_str())
            .withParams(&webcast_response.routeParamsMap)
            .asUrl();

        let url = url::Url::parse(web_socket_url.as_str()).unwrap();

        return LiveConnectionDataResponse
        {
            web_socket_timeout: self.settings.http_data.time_out,
            web_socket_cookies: headerValue,
            web_socket_url: url,
        };
    }


    fn parse_cookie_header(header: &HeaderValue) -> Result<(String, String), Utf8Error> {
        let cookie_str = header.to_str().unwrap();
        let mut parts = cookie_str.splitn(2, '=');
        let key = parts.next().unwrap_or_default().to_string();
        let value = parts.next().unwrap_or_default().to_string();
        Ok((key, value))
    }
}