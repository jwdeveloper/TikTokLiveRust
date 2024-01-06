use crate::generated::messages::webcast::*;
use crate::generated::messages::webcast::webcast_response::Message;
///
/// Generated file
///
pub struct TikTokRankTextEvent {
    pub raw_data: WebcastRankTextMessage,
}
pub struct TikTokPollEvent {
    pub raw_data: WebcastPollMessage,
}
pub struct TikTokSubNotifyEvent {
    pub raw_data: WebcastSubNotifyMessage,
}
pub struct TikTokGoalUpdateEvent {
    pub raw_data: WebcastGoalUpdateMessage,
}
pub struct TikTokPushFrameEvent {
    pub raw_data: WebcastPushFrame,
}
pub struct TikTokEnvelopeEvent {
    pub raw_data: WebcastEnvelopeMessage,
}
pub struct TikTokSocialEvent {
    pub raw_data: WebcastSocialMessage,
}
pub struct TikTokRoomUserSeqEvent {
    pub raw_data: WebcastRoomUserSeqMessage,
}
pub struct TikTokLinkEvent {
    pub raw_data: WebcastLinkMessage,
}
pub struct TikTokDisconnectedEvent {}
pub struct TikTokMemberEvent {
    pub raw_data: WebcastMemberMessage,
}
pub struct TikTokLikeEvent {
    pub raw_data: WebcastLikeMessage,
}
pub struct TikTokLinkMicBattleEvent {
    pub raw_data: WebcastLinkMicBattle,
}
pub struct TikTokCaptionEvent {
    pub raw_data: WebcastCaptionMessage,
}
pub struct TikTokRoomPinEvent {
    pub raw_data: WebcastRoomPinMessage,
}
pub struct TikTokWebsocketUnknownMessageEvent {
    pub message_name: String,
    pub raw_data: Message,
}
pub struct TikTokLinkmicBattleTaskEvent {
    pub raw_data: WebcastLinkmicBattleTaskMessage,
}
pub struct TikTokChatEvent {
    pub raw_data: WebcastChatMessage,
}
pub struct TikTokLinkMicArmiesEvent {
    pub raw_data: WebcastLinkMicArmies,
}
pub struct TikTokControlEvent {
    pub raw_data: WebcastControlMessage,
}
pub struct TikTokWebsocketResponseEvent {
    pub raw_data: WebcastResponse,
}
pub struct TikTokGiftEvent {
    pub raw_data: WebcastGiftMessage,
}
pub struct TikTokLinkMicBattlePunishFinishEvent {
    pub raw_data: WebcastLinkMicBattlePunishFinish,
}
pub struct TikTokHourlyRankEvent {
    pub raw_data: WebcastHourlyRankMessage,
}
pub struct TikTokLinkLayerEvent {
    pub raw_data: WebcastLinkLayerMessage,
}
pub struct TikTokEmoteChatEvent {
    pub raw_data: WebcastEmoteChatMessage,
}
pub struct TikTokRoomEvent {
    pub raw_data: WebcastRoomMessage,
}
pub struct TikTokInRoomBannerEvent {
    pub raw_data: WebcastInRoomBannerMessage,
}
pub struct TikTokConnectedEvent {}
pub struct TikTokLinkMicFanTicketMethodEvent {
    pub raw_data: WebcastLinkMicFanTicketMethod,
}
pub struct TikTokUnauthorizedMemberEvent {
    pub raw_data: WebcastUnauthorizedMemberMessage,
}
pub struct TikTokRoomVerifyEvent {
    pub raw_data: RoomVerifyMessage,
}
pub struct TikTokOecLiveShoppingEvent {
    pub raw_data: WebcastOecLiveShoppingMessage,
}
pub struct TikTokLiveIntroEvent {
    pub raw_data: WebcastLiveIntroMessage,
}
pub struct TikTokImDeleteEvent {
    pub raw_data: WebcastImDeleteMessage,
}
pub struct TikTokRankUpdateEvent {
    pub raw_data: WebcastRankUpdateMessage,
}
pub struct TikTokQuestionNewEvent {
    pub raw_data: WebcastQuestionNewMessage,
}
pub struct TikTokSystemEvent {
    pub raw_data: WebcastSystemMessage,
}
pub struct TikTokResponseEvent {
    pub raw_data: WebcastResponse,
}
pub struct TikTokMsgDetectEvent {
    pub raw_data: WebcastMsgDetectMessage,
}
pub struct TikTokLinkMicMethodEvent {
    pub raw_data: WebcastLinkMicMethod,
}
pub struct TikTokBarrageEvent {
    pub raw_data: WebcastBarrageMessage,
}
pub enum TikTokLiveEvent {
    OnLike(TikTokLikeEvent),
    OnQuestionNew(TikTokQuestionNewEvent),
    OnLinkMicBattlePunishFinish(TikTokLinkMicBattlePunishFinishEvent),
    OnRankUpdate(TikTokRankUpdateEvent),
    OnLinkMicFanTicketMethod(TikTokLinkMicFanTicketMethodEvent),
    OnLiveIntro(TikTokLiveIntroEvent),
    OnMember(TikTokMemberEvent),
    OnChat(TikTokChatEvent),
    OnLinkMicArmies(TikTokLinkMicArmiesEvent),
    OnLinkLayer(TikTokLinkLayerEvent),
    OnWebsocketResponse(TikTokWebsocketResponseEvent),
    OnResponse(TikTokResponseEvent),
    OnPushFrame(TikTokPushFrameEvent),
    OnRankText(TikTokRankTextEvent),
    OnSystem(TikTokSystemEvent),
    OnLinkmicBattleTask(TikTokLinkmicBattleTaskEvent),
    OnGift(TikTokGiftEvent),
    OnInRoomBanner(TikTokInRoomBannerEvent),
    OnMsgDetect(TikTokMsgDetectEvent),
    OnControl(TikTokControlEvent),
    OnLinkMicMethod(TikTokLinkMicMethodEvent),
    OnGoalUpdate(TikTokGoalUpdateEvent),
    OnCaption(TikTokCaptionEvent),
    OnHourlyRank(TikTokHourlyRankEvent),
    OnDisconnected(TikTokDisconnectedEvent),
    OnBarrage(TikTokBarrageEvent),
    OnSubNotify(TikTokSubNotifyEvent),
    OnRoomVerify(TikTokRoomVerifyEvent),
    OnSocial(TikTokSocialEvent),
    OnEmoteChat(TikTokEmoteChatEvent),
    OnPoll(TikTokPollEvent),
    OnRoomPin(TikTokRoomPinEvent),
    OnRoom(TikTokRoomEvent),
    OnEnvelope(TikTokEnvelopeEvent),
    OnWebsocketUnknownMessage(TikTokWebsocketUnknownMessageEvent),
    OnConnected(TikTokConnectedEvent),
    OnImDelete(TikTokImDeleteEvent),
    OnRoomUserSeq(TikTokRoomUserSeqEvent),
    OnUnauthorizedMember(TikTokUnauthorizedMemberEvent),
    OnOecLiveShopping(TikTokOecLiveShoppingEvent),
    OnLink(TikTokLinkEvent),
    OnLinkMicBattle(TikTokLinkMicBattleEvent),
}
