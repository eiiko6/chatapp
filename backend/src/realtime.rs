use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::routes::messages::Message;

pub type RoomId = i32;

#[derive(Clone)]
pub struct Realtime {
    pub rooms: Arc<DashMap<RoomId, broadcast::Sender<Message>>>,
}

impl Realtime {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(DashMap::new()),
        }
    }

    pub fn sender_for(&self, room: RoomId) -> broadcast::Sender<Message> {
        self.rooms
            .entry(room)
            .or_insert_with(|| broadcast::channel(100).0)
            .clone()
    }
}
