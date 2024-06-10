use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use crate::{executor, Job};
use crate::thread_manager::ThreadSignal::*;

// TODO: Make this a functional language

enum ThreadSignal {
    Task(Job),
    TaskComplete(usize),
    Available(usize),
    Null,
    Kill,
    ErrorInstruction
}


struct ThreadManager {
    num_threads: usize,
    task_queue: VecDeque<Job>,
    worker_queue: VecDeque<Sender<ThreadSignal>>,
    threads: Vec<Worker>,
}


struct Worker {
    id: usize,
    join_handle: JoinHandle<()>,
    sender: Sender<ThreadSignal>,
    receiver: Receiver<ThreadSignal>,
}


impl Worker {
    fn new(id: usize, return_address: &Sender<ThreadSignal>) -> (Self) {
        let ret = return_address.clone();
        let (tx, rx): (Sender<ThreadSignal>, Receiver<ThreadSignal>) = mpsc::channel();
        let self_id = id;
        let t = thread::spawn(move || {
            ret.send(Available(self_id)).unwrap();
            'threadLoop: loop {
                match rx.recv().unwrap_or(Null) {
                    Task(job) => {
                        executor(job);
                        ret.send(TaskComplete(self_id)).unwrap();
                    }
                    Kill => {
                        break 'threadLoop;
                    }
                    Null => {
                        continue;
                    }
                    _ => {
                        ret.send(ErrorInstruction).unwrap();
                    }
                }
            }
        });

        return Self {
            id,
            join_handle: t,
            sender: tx,
            receiver: rx,
        };
    }
}

impl ThreadManager {
    fn new(n_threads: usize) -> ThreadManager {
        let mut threads = Vec::new();
        let (local_send, local_recv) = mpsc::channel();
        for i in 0..n_threads {
            threads.push(Worker::new(i, &local_send));
        }

        return Self {
            num_threads: n_threads,
            task_queue: Default::default(),
            worker_queue: Default::default(),
            threads,
        }
    }

    fn schedule(job: Job) {

    }
}
