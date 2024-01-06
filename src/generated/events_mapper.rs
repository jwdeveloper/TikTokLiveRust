use protobuf::Message;
use crate::core::live_client::TikTokLiveClient;
use crate::generated::messages::webcast::*;
use crate::core::live_client_mapper::TikTokLiveMessageMapper;
use crate::generated::events::*;
impl TikTokLiveMessageMapper {
    pub fn handle_single_message(
        &self,
        message: crate::generated::messages::webcast::webcast_response::Message,
        client: &TikTokLiveClient,
    ) {
        let proto_message_name = &message.method;
        let proto_message_content = &message.payload;
        match proto_message_name.as_str() {
            "WebcastLikeMessage" => {
                let raw_data = WebcastLikeMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokLikeEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnLike(event));
            }
            "WebcastQuestionNewMessage" => {
                let raw_data = WebcastQuestionNewMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokQuestionNewEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnQuestionNew(event));
            }
            "WebcastLinkMicBattlePunishFinish" => {
                let raw_data = WebcastLinkMicBattlePunishFinish::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokLinkMicBattlePunishFinishEvent {
                    raw_data,
                };
                client
                    .publish_event(TikTokLiveEvent::OnLinkMicBattlePunishFinish(event));
            }
            "WebcastRankUpdateMessage" => {
                let raw_data = WebcastRankUpdateMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokRankUpdateEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnRankUpdate(event));
            }
            "WebcastLinkMicFanTicketMethod" => {
                let raw_data = WebcastLinkMicFanTicketMethod::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokLinkMicFanTicketMethodEvent {
                    raw_data,
                };
                client.publish_event(TikTokLiveEvent::OnLinkMicFanTicketMethod(event));
            }
            "WebcastLiveIntroMessage" => {
                let raw_data = WebcastLiveIntroMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokLiveIntroEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnLiveIntro(event));
            }
            "WebcastMemberMessage" => {
                let raw_data = WebcastMemberMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokMemberEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnMember(event));
            }
            "WebcastChatMessage" => {
                let raw_data = WebcastChatMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokChatEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnChat(event));
            }
            "WebcastLinkMicArmies" => {
                let raw_data = WebcastLinkMicArmies::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokLinkMicArmiesEvent {
                    raw_data,
                };
                client.publish_event(TikTokLiveEvent::OnLinkMicArmies(event));
            }
            "WebcastLinkLayerMessage" => {
                let raw_data = WebcastLinkLayerMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokLinkLayerEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnLinkLayer(event));
            }
            "WebcastResponse" => {
                let raw_data = WebcastResponse::parse_from_bytes(proto_message_content)
                    .unwrap();
                let event = TikTokResponseEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnResponse(event));
            }
            "WebcastPushFrame" => {
                let raw_data = WebcastPushFrame::parse_from_bytes(proto_message_content)
                    .unwrap();
                let event = TikTokPushFrameEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnPushFrame(event));
            }
            "WebcastRankTextMessage" => {
                let raw_data = WebcastRankTextMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokRankTextEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnRankText(event));
            }
            "WebcastSystemMessage" => {
                let raw_data = WebcastSystemMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokSystemEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnSystem(event));
            }
            "WebcastLinkmicBattleTaskMessage" => {
                let raw_data = WebcastLinkmicBattleTaskMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokLinkmicBattleTaskEvent {
                    raw_data,
                };
                client.publish_event(TikTokLiveEvent::OnLinkmicBattleTask(event));
            }
            "WebcastGiftMessage" => {
                let raw_data = WebcastGiftMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokGiftEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnGift(event));
            }
            "WebcastInRoomBannerMessage" => {
                let raw_data = WebcastInRoomBannerMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokInRoomBannerEvent {
                    raw_data,
                };
                client.publish_event(TikTokLiveEvent::OnInRoomBanner(event));
            }
            "WebcastMsgDetectMessage" => {
                let raw_data = WebcastMsgDetectMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokMsgDetectEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnMsgDetect(event));
            }
            "WebcastControlMessage" => {
                let raw_data = WebcastControlMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokControlEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnControl(event));
            }
            "WebcastLinkMicMethod" => {
                let raw_data = WebcastLinkMicMethod::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokLinkMicMethodEvent {
                    raw_data,
                };
                client.publish_event(TikTokLiveEvent::OnLinkMicMethod(event));
            }
            "WebcastGoalUpdateMessage" => {
                let raw_data = WebcastGoalUpdateMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokGoalUpdateEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnGoalUpdate(event));
            }
            "WebcastCaptionMessage" => {
                let raw_data = WebcastCaptionMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokCaptionEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnCaption(event));
            }
            "WebcastHourlyRankMessage" => {
                let raw_data = WebcastHourlyRankMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokHourlyRankEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnHourlyRank(event));
            }
            "WebcastBarrageMessage" => {
                let raw_data = WebcastBarrageMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokBarrageEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnBarrage(event));
            }
            "WebcastSubNotifyMessage" => {
                let raw_data = WebcastSubNotifyMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokSubNotifyEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnSubNotify(event));
            }
            "RoomVerifyMessage" => {
                let raw_data = RoomVerifyMessage::parse_from_bytes(proto_message_content)
                    .unwrap();
                let event = TikTokRoomVerifyEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnRoomVerify(event));
            }
            "WebcastSocialMessage" => {
                let raw_data = WebcastSocialMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokSocialEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnSocial(event));
            }
            "WebcastEmoteChatMessage" => {
                let raw_data = WebcastEmoteChatMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokEmoteChatEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnEmoteChat(event));
            }
            "WebcastPollMessage" => {
                let raw_data = WebcastPollMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokPollEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnPoll(event));
            }
            "WebcastRoomPinMessage" => {
                let raw_data = WebcastRoomPinMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokRoomPinEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnRoomPin(event));
            }
            "WebcastRoomMessage" => {
                let raw_data = WebcastRoomMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokRoomEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnRoom(event));
            }
            "WebcastEnvelopeMessage" => {
                let raw_data = WebcastEnvelopeMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokEnvelopeEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnEnvelope(event));
            }
            "WebcastImDeleteMessage" => {
                let raw_data = WebcastImDeleteMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokImDeleteEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnImDelete(event));
            }
            "WebcastRoomUserSeqMessage" => {
                let raw_data = WebcastRoomUserSeqMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokRoomUserSeqEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnRoomUserSeq(event));
            }
            "WebcastUnauthorizedMemberMessage" => {
                let raw_data = WebcastUnauthorizedMemberMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokUnauthorizedMemberEvent {
                    raw_data,
                };
                client.publish_event(TikTokLiveEvent::OnUnauthorizedMember(event));
            }
            "WebcastOecLiveShoppingMessage" => {
                let raw_data = WebcastOecLiveShoppingMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokOecLiveShoppingEvent {
                    raw_data,
                };
                client.publish_event(TikTokLiveEvent::OnOecLiveShopping(event));
            }
            "WebcastLinkMessage" => {
                let raw_data = WebcastLinkMessage::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokLinkEvent { raw_data };
                client.publish_event(TikTokLiveEvent::OnLink(event));
            }
            "WebcastLinkMicBattle" => {
                let raw_data = WebcastLinkMicBattle::parse_from_bytes(
                        proto_message_content,
                    )
                    .unwrap();
                let event = TikTokLinkMicBattleEvent {
                    raw_data,
                };
                client.publish_event(TikTokLiveEvent::OnLinkMicBattle(event));
            }
            _ => {
                client
                    .publish_event(
                        TikTokLiveEvent::OnWebsocketUnknownMessage(TikTokWebsocketUnknownMessageEvent {
                            message_name: message.method.clone(),
                            raw_data: message,
                        }),
                    );
            }
        }
    }
}
