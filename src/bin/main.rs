use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use http_server::ThreadPool;


fn main() {
    let listener = TcpListener::bind("localhost:8090").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(_stream);
        });
    }
    println!("Shutting down...");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buff = [0; 512];
    stream.read(&mut buff).unwrap();
    let get_prefix = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, filename) = if buff.starts_with(get_prefix) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buff.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
