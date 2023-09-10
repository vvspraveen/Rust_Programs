use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

fn get_messages(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    // Connection closed by the server
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..n]);
                
                println!("{}",message);
               // println!("Via Server:{}: {}",stream.peer_addr().unwrap(), message);
            }
            Err(_) => {
                // An error occurred, terminate the thread
                break;
            }
        }
    }
}

fn main() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the server!");

    // Spawn a separate thread to receive messages from the server
    let mut receive_stream = stream.try_clone()?;
    thread::spawn(move || {
        get_messages(&mut receive_stream);
    });

    // Main thread for sending  messages
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;

	}

}

