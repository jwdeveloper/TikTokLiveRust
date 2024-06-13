use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientData {
    pub data: Data,
    pub extra: Extra2,
    pub status_code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "AnchorABMap")]
    pub anchor_abmap: AnchorAbmap,
    pub adjust_display_order: i64,
    pub admin_ec_show_permission: AnchorAbmap,
    pub admin_user_ids: Vec<Value>,
    pub age_restricted: AgeRestricted,
    pub allow_preview_time: i64,
    pub anchor_scheduled_time_text: String,
    pub anchor_share_text: String,
    pub anchor_tab_type: i64,
    pub answering_question_content: String,
    pub app_id: i64,
    pub audio_mute: i64,
    pub auto_cover: i64,
    pub book_end_time: i64,
    pub book_time: i64,
    pub business_live: i64,
    pub challenge_info: String,
    pub client_version: i64,
    pub comment_has_text_emoji_emote: i64,
    pub comment_name_mode: i64,
    pub commerce_info: CommerceInfo,
    pub common_label_list: String,
    pub content_tag: String,
    pub cover: Cover,
    pub cpp_version: i64,
    pub create_time: i64,
    pub deco_list: Vec<Value>,
    pub deprecated10: String,
    pub deprecated11: String,
    pub deprecated12: String,
    pub deprecated13: String,
    pub deprecated14: i64,
    pub deprecated15: i64,
    pub deprecated16: i64,
    pub deprecated17: Vec<Value>,
    pub deprecated18: i64,
    pub deprecated19: String,
    pub deprecated195: bool,
    pub deprecated2: String,
    pub deprecated20: i64,
    pub deprecated21: bool,
    pub deprecated22: i64,
    pub deprecated23: String,
    pub deprecated24: i64,
    pub deprecated26: String,
    pub deprecated28: String,
    pub deprecated3: AnchorAbmap,
    pub deprecated30: String,
    pub deprecated31: bool,
    pub deprecated32: String,
    pub deprecated35: i64,
    pub deprecated36: i64,
    pub deprecated39: String,
    pub deprecated4: i64,
    pub deprecated41: i64,
    pub deprecated43: bool,
    pub deprecated44: i64,
    pub deprecated5: bool,
    pub deprecated6: String,
    pub deprecated7: i64,
    pub deprecated8: String,
    pub deprecated9: String,
    pub disable_preload_stream: bool,
    pub disable_preview_sub_only: i64,
    pub drawer_tab_position: String,
    pub drop_comment_group: i64,
    pub effect_info: Vec<Value>,
    pub enable_server_drop: i64,
    pub existed_commerce_goods: bool,
    pub fansclub_msg_style: i64,
    pub feed_room_label: Cover,
    pub feed_room_labels: Vec<Value>,
    pub filter_msg_rules: Vec<Value>,
    pub finish_reason: i64,
    pub finish_time: i64,
    pub finish_url: String,
    pub finish_url_v2: String,
    pub follow_msg_style: i64,
    pub forum_extra_data: String,
    pub game_demo: i64,
    pub game_tag: Vec<GameTag>,
    pub gift_msg_style: i64,
    pub gift_poll_vote_enabled: bool,
    pub group_source: i64,
    pub has_commerce_goods: bool,
    pub has_more_history_comment: bool,
    pub has_used_music: bool,
    pub hashtag: Option<Hashtag>,
    pub have_wishlist: bool,
    pub history_comment_cursor: String,
    pub history_comment_list: Vec<Value>,
    pub hot_sentence_info: String,
    pub id: i64,
    pub id_str: String,
    pub idc_region: String,
    pub indicators: Vec<Value>,
    pub interaction_question_version: i64,
    pub introduction: String,
    pub is_gated_room: bool,
    pub is_replay: bool,
    pub is_show_user_card_switch: bool,
    pub last_ping_time: i64,
    pub layout: i64,
    pub like_count: i64,
    pub link_mic: LinkMic,
    pub linker_map: AnchorAbmap,
    pub linkmic_layout: i64,
    pub lite_user_not_visible: bool,
    pub lite_user_visible: bool,
    pub live_distribution: Vec<Value>,
    pub live_id: i64,
    pub live_reason: String,
    pub live_room_mode: i64,
    pub live_sub_only: i64,
    pub live_sub_only_use_music: i64,
    pub live_type_audio: bool,
    pub live_type_linkmic: bool,
    pub live_type_normal: bool,
    pub live_type_sandbox: bool,
    pub live_type_screenshot: bool,
    pub live_type_social_live: bool,
    pub live_type_third_party: bool,
    pub living_room_attrs: LivingRoomAttrs,
    pub lottery_finish_time: i64,
    pub max_preview_time: i64,
    pub mosaic_status: i64,
    pub multi_stream_id: i64,
    pub multi_stream_id_str: String,
    pub multi_stream_scene: i64,
    pub multi_stream_source: i64,
    pub net_mode: i64,
    pub os_type: i64,
    pub owner: Owner,
    pub owner_device_id: i64,
    pub owner_device_id_str: String,
    pub owner_user_id: i64,
    pub owner_user_id_str: String,
    pub paid_event: PaidEvent,
    pub pico_live_type: i64,
    pub polling_star_comment: bool,
    pub pre_enter_time: i64,
    pub preview_flow_tag: i64,
    pub quota_config: AnchorAbmap,
    pub rank_comment_groups: Vec<Value>,
    pub ranklist_audience_type: i64,
    pub regional_restricted: RegionalRestricted,
    pub relation_tag: String,
    pub replay: bool,
    pub room_audit_status: i64,
    pub room_auth: RoomAuth,
    pub room_create_ab_param: String,
    pub room_layout: i64,
    pub room_pcu: i64,
    pub room_sticker_list: Vec<Value>,
    pub room_tabs: Vec<Value>,
    pub room_tag: i64,
    pub rtc_app_id: String,
    pub scroll_config: String,
    pub search_id: i64,
    pub share_msg_style: i64,
    pub share_url: String,
    pub short_title: String,
    pub short_touch_items: Vec<Value>,
    pub show_star_comment_entrance: bool,
    pub social_interaction: SocialInteraction,
    pub start_time: i64,
    pub stats: Stats,
    pub status: i64,
    pub sticker_list: Vec<Value>,
    pub stream_id: i64,
    pub stream_id_str: String,
    pub stream_status: i64,
    pub stream_url: StreamUrl,
    pub stream_url_filtered_info: StreamUrlFilteredInfo,
    pub support_quiz: i64,
    pub title: String,
    pub top_fans: Vec<TopFan>,
    pub use_filter: bool,
    pub user_count: i64,
    pub user_share_text: String,
    pub video_feed_tag: String,
    pub webcast_comment_tcs: i64,
    pub webcast_sdk_version: i64,
    pub with_draw_something: bool,
    pub with_ktv: bool,
    pub with_linkmic: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnchorAbmap {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgeRestricted {
    #[serde(rename = "AgeInterval")]
    pub age_interval: i64,
    pub restricted: bool,
    pub source: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommerceInfo {
    pub commerce_permission: i64,
    pub oec_live_enter_room_init_data: String,
    pub product_num: i64,
    pub use_async_load: bool,
    pub use_new_promotion: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cover {
    pub avg_color: String,
    pub height: i64,
    pub image_type: i64,
    pub is_animated: bool,
    pub open_web_url: String,
    pub uri: String,
    pub url_list: Vec<String>,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameTag {
    pub bundle_id: String,
    pub full_name: String,
    pub game_category: Vec<Value>,
    pub hashtag_id: Vec<i64>,
    pub hashtag_list: Vec<Value>,
    pub id: i64,
    pub is_new_game: bool,
    pub landscape: i64,
    pub package_name: String,
    pub short_name: String,
    pub show_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hashtag {
    pub id: i64,
    pub image: Cover,
    pub namespace: i64,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkMic {
    pub audience_id_list: Vec<Value>,
    pub battle_scores: Vec<Value>,
    pub battle_settings: BattleSettings,
    pub channel_id: i64,
    pub followed_count: i64,
    pub linked_user_list: Vec<Value>,
    pub multi_live_enum: i64,
    pub rival_anchor_id: i64,
    pub show_user_list: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BattleSettings {
    pub battle_id: i64,
    pub channel_id: i64,
    pub duration: i64,
    pub finished: i64,
    pub match_type: i64,
    pub start_time: i64,
    pub start_time_ms: i64,
    pub theme: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LivingRoomAttrs {
    pub admin_flag: i64,
    pub rank: i64,
    pub room_id: i64,
    pub room_id_str: String,
    pub silence_flag: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    pub allow_find_by_contacts: bool,
    pub allow_others_download_video: bool,
    pub allow_others_download_when_sharing_video: bool,
    pub allow_share_show_profile: bool,
    pub allow_show_in_gossip: bool,
    pub allow_show_my_action: bool,
    pub allow_strange_comment: bool,
    pub allow_unfollower_comment: bool,
    pub allow_use_linkmic: bool,
    pub avatar_large: Cover,
    pub avatar_medium: Cover,
    pub avatar_thumb: Cover,
    pub badge_image_list: Vec<Value>,
    pub badge_list: Vec<BadgeList>,
    pub bg_img_url: String,
    pub bio_description: String,
    pub block_status: i64,
    pub border_list: Vec<Value>,
    pub comment_restrict: i64,
    pub commerce_webcast_config_ids: Vec<Value>,
    pub constellation: String,
    pub create_time: i64,
    pub deprecated1: i64,
    pub deprecated12: i64,
    pub deprecated13: i64,
    pub deprecated15: i64,
    pub deprecated16: bool,
    pub deprecated17: bool,
    pub deprecated18: String,
    pub deprecated19: bool,
    pub deprecated2: i64,
    pub deprecated21: i64,
    pub deprecated28: bool,
    pub deprecated29: String,
    pub deprecated3: i64,
    pub deprecated4: i64,
    pub deprecated5: String,
    pub deprecated6: i64,
    pub deprecated7: String,
    pub deprecated8: i64,
    pub disable_ichat: i64,
    pub display_id: String,
    pub enable_ichat_img: i64,
    pub exp: i64,
    pub fan_ticket_count: i64,
    pub fold_stranger_chat: bool,
    pub follow_info: FollowInfo,
    pub follow_status: i64,
    pub ichat_restrict_type: i64,
    pub id: i64,
    pub id_str: String,
    pub is_block: bool,
    pub is_follower: bool,
    pub is_following: bool,
    pub is_subscribe: bool,
    pub link_mic_stats: i64,
    pub media_badge_image_list: Vec<Value>,
    pub mint_type_label: Vec<i64>,
    pub modify_time: i64,
    pub need_profile_guide: bool,
    pub new_real_time_icons: Vec<Value>,
    pub nickname: String,
    pub own_room: OwnRoom,
    pub pay_grade: PayGrade,
    pub pay_score: i64,
    pub pay_scores: i64,
    pub push_comment_status: bool,
    pub push_digg: bool,
    pub push_follow: bool,
    pub push_friend_action: bool,
    pub push_ichat: bool,
    pub push_status: bool,
    pub push_video_post: bool,
    pub push_video_recommend: bool,
    pub real_time_icons: Vec<Value>,
    pub scm_label: String,
    pub sec_uid: String,
    pub secret: i64,
    pub share_qrcode_uri: String,
    pub special_id: String,
    pub status: i64,
    pub ticket_count: i64,
    pub top_fans: Vec<Value>,
    pub top_vip_no: i64,
    pub upcoming_event_list: Vec<Value>,
    pub user_attr: UserAttr,
    pub user_role: i64,
    pub verified: bool,
    pub verified_content: String,
    pub verified_reason: String,
    pub with_car_management_permission: bool,
    pub with_commerce_permission: bool,
    pub with_fusion_shop_entry: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BadgeList {
    #[serde(rename = "OpenWebURL")]
    pub open_web_url: String,
    pub combine: Option<Combine>,
    pub display: bool,
    pub display_status: i64,
    pub display_type: i64,
    pub exhibition_type: i64,
    pub greyed_by_client: i64,
    pub is_customized: bool,
    pub position: i64,
    pub priority_type: i64,
    pub privilege_log_extra: PrivilegeLogExtra,
    pub scene_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Combine {
    pub background: Background,
    pub background_auto_mirrored: bool,
    pub background_dark_mode: Background,
    pub display_type: i64,
    pub font_style: Option<FontStyle>,
    pub icon: Cover,
    pub icon_auto_mirrored: bool,
    pub multi_guest_show_style: i64,
    pub padding: Option<Padding>,
    pub padding_new_font: Option<Padding>,
    pub personal_card_show_style: i64,
    pub profile_card_panel: Option<ProfileCardPanel>,
    pub public_screen_show_style: i64,
    pub ranklist_online_audience_show_style: i64,
    pub str: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Background {
    pub background_color_code: String,
    pub border_color_code: String,
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub avg_color: String,
    pub height: i64,
    pub image_type: i64,
    pub is_animated: bool,
    pub open_web_url: String,
    pub uri: String,
    pub url_list: Vec<Value>,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontStyle {
    pub border_color: String,
    pub font_color: String,
    pub font_size: i64,
    pub font_width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Padding {
    pub badge_width: i64,
    pub horizontal_padding_rule: i64,
    pub icon_bottom_padding: i64,
    pub icon_top_padding: i64,
    pub left_padding: i64,
    pub middle_padding: i64,
    pub right_padding: i64,
    pub use_specific: bool,
    pub vertical_padding_rule: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileCardPanel {
    pub badge_text_position: i64,
    pub profile_content: ProfileContent,
    pub projection_config: ProjectionConfig,
    pub use_new_profile_card_style: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileContent {
    pub icon_list: Vec<Value>,
    pub use_content: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectionConfig {
    pub icon: Image,
    pub use_projection: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivilegeLogExtra {
    pub data_version: String,
    pub level: String,
    pub privilege_id: String,
    pub privilege_order_id: String,
    pub privilege_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowInfo {
    pub follow_status: i64,
    pub follower_count: i64,
    pub following_count: i64,
    pub push_status: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OwnRoom {
    pub room_ids: Vec<i64>,
    pub room_ids_str: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayGrade {
    pub deprecated20: i64,
    pub deprecated22: i64,
    pub deprecated23: i64,
    pub deprecated24: i64,
    pub deprecated25: i64,
    pub deprecated26: i64,
    pub grade_banner: String,
    pub grade_describe: String,
    pub grade_icon_list: Vec<Value>,
    pub level: i64,
    pub name: String,
    pub next_name: String,
    pub next_privileges: String,
    pub score: i64,
    pub screen_chat_type: i64,
    pub upgrade_need_consume: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserAttr {
    pub admin_permissions: AnchorAbmap,
    pub is_admin: bool,
    pub is_muted: bool,
    pub is_super_admin: bool,
    pub mute_duration: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaidEvent {
    pub event_id: i64,
    pub paid_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionalRestricted {
    pub block_list: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomAuth {
    #[serde(rename = "Banner")]
    pub banner: i64,
    #[serde(rename = "BroadcastMessage")]
    pub broadcast_message: i64,
    #[serde(rename = "Chat")]
    pub chat: bool,
    #[serde(rename = "ChatL2")]
    pub chat_l2: bool,
    #[serde(rename = "ChatSubOnly")]
    pub chat_sub_only: bool,
    #[serde(rename = "CommercePermission")]
    pub commerce_permission: i64,
    #[serde(rename = "CommunityFlagged")]
    pub community_flagged: bool,
    #[serde(rename = "CommunityFlaggedReview")]
    pub community_flagged_review: bool,
    #[serde(rename = "CustomizableGiftPoll")]
    pub customizable_gift_poll: i64,
    #[serde(rename = "CustomizablePoll")]
    pub customizable_poll: i64,
    #[serde(rename = "Danmaku")]
    pub danmaku: bool,
    #[serde(rename = "Digg")]
    pub digg: bool,
    #[serde(rename = "DonationSticker")]
    pub donation_sticker: i64,
    #[serde(rename = "EnableFansLevel")]
    pub enable_fans_level: bool,
    #[serde(rename = "EventPromotion")]
    pub event_promotion: i64,
    #[serde(rename = "Explore")]
    pub explore: bool,
    #[serde(rename = "GameRankingSwitch")]
    pub game_ranking_switch: i64,
    #[serde(rename = "Gift")]
    pub gift: bool,
    #[serde(rename = "GiftAnchorMt")]
    pub gift_anchor_mt: i64,
    #[serde(rename = "GiftPoll")]
    pub gift_poll: i64,
    #[serde(rename = "GoldenEnvelope")]
    pub golden_envelope: i64,
    #[serde(rename = "GoldenEnvelopeActivity")]
    pub golden_envelope_activity: i64,
    #[serde(rename = "InteractionQuestion")]
    pub interaction_question: bool,
    #[serde(rename = "Landscape")]
    pub landscape: i64,
    #[serde(rename = "LandscapeChat")]
    pub landscape_chat: i64,
    #[serde(rename = "LuckMoney")]
    pub luck_money: bool,
    #[serde(rename = "MultiEnableReserve")]
    pub multi_enable_reserve: bool,
    #[serde(rename = "Pictionary")]
    pub pictionary: i64,
    #[serde(rename = "PictionaryBubble")]
    pub pictionary_bubble: i64,
    #[serde(rename = "PictionaryPermission")]
    pub pictionary_permission: i64,
    #[serde(rename = "Poll")]
    pub poll: i64,
    #[serde(rename = "Promote")]
    pub promote: bool,
    #[serde(rename = "PromoteOther")]
    pub promote_other: i64,
    #[serde(rename = "Props")]
    pub props: bool,
    #[serde(rename = "PublicScreen")]
    pub public_screen: i64,
    #[serde(rename = "QuickChat")]
    pub quick_chat: i64,
    #[serde(rename = "Rank")]
    pub rank: i64,
    #[serde(rename = "RankingChangeAlterSwitch")]
    pub ranking_change_alter_switch: i64,
    #[serde(rename = "RoomContributor")]
    pub room_contributor: bool,
    #[serde(rename = "SecretRoom")]
    pub secret_room: i64,
    #[serde(rename = "Share")]
    pub share: bool,
    #[serde(rename = "ShareEffect")]
    pub share_effect: i64,
    #[serde(rename = "ShoppingRanking")]
    pub shopping_ranking: i64,
    #[serde(rename = "SpamComments")]
    pub spam_comments: bool,
    #[serde(rename = "UserCard")]
    pub user_card: bool,
    #[serde(rename = "UserCount")]
    pub user_count: i64,
    #[serde(rename = "Viewers")]
    pub viewers: bool,
    pub comment_tray_status: i64,
    pub credit_entrance_for_audience: bool,
    pub deprecated1: bool,
    pub deprecated118: Vec<Value>,
    pub deprecated119: i64,
    pub deprecated2: i64,
    pub deprecated3: i64,
    pub deprecated4: i64,
    pub deprecated5: i64,
    pub deprecated6: i64,
    pub deprecated7: i64,
    pub deprecated8: i64,
    pub deprecated9: i64,
    pub game_guess_permission: bool,
    pub guess_entrance_for_host: bool,
    pub show_credit_widget: bool,
    pub transaction_history: i64,
    pub use_user_pv: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialInteraction {
    pub linkmic_scene_linker: AnchorAbmap,
    pub multi_live: MultiLive,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiLive {
    pub audience_shared_invitee_panel_type: i64,
    pub host_gifter_linkmic_enum: i64,
    pub host_multi_guest_dev_mode: i64,
    pub linkmic_service_version: i64,
    pub try_open_multi_guest_when_create_room: bool,
    pub user_settings: UserSettings,
    pub viewer_gifter_linkmic_enum: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSettings {
    pub applier_sort_gift_score_threshold: i64,
    pub applier_sort_setting: i64,
    pub multi_live_apply_permission: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub deprecated1: i64,
    pub deprecated2: String,
    pub digg_count: i64,
    pub enter_count: i64,
    pub fan_ticket: i64,
    pub follow_count: i64,
    pub gift_uv_count: i64,
    pub id: i64,
    pub id_str: String,
    pub like_count: i64,
    pub replay_fan_ticket: i64,
    pub replay_viewers: i64,
    pub share_count: i64,
    pub total_user: i64,
    pub total_user_desp: String,
    pub watermelon: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamUrl {
    pub candidate_resolution: Vec<String>,
    pub complete_push_urls: Vec<Value>,
    pub default_resolution: String,
    pub extra: Extra,
    pub flv_pull_url: Option<FlvPullUrl>,
    pub flv_pull_url_params: FlvPullUrl,
    pub hls_pull_url: String,
    pub hls_pull_url_map: AnchorAbmap,
    pub hls_pull_url_params: String,
    pub id: i64,
    pub id_str: String,
    pub live_core_sdk_data: Option<LiveCoreSdkData>,
    pub provider: i64,
    pub push_resolution: String,
    pub push_urls: Vec<Value>,
    pub resolution_name: ResolutionName,
    pub rtmp_pull_url: String,
    pub rtmp_pull_url_params: String,
    pub rtmp_push_url: String,
    pub rtmp_push_url_params: String,
    pub stream_app_id: i64,
    pub stream_control_type: i64,
    pub stream_delay_ms: i64,
    pub vr_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extra {
    pub anchor_interact_profile: i64,
    pub audience_interact_profile: i64,
    pub bframe_enable: bool,
    pub bitrate_adapt_strategy: i64,
    pub bytevc1_enable: bool,
    pub default_bitrate: i64,
    pub deprecated1: bool,
    pub fps: i64,
    pub gop_sec: i64,
    pub hardware_encode: bool,
    pub height: i64,
    pub max_bitrate: i64,
    pub min_bitrate: i64,
    pub roi: bool,
    pub sw_roi: bool,
    pub video_profile: i64,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlvPullUrl {
    #[serde(rename = "HD1")]
    pub hd1: Option<String>,
    #[serde(rename = "SD1")]
    pub sd1: Option<String>,
    #[serde(rename = "SD2")]
    pub sd2: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiveCoreSdkData {
    pub pull_data: PullData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullData {
    pub options: Options,
    pub stream_data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Options {
    pub default_quality: DefaultQuality,
    pub qualities: Vec<DefaultQuality>,
    pub show_quality_button: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefaultQuality {
    pub icon_type: i64,
    pub level: i64,
    pub name: String,
    pub resolution: String,
    pub sdk_key: String,
    pub v_codec: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolutionName {
    #[serde(rename = "AUTO")]
    pub auto: Option<String>,
    #[serde(rename = "FULL_HD1")]
    pub full_hd1: Option<String>,
    #[serde(rename = "HD1")]
    pub hd1: Option<String>,
    #[serde(rename = "ORIGION")]
    pub origion: Option<String>,
    #[serde(rename = "SD1")]
    pub sd1: Option<String>,
    #[serde(rename = "SD2")]
    pub sd2: Option<String>,
    pub pm_mt_video_1080p60: Option<String>,
    pub pm_mt_video_720p60: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamUrlFilteredInfo {
    pub is_gated_room: bool,
    pub is_paid_event: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopFan {
    pub fan_ticket: i64,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub allow_find_by_contacts: bool,
    pub allow_others_download_video: bool,
    pub allow_others_download_when_sharing_video: bool,
    pub allow_share_show_profile: bool,
    pub allow_show_in_gossip: bool,
    pub allow_show_my_action: bool,
    pub allow_strange_comment: bool,
    pub allow_unfollower_comment: bool,
    pub allow_use_linkmic: bool,
    pub avatar_large: Cover,
    pub avatar_medium: Cover,
    pub avatar_thumb: Cover,
    pub badge_image_list: Vec<Value>,
    pub badge_list: Vec<BadgeList>,
    pub bg_img_url: String,
    pub bio_description: String,
    pub block_status: i64,
    pub border_list: Vec<Value>,
    pub comment_restrict: i64,
    pub commerce_webcast_config_ids: Vec<Value>,
    pub constellation: String,
    pub create_time: i64,
    pub deprecated1: i64,
    pub deprecated12: i64,
    pub deprecated13: i64,
    pub deprecated15: i64,
    pub deprecated16: bool,
    pub deprecated17: bool,
    pub deprecated18: String,
    pub deprecated19: bool,
    pub deprecated2: i64,
    pub deprecated21: i64,
    pub deprecated28: bool,
    pub deprecated29: String,
    pub deprecated3: i64,
    pub deprecated4: i64,
    pub deprecated5: String,
    pub deprecated6: i64,
    pub deprecated7: String,
    pub deprecated8: i64,
    pub disable_ichat: i64,
    pub display_id: String,
    pub enable_ichat_img: i64,
    pub exp: i64,
    pub fan_ticket_count: i64,
    pub fold_stranger_chat: bool,
    pub follow_info: FollowInfo,
    pub follow_status: i64,
    pub ichat_restrict_type: i64,
    pub id: i64,
    pub id_str: String,
    pub is_block: bool,
    pub is_follower: bool,
    pub is_following: bool,
    pub is_subscribe: bool,
    pub link_mic_stats: i64,
    pub media_badge_image_list: Vec<Value>,
    pub mint_type_label: Vec<i64>,
    pub modify_time: i64,
    pub need_profile_guide: bool,
    pub new_real_time_icons: Vec<Value>,
    pub nickname: String,
    pub pay_grade: PayGrade,
    pub pay_score: i64,
    pub pay_scores: i64,
    pub push_comment_status: bool,
    pub push_digg: bool,
    pub push_follow: bool,
    pub push_friend_action: bool,
    pub push_ichat: bool,
    pub push_status: bool,
    pub push_video_post: bool,
    pub push_video_recommend: bool,
    pub real_time_icons: Vec<Value>,
    pub scm_label: String,
    pub sec_uid: String,
    pub secret: i64,
    pub share_qrcode_uri: String,
    pub special_id: String,
    pub status: i64,
    pub ticket_count: i64,
    pub top_fans: Vec<Value>,
    pub top_vip_no: i64,
    pub upcoming_event_list: Vec<Value>,
    pub user_attr: UserAttr,
    pub user_role: i64,
    pub verified: bool,
    pub verified_content: String,
    pub verified_reason: String,
    pub with_car_management_permission: bool,
    pub with_commerce_permission: bool,
    pub with_fusion_shop_entry: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extra2 {
    pub now: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamData {
    pub common: Common,
    pub data: NestedData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Common {
    pub peer_anchor_level: i64,
    pub session_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NestedData {
    pub ao: Option<Ao>,
    pub hd: Option<Ao>,
    pub ld: Option<Ao>,
    pub origin: Option<Ao>,
    pub sd: Option<Ao>,
    pub uhd: Option<Ao>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ao {
    pub main: Main,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Main {
    pub cmaf: String,
    pub dash: String,
    pub flv: String,
    pub hls: String,
    pub lls: String,
    pub tile: String,
    pub tsl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Demotion {
    #[serde(rename = "StallCount")]
    pub stall_count: i64,
}

#[derive(Clone)]
pub struct TikTokLiveSettings {
    pub host_name: String,
    pub language: String,
    pub reconnect_on_fail: bool,
    pub print_logs: bool,
    pub http_data: HttpData,
}

#[derive(Clone, Default)]
pub struct HttpData {
    pub time_out: Duration,
    pub params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
}

#[derive(Default)]
pub struct TikTokLiveInfo {
    pub room_id: String,
    pub client_data: String,
    pub likes: i32,
    pub viewers: i32,
    pub total_viewers: i32,
    pub host_name: String,
    pub title: String,
    pub language: String,
    pub connection_state: Mutex<ConnectionState>,
}

#[derive(PartialEq, Debug)]
pub enum ConnectionState {
    CONNECTING,
    CONNECTED,
    DISCONNECTED,
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::DISCONNECTED
    }
}
