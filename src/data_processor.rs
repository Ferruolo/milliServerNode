use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::internal_lang::KeyType;

pub struct DataManager<T> {
    map: HashMap<KeyType, Arc<Mutex<T>>>,
    prev_size: usize,
}

impl<T> DataManager<T> {
    pub fn new() -> Self {
        let initial_reserve: usize = 100; // Arbitrary Value

        let mut map: HashMap<KeyType, Arc<Mutex<T>>> = HashMap::new();
        map.reserve(initial_reserve);
        return Self {
            map,
            prev_size: initial_reserve,
        };
    }

    pub fn get_reference(&mut self, key: &KeyType) -> Option<Arc<Mutex<T>>> {
        match self.map.get_mut(key) {
            None => {
                None
            }
            Some(item) => {
                Some(Arc::clone(item))
            }
        }
    }

    pub fn insert(&mut self, key: &KeyType, datum: T) -> Option<Arc<Mutex<T>>> {
        if self.prev_size <= self.map.len() {
            self.map.reserve(self.prev_size * 2);
            self.prev_size = self.map.len();
        }

        self.map.insert(key, Arc::new(Mutex::new(datum)))
    }
}
