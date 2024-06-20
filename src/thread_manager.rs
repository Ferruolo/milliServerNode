use std::collections::VecDeque;
use std::mem::swap;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use crate::data_processor::DataManager;
use crate::internal_lang::{execute, ImperativeOps, OperationsLang};
use crate::thread_manager::ThreadSignal::*;

//TODO: Write unit tests

enum ThreadSignal<T> {
    Task(OperationsLang<T>),
    Available(usize, Sender<ThreadSignal<T>>),
    Null,
    Kill,
    ErrorInstruction,
}

struct DBWorker<T> {
    id: usize,
    join_handle: JoinHandle<()>,
    sender: Sender<ThreadSignal<T>>,
}

pub struct ThreadManager<T> {
    num_threads: usize,
    threads: Vec<DBWorker<T>>,
    master_thread: Option<JoinHandle<()>>,
    master_mailbox: Sender<ThreadSignal<T>>,
    terminated: bool,
}








impl<T> DBWorker<T> {
    fn new(id: usize, return_address: &Sender<ThreadSignal<T>>, db_ref: &mut Arc<Mutex<DataManager<T>>>) -> Self {
        let ret = return_address.clone();
        let (tx, rx): (Sender<ThreadSignal<T>>, Receiver<ThreadSignal<T>>) = mpsc::channel();
        let thread_send = tx.clone();
        let self_id = id;
        println!("Spinning up thread {id}");
        let mut local_db = db_ref.clone();


        let t = thread::spawn(move || {
            ret.send(Available(self_id, thread_send.clone())).unwrap();
            println!("Thread {self_id} spun up!");
            'threadLoop: loop {
                match rx.recv().unwrap_or(Null) {
                    Task(job) => {
                        execute(job, &mut local_db);
                        ret.send(Available(self_id, thread_send.clone())).unwrap();
                    }
                    Kill => {
                        println!("Killing thread {self_id}");
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
        };
    }
}



impl <T> ThreadManager<T> {
    pub(crate) fn new(n_threads: usize) -> ThreadManager<T> {
        let mut threads = Vec::new();

        let (master_send, master_recv) = mpsc::channel();
        for i in 0..n_threads {
            threads.push(DBWorker::new(i, &master_send));
        }


        let master_thread = thread::spawn(move || {
            let mut signal_queue: VecDeque<ThreadSignal<T>> = VecDeque::new();
            let mut worker_queue: VecDeque<Sender<ThreadSignal<T>>> = VecDeque::new();
            let mut killswitch = false;

            loop {
                match master_recv.recv_timeout(Duration::from_millis(10)).unwrap_or(Null) {
                    Task(t) => {
                        match worker_queue.pop_front() {
                            None => {
                                signal_queue.push_back(Task(t));
                            }
                            Some(w) => {
                                w.send(Task(t)).unwrap()
                            }
                        }
                    }
                    Available(idx, w) => {
                        println!("Thread {idx} now available");
                        match signal_queue.pop_front() {
                            None => {
                                println!("worker_queue length {}", worker_queue.len());
                                worker_queue.push_back(w);
                                println!("Added {idx} to worker queue");
                                println!("worker_queue length {}", worker_queue.len());
                            }
                            Some(t) => {
                                w.send(t).unwrap()
                            }
                        }
                    }
                    Null => {
                        println!("Worker Queue Len {} | Signal Queue len {}", worker_queue.len(), signal_queue.len());

                        if signal_queue.len() > 0 && worker_queue.len() > 0 {
                            match (signal_queue.pop_front(), worker_queue.pop_front()) {
                                (Some(s), Some(w)) => {
                                    w.send(s).unwrap()
                                }
                                (_, _) => panic!("Issue with non-empty queues")
                            }
                        } else if killswitch {
                            println!("Killing main thread!");
                            break;
                        }
                    }
                    Kill => {
                        println!("Kill message received");
                        for _ in 0..n_threads {
                            signal_queue.push_back(Kill)
                        }
                        killswitch = true;
                    }
                    ErrorInstruction => {}
                    _ => {}
                }
            }
        });

        return Self {
            num_threads: n_threads,
            threads,
            master_thread: Some(master_thread),
            master_mailbox: master_send.clone(),
            terminated: false,
        };
    }

    pub fn schedule(&mut self, task: ImperativeOps<T>) {
        if !self.terminated {
            self.master_mailbox.send(Task(task)).unwrap();
        }
    }

    pub fn terminate(&mut self) {
        println!("Send kill message to all threads");
        self.master_mailbox.send(Kill).unwrap();
        println!("Waiting on every thread to join");
        while let Some(t) = self.threads.pop() {
            t.join_handle.join().unwrap();
        }
        let mut master_thread = None;
        swap(&mut master_thread, &mut self.master_thread);
        match master_thread {
            None => {
                panic!("Beating a dead horse")
            }
            Some(t) => {
                t.join().unwrap();
            }
        }
        self.terminated = true;
    }
}
