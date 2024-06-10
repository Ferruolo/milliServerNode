use std::net::TcpListener;

static HTML_DIR: &str = "./html/";


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();



    'outer: loop {
        for stream in listener.incoming().take(1) {
            let stream = stream.unwrap();

        }
    }



    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }


    println!("Shutting down.");
}
