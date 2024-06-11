extern crate crossbeam;


// src/macros.rs
#[macro_export]
macro_rules! timed_test {
    ($test_name:ident, $timeout:expr, $test_impl:ident) => {
        #[test]
        fn $test_name() {
            let (sender, receiver) = std::sync::mpsc::channel();

            std::thread::spawn(move || {
                $test_impl();
                let _ = sender.send(());
            });

            if receiver.recv_timeout(std::time::Duration::from_secs($timeout)).is_err() {
                panic!("Test Timed Out");
            }
        }
    };
}
