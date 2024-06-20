use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use crate::data_processor::DataManager;
use crate::internal_lang::{FakeDatum, ImperativeOps, KeyType};
use crate::Job::Execute;
use crate::thread_manager::ThreadManager;


/*
* Mocking up real world functionality before I start getting into parsing/command structure
*/

pub fn run_fake_web_server(n_threads: usize, fake_data: Vec<FakeDatum>, mut fake_commands: Vec<ImperativeOps<FakeDatum>>) {
    let mut operations: VecDeque<ImperativeOps<FakeDatum>> = Default::default();

    //Please tell me there's an easier way to copy between the two
    copy_vec_to_deque(&mut fake_commands, &mut operations);

    let mut threadpool = ThreadManager::new(n_threads);
    let mut datastore: Arc<Mutex<DataManager<FakeDatum>>> = Arc::new(Mutex::new(DataManager::new()));

    for (i, d) in fake_data.iter().enumerate() {
        datastore.lock().unwrap().insert(i as KeyType, *d as FakeDatum);
    }



    'fakeCommandLoop: while let Some(cmd) = operations.pop_front() {
        if handle_command(&mut threadpool, &mut datastore, cmd) {
            break 'fakeCommandLoop;
        }
    }
}

fn copy_vec_to_deque(vec: &mut Vec<ImperativeOps<FakeDatum>>, deque: &mut VecDeque<ImperativeOps<FakeDatum>>) {
    while let Some(item) = vec.pop() {
        deque.push_front(item);
    }
}

fn handle_command(
    threadpool: &mut ThreadManager,
    datastore: &mut Arc<Mutex<DataManager<FakeDatum>>>,
    cmd: ImperativeOps<FakeDatum>)
    -> bool {
    let ds_clone = datastore.clone();

    match cmd {
        ImperativeOps::Get(k) => {

            let new_job = move || {
                let datum = get_datum(&ds_clone, k);
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
            let wrapped_job = Arc::new(new_job);
            threadpool.schedule(Execute(wrapped_job));
        }
        ImperativeOps::Set(k, v) => {
            let new_job = move || {
                let datum = get_datum(&ds_clone, k);
                match datum {
                    None => {
                        ds_clone.lock().unwrap().insert(k, v);
                    }
                    Some(d) => {
                        *d.lock().unwrap() = v;
                    }
                }
            };
            let wrapped_job = Arc::new(new_job);
            threadpool.schedule(Execute(wrapped_job));
        }
        ImperativeOps::SHUTDOWN => {
            return true;
        }
    }
    false
}

// TODO: Implement Wrap and Schedule (wrap jobs in ARC and schedule them)

fn get_datum(datastore: &Arc<Mutex<DataManager<FakeDatum>>>, k: KeyType) -> Option<Arc<Mutex<FakeDatum>>> {
    let datum = {
        let mut db = datastore.lock().unwrap();
        let datum = db.get_reference(k);
        datum
    };
    datum
}
