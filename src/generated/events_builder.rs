use crate::core::live_client::TikTokLiveClient;
use crate::core::live_client_builder::TikTokLiveBuilder;
use crate::generated::events::*;
///
///  Generated code
///
///
impl TikTokLiveBuilder {
    pub fn on_like(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokLikeEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnLike(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_question_new(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokQuestionNewEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnQuestionNew(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_link_mic_battle_punish_finish(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokLinkMicBattlePunishFinishEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnLinkMicBattlePunishFinish(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_rank_update(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokRankUpdateEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnRankUpdate(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_link_mic_fan_ticket_method(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokLinkMicFanTicketMethodEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnLinkMicFanTicketMethod(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_live_intro(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokLiveIntroEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnLiveIntro(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_member(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokMemberEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnMember(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_chat(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokChatEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnChat(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_link_mic_armies(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokLinkMicArmiesEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnLinkMicArmies(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_link_layer(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokLinkLayerEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnLinkLayer(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_websocket_response(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokWebsocketResponseEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnWebsocketResponse(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_response(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokResponseEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnResponse(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_push_frame(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokPushFrameEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnPushFrame(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_rank_text(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokRankTextEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnRankText(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_system(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokSystemEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnSystem(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_linkmic_battle_task(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokLinkmicBattleTaskEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnLinkmicBattleTask(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_gift(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokGiftEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnGift(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_in_room_banner(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokInRoomBannerEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnInRoomBanner(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_msg_detect(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokMsgDetectEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnMsgDetect(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_control(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokControlEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnControl(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_link_mic_method(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokLinkMicMethodEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnLinkMicMethod(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_goal_update(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokGoalUpdateEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnGoalUpdate(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_caption(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokCaptionEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnCaption(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_hourly_rank(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokHourlyRankEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnHourlyRank(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_disconnected(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokDisconnectedEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnDisconnected(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_barrage(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokBarrageEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnBarrage(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_sub_notify(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokSubNotifyEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnSubNotify(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_room_verify(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokRoomVerifyEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnRoomVerify(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_social(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokSocialEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnSocial(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_emote_chat(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokEmoteChatEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnEmoteChat(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_poll(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokPollEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnPoll(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_room_pin(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokRoomPinEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnRoomPin(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_room(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokRoomEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnRoom(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_envelope(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokEnvelopeEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnEnvelope(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_websocket_unknown_message(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokWebsocketUnknownMessageEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnWebsocketUnknownMessage(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_connected(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokConnectedEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnConnected(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_im_delete(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokImDeleteEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnImDelete(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_room_user_seq(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokRoomUserSeqEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnRoomUserSeq(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_unauthorized_member(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokUnauthorizedMemberEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnUnauthorizedMember(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_oec_live_shopping(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokOecLiveShoppingEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnOecLiveShopping(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_link(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokLinkEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnLink(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
    pub fn on_link_mic_battle(
        &mut self,
        on_event: fn(client: &TikTokLiveClient, event_data: &TikTokLinkMicBattleEvent),
    ) -> &mut Self {
        self.on_event(|client, incoming_event| match incoming_event {
            TikTokLiveEvent::OnLinkMicBattle(event_instance) => {
                on_event(client, event_instance);
            }
            _ => {}
        })
    }
}
