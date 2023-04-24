use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    loop {
        let request = String::from("Ping");
        stream.write(request.as_bytes())?;
        stream.flush().unwrap();
        println!("Sent request: {}", request);
        let mut response_buffer = [0; 1024]; // Read up to 1024 bytes
        let num_bytes = stream.read(&mut response_buffer)?;
        let response = String::from_utf8_lossy(&response_buffer[..num_bytes]);
        println!("Received response: {response}");
        thread::sleep(Duration::from_secs(1));
    }
}
