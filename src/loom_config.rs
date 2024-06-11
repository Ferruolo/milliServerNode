#[cfg(loom)]
pub(crate) use loom::sync::mpsc;

#[cfg(loom)]
pub(crate) use loom::thread;


#[cfg(not(loom))]
pub(crate) use std::sync::mpsc;

#[cfg(not(loom))]
pub(crate) use std::thread;



