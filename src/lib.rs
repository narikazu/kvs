use std::collections::HashMap;

pub struct KvStore {
    hash_map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            hash_map: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.hash_map.get(&key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.hash_map.insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        self.hash_map.remove(&key);
    }
}
