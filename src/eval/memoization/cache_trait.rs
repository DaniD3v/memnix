use std::fmt::Debug;

use cached::{ConcurrentCached, DiskCache};
use serde::{Serialize, de::DeserializeOwned};

pub trait CacheBackend {
    // `Cache` can't be generic over a store `C: ConcurrentCached<K, V>`:
    // they have different `V`s, so no specific type `C` fits both.
    //
    // -> GAT that is generic over `V`
    type Store<K, V>: ConcurrentCached<K, V, Error: Debug>
    where
        K: ToString + Clone,
        V: Serialize + DeserializeOwned;
}

pub struct Disk;
impl CacheBackend for Disk {
    type Store<K, V>
        = DiskCache<K, V>
    where
        K: ToString + Clone,
        V: Serialize + DeserializeOwned;
}
