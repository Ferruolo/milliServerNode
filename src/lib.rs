use std::sync::Arc;

mod data_processor;
mod thread_manager;

#[macro_use]
mod macros;
mod internal_lang;


enum Job {
    Kill,
    Execute(Arc<dyn Fn() + Send + Sync>),
    ParseAndCompile
}

fn executor(job: Job) {
    match job {
        Job::Execute(f) => {
            f();
        }
        _ => {}
    }
}


fn parser()

