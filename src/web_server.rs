use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use crate::data_processor::DataManager;
use crate::internal_lang::{ImperativeOps, KeyType};
use crate::Job::Execute;
use crate::thread_manager::ThreadManager;

type FakeDatum = u8;


/*
* Mocking up real world functionality before I start getting into parsing/command structure
*/

fn run_fake_web_server(n_threads: usize) {
    let mut operations: VecDeque<ImperativeOps<FakeDatum>> = Default::default();
    let mut threadpool = ThreadManager::new(n_threads);
    let mut datastore: Arc<Mutex<DataManager<FakeDatum>>> = Arc::new(Mutex::new(DataManager::new()));

    'fakeCommandLoop: while let Some(next) = operations.pop_front() {
        match next {
            ImperativeOps::Get(k) => {
                let new_job = || {
                    let datum = get_datum(&datastore, k);
                    match datum {
                        None => {
                            println!("Item {} not found", k)
                        }
                        Some(d) => {
                            let x = d.lock().unwrap().clone();
                            println!("Key {} retrieved {}", k, x);
                        }
                    }
                };

                wrap_and_schedule(&mut threadpool, new_job);
            }
            ImperativeOps::Set(k, v) => {
                let new_job = || {
                    let datum = get_datum(&datastore, k);
                    match datum {
                        None => {
                            datastore.lock().unwrap().insert(k, v);
                        }
                        Some(d) => {
                            *d.lock().unwrap() = v;
                        }
                    }
                };
                wrap_and_schedule(&mut threadpool, new_job);
            }
            ImperativeOps::SHUTDOWN => {
                break 'fakeCommandLoop;
            }
        }
    }
}

fn wrap_and_schedule(threadpool: &mut ThreadManager, new_job: fn()) {
    let wrapped_job = Arc::new(Mutex::new(new_job));
    threadpool.schedule(Execute(wrapped_job));
}

#[inline] // Does this even do anything???
fn get_datum(mut datastore: &Arc<Mutex<DataManager<FakeDatum>>>, k: KeyType) -> Option<Arc<Mutex<FakeDatum>>> {
    let datum = {
        let mut db = datastore.lock().unwrap();
        let datum = db.get_reference(k);
        datum
    };
    datum
}



