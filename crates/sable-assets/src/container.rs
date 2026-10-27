use std::{ops::Deref, sync::Arc};

use parking_lot::RwLock;
use slotmap::{Key, SlotMap};

/// Generic container for holding assets
pub struct Container<K: Key, T> {
    storage: RwLock<Storage<K, T>>,
}

impl<K: Key, T> Container<K, T> {
    pub fn new() -> Self {
        Self {
            storage: RwLock::new(Storage::default()),
        }
    }
}

impl<K: Key, T> Container<K, T> {
    pub fn add(&self, asset: impl Into<T>) -> K {
        let mut storage = self.storage.write();
        storage.contents.insert(Item::new(asset.into()))
    }

    pub fn get(&self, key: K) -> Option<Arc<T>> {
        let storage = self.storage.read();
        storage.contents.get(key).map(|value| value.asset.clone())
    }
}

/// Inner storage for generic container
struct Storage<K: Key, T> {
    contents: SlotMap<K, Item<T>>,
}

impl<K: Key, V> Default for Storage<K, V> {
    fn default() -> Self {
        Self {
            contents: SlotMap::default(),
        }
    }
}

struct Item<T> {
    asset: Arc<T>,
}

impl<T> Item<T> {
    fn new(asset: T) -> Self {
        Self {
            asset: Arc::new(asset),
        }
    }
}

impl<T> Deref for Item<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.asset
    }
}
