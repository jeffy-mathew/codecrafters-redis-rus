// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0; 512];
                loop {
                    if stream.read(&mut buf).unwrap() == 0 {
                        break;
                    }
                    println!("{}", String::from_utf8_lossy(&buf));
                    stream.write("+PONG\r\n".as_bytes()).unwrap();
                }
            }

            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
