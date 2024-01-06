use protobuf::Message;

use crate::data::live_common::TikTokLiveSettings;
use crate::http::http_data::{LiveConnectionDataRequest, LiveConnectionDataResponse, LiveDataRequest, LiveDataResponse, LiveUserDataRequest, LiveUserDataResponse};
use crate::http::http_data_mappers::{map_live_data_response, map_live_user_data_response, map_sign_server_response};
use crate::http::http_request_builder::HttpRequestFactory;
use crate::generated::messages::webcast::WebcastResponse;

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
            .with_url(url.as_str())
            .with_param("uniqueId", request.user_name.as_str())
            .with_param("sourceType", "54")
            .as_json()
            .await;

        if option.is_none()
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
            .with_url(url.as_str())
            .with_param("room_id", request.room_id.as_str())
            .as_json()
            .await;


        if option.is_none()
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
            .with_url(format!("{}{}", TIKTOK_URL_WEBCAST, "im/fetch").as_str())
            .with_param("room_id", request.room_id.as_str())
            .as_url();

        //Signing URL
        let option = self.factory
            .request()
            .with_url(TIKTOK_SIGN_API)
            .with_param("client", "ttlive-rust")
            .with_param("uuc", "1")
            .with_param("url", url_to_sign.as_str())
            .as_json()
            .await;

        if option.is_none()
        {
            panic!("Unable sign url {}", url_to_sign.as_str())
        }
        let json = option.unwrap();
        let sign_server_response = map_sign_server_response(json);

        //Getting credentials for connection to websocket
        let response = self.factory
            .request()
            .with_reset()
            .with_time_out(self.settings.http_data.time_out)
            .with_url(sign_server_response.signed_url.as_str())
            .build_get_request()
            .send()
            .await
            .unwrap();


        let optional_header = response.headers().get("set-cookie");

        if optional_header.is_none()
        {
            panic!("Header was not received not provided")
        }
        let header_value = optional_header.unwrap().to_str().unwrap().to_string();


        let protocol_buffer_message = response.bytes().await.unwrap();
        let webcast_response = WebcastResponse::parse_from_bytes(protocol_buffer_message.as_ref())
            .expect("Unable to parse bytes to Push Frame!");


        //preparing websocket url
        let web_socket_url = self.factory
            .request()
            .with_url(webcast_response.pushServer.as_str())
            .with_param("room_id", request.room_id.as_str())
            .with_param("cursor", webcast_response.cursor.as_str())
            .with_param("resp_content_type", "protobuf")
            .with_param("internal_ext", webcast_response.internalExt.as_str())
            .with_params(&webcast_response.routeParamsMap)
            .as_url();

        let url = url::Url::parse(web_socket_url.as_str()).unwrap();

        return LiveConnectionDataResponse
        {
            web_socket_timeout: self.settings.http_data.time_out,
            web_socket_cookies: header_value,
            web_socket_url: url,
        };
    }
}