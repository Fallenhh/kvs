use std::collections::HashMap;

pub struct KvStore{
    hashmap: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            hashmap: HashMap::new(),
        }
    }

    pub fn set(&mut self, k: String, v: String) {
        self.hashmap.insert(k, v);
    }

    pub fn remove(&mut self, k: String) {
        self.hashmap.remove(&k);
    }

    pub fn get(&self, k:String) -> Option<String> {
        self.hashmap.get(&k).cloned()
    }
}