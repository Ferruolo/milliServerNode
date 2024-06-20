/*
* GOAL: Internal implementation of the simply typed lambda calculus, with extensions
* Will be extended, optimized to work with database
*/

/*
* CURRENT STATUS:
* Basic Read, write commands
*/

use std::sync::{Arc, Mutex};
use crate::data_processor::DataManager;

pub type FakeDatum = u64;

pub type KeyType = usize;

pub enum ImperativeOps<T> {
    Get(KeyType),
    Set(KeyType, T),
}


pub union OperationsLang<T> {
    ops: ImperativeOps<T>
}


/*
* Executes task defined by Internal Lang. Returns true if successful, false otherwise
*/
pub fn execute <T> (task: OperationsLang<T>, db: &mut Arc<Mutex<DataManager<T>>>) -> bool {
    return match task {
        ImperativeOps::Get(key) => {
            let object = db.lock().unwrap().get_reference(&key);
            match object {
                None => {
                    println!("Item {key} not found");
                    false
                }
                Some(d) => {
                    let data = *(d.lock().unwrap());
                    println!("Key {key} returned {data}");
                    true
                }
            }
        }
        ImperativeOps::Set(key, value) => {
            let object = db.lock().unwrap().get_reference(&key);
            match object {
                None => {
                    db.lock().unwrap().insert(&key, value);
                    println!("Added {key} of value {value} to db");
                    true
                }
                Some(d) => {
                    *(d.lock().unwrap()) = value;
                    println!("Key {key} set to {data}");
                    true
                }
            }
        }
    }
}