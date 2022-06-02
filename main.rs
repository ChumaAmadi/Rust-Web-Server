// Listen for a TCP connection
use std::net::TcpListener;
// Read data through TCP stream
use std::net:TcpStream;
use std::io::prelude;

fn main()  {
    let listener: TcpListener =
    // The listener binds to the local address and listens for connections
        TcpListener::bind("127.0.0.1:8080").unwrap(); 

    for stream: Result<TcpStream, Error> in listener.incoming() {
    // Loop through our connections and determine which is established
        let stream: TcpStream = stream.unwrap();

        // Call our handle_connection function to read the stream
        handle_connection(stream);
    }
}

// Reads a TCP Stream
fn handle_connection(mut stream: TCP Stream) {
    // Buffer stores our data
    let mut buffer: [i32; 1024] = [0; 1024];
}