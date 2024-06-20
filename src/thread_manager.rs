use std::collections::VecDeque;
use std::mem::swap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use crate::{executor, Job};
use crate::thread_manager::ThreadSignal::*;

//TODO: Make this functional rather than imperative?? Just do whatever is cleaner

enum ThreadSignal {
    Task(Job),
    Available(usize, Sender<ThreadSignal>),
    Null,
    Kill,
    ErrorInstruction,
}

struct Worker {
    id: usize,
    join_handle: JoinHandle<()>,
    sender: Sender<ThreadSignal>,
}

pub struct ThreadManager {
    num_threads: usize,
    threads: Vec<Worker>,
    master_thread: Option<JoinHandle<()>>,
    master_mailbox: Sender<ThreadSignal>,
    terminated: bool,
}

impl Worker {
    fn new(id: usize, return_address: &Sender<ThreadSignal>) -> Self {
        let ret = return_address.clone();
        let (tx, rx): (Sender<ThreadSignal>, Receiver<ThreadSignal>) = mpsc::channel();
        let thread_send = tx.clone();
        let self_id = id;
        println!("Spinning up thread {id}");

        let t = thread::spawn(move || {
            ret.send(Available(self_id, thread_send.clone())).unwrap();
            println!("Thread {self_id} spun up!");
            'threadLoop: loop {
                match rx.recv().unwrap_or(Null) {
                    Task(job) => {
                        executor(job);
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



impl ThreadManager {
    pub(crate) fn new(n_threads: usize) -> ThreadManager {
        let mut threads = Vec::new();

        let (master_send, master_recv) = mpsc::channel();
        for i in 0..n_threads {
            threads.push(Worker::new(i, &master_send));
        }


        let master_thread = thread::spawn(move || {
            let mut signal_queue: VecDeque<ThreadSignal> = VecDeque::new();
            let mut worker_queue: VecDeque<Sender<ThreadSignal>> = VecDeque::new();
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

    pub fn schedule(&mut self, job: Job) {
        if !self.terminated {
            self.master_mailbox.send(Task(job)).unwrap();
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


#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::Job::Execute;
    use crate::timed_test;

    use super::*;

    fn basic_init_kill() {
        let mut manager = ThreadManager::new(4);
        manager.terminate();
    }

    // Use the macro to create a timed test
    #[test]
    timed_test!(timed_init_kill, 1, basic_init_kill);

    fn single_job_execute() {
        let num_jobs: u8 = 10;
        let mut manager = ThreadManager::new(4);
        let items: Vec<Arc<Mutex<u8>>> = (0..num_jobs).map(|x| Arc::new(Mutex::new(x))).collect();

        for (idx, item) in items.iter().enumerate() {
            assert_eq!(*item.lock().unwrap(), idx as u8);
        }

        for i in 0..num_jobs {
            let item_copy = Arc::clone(&items[i as usize]);
            let fxn = Arc::new(move || {
                println!("Executing Job {i}");
                let new_val = *item_copy.lock().unwrap() + 1;
                *item_copy.lock().unwrap() = new_val;
                println!("Job {i} Executed");
            });


            manager.schedule(Execute(fxn));
        }


        for (idx, item) in items.iter().enumerate() {
            println!("Test Idx being conducted");
            let item_copy = Arc::clone(item);
            let expected_idx = idx as u8 + 1;
            let fxn = Arc::new(move || {
                assert_eq!(*item_copy.lock().unwrap(), expected_idx);
            });
            manager.schedule(Execute(fxn));
        }

        manager.terminate();
    }

    #[test]
    timed_test!(timed_single_job_execute, 1, single_job_execute);
}

