use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    stream.write(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{}", response);

    Ok(())
}
