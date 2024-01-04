

pub struct TikTokGiftEvent
{
    pub gift_id: i32,
    pub gift_name: String
}

pub struct TikTokChestEvent
{
    pub gift_id: i32,
    pub gift_name: String
}

pub enum TikTokLiveEvent
{
    onGift(TikTokGiftEvent),
    onChest(TikTokChestEvent),
}

