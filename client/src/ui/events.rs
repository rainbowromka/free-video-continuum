use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

pub type UiId = u64;

use std::sync::atomic::{AtomicU64};

pub static NEXT_UI_ID: AtomicU64 = AtomicU64::new(0);

pub struct EventRegion {
    pub rect: (f32, f32, f32, f32),
    pub z: u32,
    pub handler: Box<dyn FnMut() + Send + 'static>,
}

impl EventRegion {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        let (rx, ry, rw, rh) = self.rect;
        x >= rx && x <= rx + rw && y >= ry && y <= ry + rh
    }
}

lazy_static! {
    pub static ref EVENT_REGISTRY: Mutex<HashMap<UiId, Vec<EventRegion>>> =
        Mutex::new(HashMap::new());
}