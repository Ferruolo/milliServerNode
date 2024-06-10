use std::collections::VecDeque;
use std::mem::swap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use crate::{executor, Job};
use crate::thread_manager::ThreadSignal::*;

//TODO: Make this functional rather than imperative?? Just do whatever is cleaner


enum ThreadSignal {
    Task(Job),
    Available(usize, Sender<ThreadSignal>),
    Null,
    Kill,
    ErrorInstruction
}


struct ThreadManager {
    num_threads: usize,
    threads: Vec<Worker>,
    master_thread: Option<JoinHandle<()>>,
    master_mailbox: Sender<ThreadSignal>
}


struct Worker {
    id: usize,
    join_handle: JoinHandle<()>,
    sender: Sender<ThreadSignal>
    // receiver: Receiver<ThreadSignal>,
}


impl Worker {
    fn new(id: usize, return_address: &Sender<ThreadSignal>) -> Self {
        let ret = return_address.clone();
        let (tx, rx): (Sender<ThreadSignal>, Receiver<ThreadSignal>) = mpsc::channel();
        let thread_send = tx.clone();
        let self_id = id;


        let t = thread::spawn(move || {
            ret.send(Available(self_id, thread_send.clone())).unwrap();
            'threadLoop: loop {
                match rx.recv().unwrap_or(Null) {
                    Task(job) => {
                        executor(job);
                        ret.send(Available(self_id, thread_send.clone())).unwrap();
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
            sender: tx
        };
    }
}

impl ThreadManager {
    fn new(n_threads: usize) -> ThreadManager {
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
                match master_recv.recv().unwrap_or(Null) {
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
                    Available(_, w) => {
                        match signal_queue.pop_front() {
                            None => {
                                worker_queue.push_back(w)
                            }
                            Some(t) => {
                                w.send(t).unwrap()
                            }
                        }
                    }
                    Null => {
                        if killswitch {
                            break;
                        }
                        if signal_queue.len() > 0 && worker_queue.len() > 0{
                            match (signal_queue.pop_front(), worker_queue.pop_front()) {
                                (Some(s), Some(w)) => {
                                    w.send(s).unwrap()
                                }
                                (_, _) => panic!("Issue with non-empty queues")
                            }
                        }
                    }
                    Kill => {
                        for _ in 0..n_threads {
                            signal_queue.push_back(Kill)
                        }
                        killswitch = true;
                    }
                    ErrorInstruction => {}
                }
            }
        });

        return Self {
            num_threads: n_threads,
            threads,
            master_thread: Some(master_thread),
            master_mailbox: master_send.clone(),
        }
    }

    fn schedule(&mut self, job: Job) {
        self.master_mailbox.send(Task(job)).unwrap()
    }

    fn terminate(&mut self) {
        self.master_mailbox.send(Kill).unwrap();
        while let Some(t) = self.threads.pop() {
            t.join_handle.join().unwrap();
        }
        let mut master_thread = None;
        swap(&mut master_thread, &mut self.master_thread);
        master_thread.join().unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_are_testy() {
        assert_eq!(true, true);
    }
}
