use std::collections::HashMap;
use std::mem::swap;
use crate::data_processor::DatumType::{KeyNotFound, OutForProcessing};

enum DatumType <T> {
    OutForProcessing,
    Available(T),
    KeyNotFound
}

struct DataStore <T> {
    map: HashMap<u64, DatumType<T>>
}

impl <T> DataStore<T> {
    fn get(&mut self, key: u64) -> DatumType<T> {
        return match self.map.get_mut(&key) {
            None => {
                KeyNotFound
            }
            Some(x) => {
                let mut swapper = OutForProcessing;
                swap(&mut swapper, x);
                swapper
            }
        }
    }

    fn set(&mut self, key: u64, val: &T) -> Option<&T> {
        return match self.map.get_mut(&key) {
            None => {
                None
            }
            Some(x) => {
                match x {
                    DatumType::Available(x) => {
                        *x = val;
                        return Some(x);
                    }
                    _ => None
                }
            }
        }
    }

    fn insert(&mut self, key:u64, val: &T) {
        return self.insert(key, val);
    }

}
