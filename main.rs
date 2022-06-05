// Listen for a TCP connection
use std::net::TcpListener;
// Read data through TCP stream
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main()  {
    let listener: TcpListener =
    // The listener binds to the local address and listens for connections
        TcpListener::bind("127.0.0.1:8080").unwrap(); 

    for stream in listener.incoming() {
    // Loop through our connections and determine which is established
        let stream: TcpStream = stream.unwrap();

        // Call our handle_connection function to read the stream
        handle_connection(stream);
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

    // Checking if this is indeed the correct response
    if buffer.starts_with(get) {
        // Stores content of our html file into a readable string
        let contents = fs::read_to_string("index.html").unwrap();
        //  Calling the amount of bytes in the contents for string interpolation
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(), contents);

        // Server receives response as bytes
        stream.write(response.as_bytes()).unwrap();
        // Server will read all the bytes before dumping them with flush and returning  a response each time
        stream.flush().unwrap();

    // Return an error 404 page if the response is incorrect
    } else {
    
    }

}