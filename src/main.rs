

use rand::{random, Rng, thread_rng};
use serverNode::internal_lang::{FakeDatum, ImperativeOps, KeyType};
use serverNode::internal_lang::ImperativeOps::{Get, Set};
use serverNode::web_server::run_fake_web_server;

// cargo run  846.61s user 8.23s system 99% cpu 14:19.16 total


const NUM_DATA_ENTRIES: u32 = 1000000;

const NUM_OPERATIONS: u32 = 1000000000;

const MAX_VAL: FakeDatum = 200;

const N_THREADS: usize = 8;

fn main() {
    let mut initial_data: Vec<FakeDatum> = vec![];
    for _ in 0..NUM_DATA_ENTRIES {
        let rand_num = thread_rng().gen_range(0..MAX_VAL);
        initial_data.push(rand_num as FakeDatum)
    }

    let mut commands_list: Vec<ImperativeOps<FakeDatum>> = vec![];

    for _ in 0..NUM_OPERATIONS {
        let key_num = thread_rng().gen_range(0..NUM_DATA_ENTRIES) as KeyType;
        commands_list.push({
            if random()
            {
                // add Setter
                let new_entry = thread_rng().gen_range(0..MAX_VAL) as FakeDatum;
                Set(key_num, new_entry)
            } else {
                // add Getter
                Get(key_num)
            }
        });
    }
    run_fake_web_server(N_THREADS, initial_data, commands_list);

    println!("Shutting down.");
}