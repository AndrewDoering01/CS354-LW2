use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let mut is_starting = false;
    coin_toss(is_starting, stream);
    loop {
        if is_starting {
            let request = "Ping";
            stream.write(request.as_bytes());
            let mut response = String::new();
            stream.read_to_string(&mut response);
            println!("{}", response);
            stream.flush().unwrap();
        } else {
            let mut response = String::new();
            stream.read_to_string(&mut response);
            println!("{}", response);
            stream.flush().unwrap();
            let request = "Pong";
            stream.write(request.as_bytes());
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening on: {}", listener.local_addr()?);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn coin_toss(is_starting: bool, stream: TcpStream) {
    let mut rng = rand::thread_rng();
    let mut is_over = false;
    let mut buf = [0; 1024];
    while !is_over {
        stream.read_exact(&mut buf);
        let coin: u8 = rng.gen_range(0..=1);
        stream.read_exact(&mut buf).unwrap();
        let coin2: u8 = buf[0];

        if coin < coin2 {
            let lost: u8 = 2;
            stream.write(&[lost as u8]).unwrap();
            is_over = true;
            is_starting = false;
        }
        if coin > coin2 {
            let won: u8 = 3;
            stream.write(&[won as u8]).unwrap();
            is_over = true;
            is_starting = true;
        } else {
            stream.write(&[coin as u8]).unwrap();
        }
    }
}
