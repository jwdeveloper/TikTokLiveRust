use std::rc::Rc;

use log::{error, info, warn};

use crate::core::live_client_events::TikTokLiveEventObserver;
use crate::core::live_client_http::TikTokLiveHttpClient;
use crate::core::live_client_websocket::TikTokLiveWebsocketClient;
use crate::data::live_common::{ConnectionState, TikTokLiveInfo, TikTokLiveSettings};
use crate::data::live_common::ConnectionState::{CONNECTED, CONNECTING, DISCONNECTED};
use crate::data::live_events::TikTokLiveEvent;
use crate::http::http_data::{LiveConnectionDataRequest, LiveDataRequest, LiveUserDataRequest};
use crate::http::http_data::LiveStatus::HostOnline;

pub struct TikTokLiveClient
{
    pub(crate) settings: TikTokLiveSettings,
    pub(crate) http_client: TikTokLiveHttpClient,
    pub(crate) event_observer:  Rc<TikTokLiveEventObserver>,
    pub(crate) websocket_client: TikTokLiveWebsocketClient,
    pub room_info: TikTokLiveInfo,

}

impl TikTokLiveClient
{


    pub async fn connect(&self)
    {
        if self.room_info.connection_state.take() != DISCONNECTED {
            warn!("Client already connected!");
            return;
        }

        self.set_connection_state(CONNECTING);

        info!("Getting live user information's");
        let response = self.http_client.fetch_live_user_data(LiveUserDataRequest
        {
            user_name: self.settings.host_name.clone()
        }).await;

        info!("Getting live room information's");
        let room_id = response.room_id;
        let response = self.http_client.fetch_live_data(LiveDataRequest
        {
            room_id: room_id.clone()
        }).await;
        if response.live_status != HostOnline
        {
            error!("Live stream for host is not online!, current status {:?}",response.live_status);
            return;
        }

        info!("Getting live connections information's");
        let response = self.http_client.fetch_live_connection_data(LiveConnectionDataRequest
        {
            room_id: room_id.clone()
        }).await;

        self.websocket_client.start(response, &self).await;
        self.set_connection_state(CONNECTED);
    }

    pub fn disconnect(&self)
    {
        self.websocket_client.stop();
        self.set_connection_state(DISCONNECTED)
    }

    pub fn publish_event(&self, event: TikTokLiveEvent)
    {
        self.event_observer.publish(self, event);
    }


    fn set_connection_state(&self, state: ConnectionState)
    {
        self.room_info.connection_state.set(state);
        info!("TikTokLive: {:?}", &self.room_info.connection_state.take());
    }
}