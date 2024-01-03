use std::cell::RefCell;
use crate::live_client::TikTokLiveClient;



pub struct TikTokLiveEvent
{
    pub created_at: i32,
    pub name: String,
}

pub type TikTokEventHandler = fn(client: &TikTokLiveClient, event: &TikTokLiveEvent);

#[derive(Clone)]
pub struct TikTokLiveEventObserver
{
    pub(crate) events : Vec<TikTokEventHandler>
}

impl TikTokLiveEventObserver
{
    pub fn new() -> Self
    {
        TikTokLiveEventObserver
        {
          events: vec![]
        }
    }

    pub fn attach(&mut self, handler: TikTokEventHandler)
    {
        self.events.push(handler);
    }

    pub fn publish(&self, client: &TikTokLiveClient, event: &TikTokLiveEvent)
    {
        for handler in &self.events
        {
            handler(client,event);
        }
    }

}
