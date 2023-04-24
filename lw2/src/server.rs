use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream, game_num: u32) {
    loop {
        let mut request_buffer = [0; 1024]; // Read up to 1024 bytes
        let num_bytes = stream.read(&mut request_buffer).unwrap();
        //Atempts to create a srting using UTF-8 from a byte array the ..num_bytes creates a range for the number of bytes that
        // can be used to create the string
        let request = String::from_utf8_lossy(&request_buffer[..num_bytes]).to_string();
        println!("Game {}: Received message: {}", game_num, request);

        let response = String::from("Pong");
        // writes response to the stream as a byte array
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("Game {}: Sent response: {}", game_num, response);

        thread::sleep(Duration::from_secs(1));
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening on: {}", listener.local_addr()?);
    let mut i: u32 = 1;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let game_num = i;
                // move gives ownership of the value of game_num to the function handle_client
                std::thread::spawn(move || {
                    handle_client(stream, game_num);
                });
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
        i += 1;
    }

    Ok(())
}
