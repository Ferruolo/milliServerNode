use std::sync::Arc;

pub mod data_processor;
pub mod thread_manager;

#[macro_use]
mod macros;
pub mod internal_lang;
pub mod web_server;





pub enum Job {
    Kill,
    Execute(Arc<dyn Fn() + Send + Sync>)
}

fn executor(job: Job) {
    match job {
        Job::Execute(f) => {
            f();
        }
        _ => {}
    }
}

// pub const SERVER: fn(usize, Vec<ImperativeOps<FakeDatum>>) = web_server::run_fake_web_server;

