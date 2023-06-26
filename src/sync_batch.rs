use std::sync::Mutex;

use bevy::prelude::Resource;

/// A synchronized buffer of `T` items that only supports extending and draining.
#[derive(Resource)]
pub struct SyncBatch<T> {
    items: Mutex<Vec<T>>,
}

impl<T> SyncBatch<T> {
    pub fn extend(&self, commands: impl Iterator<Item = T>) {
        let mut items = self.items.lock().unwrap();
        items.extend(commands);
    }

    pub fn take_all(&self) -> Vec<T> {
        let mut items = self.items.lock().unwrap();
        let taken = items.drain(..).collect();
        taken
    }
}

impl<T> Default for SyncBatch<T> {
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
}
