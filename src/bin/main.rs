use std::net::TcpListener;
// Read data through TCP stream
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use server::ThreadPool;

fn main()  {
    // The listener binds to the local address and listens for connections 
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
    // Loop through our connections and determine which is established
        let stream = stream.unwrap();

        pool.execute(|| {
        // Call our handle_connection function to read the stream
            handle_connection(stream);
        });
    }
}

// Reads a TCP Stream
fn handle_connection(mut stream: TcpStream) {
    // Buffer stores our request
    let mut buffer = [0; 1024];
    // Read through buffer
    stream.read(&mut buffer).unwrap();
    // Expected string response
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // Checking if this is indeed the correct response
    let (status_line, filename) =
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        } else if buffer.starts_with(sleep) { 
            thread::sleep(Duration::from_secs(5)); 
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "error.html")
        };
    
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
    status_line, contents.len(), contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}