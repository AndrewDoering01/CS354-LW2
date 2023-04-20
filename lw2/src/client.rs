use rand::Rng;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    let mut is_starting = false;
    coin_toss(is_starting, stream);
    while true {
        if is_starting {
            let request = "Ping";
            stream.write(request.as_bytes())?;
            let mut response = String::new();
            stream.read_to_string(&mut response)?;
            println!("{}", response);
        } else {
            let mut response = String::new();
            stream.read_to_string(&mut response)?;
            println!("{}", response);
            let request = "Pong";
            stream.write(request.as_bytes())?;
        }
    }

    Ok(())
}
// returns a true if cointoss is over
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
