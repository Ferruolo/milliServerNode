mod data_processor;
mod thread_manager;

enum Mutations {
    Null,
    Insert,
    Add,
    Subtract,
    Return,
    Delete
}


enum Job {
    CheckInWithMeAndDoYourJob(usize)
}
fn executor(job: Job) {
    match job {
        Job::CheckInWithMeAndDoYourJob(i) => {
            println!("Job number {i} received")
        }
    }
}








