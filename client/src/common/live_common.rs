
pub struct TikTokLiveSettings
{
    pub language: String,
    pub reconnect_on_fail: bool,
    pub print_logs: bool,
    pub http_data: TikTokHttpData,
}


pub struct TikTokHttpData
{
    pub http_params: Vec<(String, String)>,
    pub http_headers: Vec<(String, String)>,
    pub http_cookies: Vec<(String, String)>,
}