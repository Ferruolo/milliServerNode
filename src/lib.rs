use std::sync::Arc;
mod data_processor;
mod thread_manager;

#[macro_use]
mod macros;
mod internal_lang;
mod web_server;

enum Job {
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
