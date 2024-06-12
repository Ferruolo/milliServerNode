use std::char::from_u32_unchecked;
use std::sync::Arc;

mod data_processor;
mod thread_manager;

#[macro_use]
mod macros;

enum Mutations {
    Null,
    Insert,
    Add,
    Subtract,
    Return,
    Delete,
}


enum Job {
    Kill,
    CheckInWithMeAndDoYourJob(Arc<dyn Fn() + Send + Sync>),
}

fn executor(job: Job) {
    match job {
        Job::CheckInWithMeAndDoYourJob(f) => {
            f();
        }
        Job::Kill => {}
    }
}








