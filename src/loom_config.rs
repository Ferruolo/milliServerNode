

// #[cfg(loom)]
// pub(crate) mod sync {
//     pub use loom::sync::mpsc;
//     pub use loom::thread;
// }

#[cfg(not(loom))]
pub(crate) mod sync {
    pub use std::sync::mpsc;
    pub use std::thread;
}
