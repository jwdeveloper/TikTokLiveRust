use std::cell::RefCell;
use crate::live_client_events::{TikTokLiveEvent, TikTokLiveEventObserver};

pub struct TikTokLiveClient
{
    event_observer: TikTokLiveEventObserver,
}

impl TikTokLiveClient
{
    pub fn new(event_observer: TikTokLiveEventObserver) -> Self
    {
        TikTokLiveClient
        {
            event_observer
        }
    }

    pub fn connect(&self)
    {
        println!("connected!");

        let event = TikTokLiveEvent
        {
            name: "dupa".to_string(),
            created_at: 0,
        };
        self.event_observer.publish(&self, &event)
    }

    pub fn disconnect(&self) {
        todo!()
    }

    pub fn publish_event(&mut self, event: TikTokLiveEvent)
    {
        // self.event_observer.borrow_mut().publish(self, &event);
    }

    pub fn live_info(&self) {
        todo!()
    }
}