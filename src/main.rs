use std::net::TcpListener;

use http_server::{handler, threading::thread_pool::ThreadPool};

// TODO: Understand multithreading T~T
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8096").expect("Listener failed to bind!");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handler(stream))
    }
}
