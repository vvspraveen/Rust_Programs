#[macro_use] extern crate magic_crypt;

extern crate crypto_hash;
use crypto_hash::{hex_digest, Algorithm};

use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
use magic_crypt::MagicCryptTrait;

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
    	let mcrypt = new_magic_crypt!("magickey", 256);
        input.clear();
        io::stdin().read_line(&mut input)?;
        let encrypted_string = mcrypt.encrypt_str_to_base64(&input);
        //stream.write_all(input.as_bytes())?;
        println!("Encrypted String: {}", encrypted_string);
        
        let mut s=String::new();
                   s.push_str(&encrypted_string);
                   s.push_str(":");
                   s.push_str(&input);
                   s.push_str(":");
                           
        let hashed_value = hex_digest(Algorithm::SHA256,input.as_bytes());
    	println!("Hashed value : {}",hashed_value);
           
                   
                   s.push_str(&hashed_value);
        
       // stream.write_all(encrypted_string.as_bytes())?;
        stream.write_all(s.as_bytes()).unwrap();
       // stream.write_all(input.as_bytes())?;
        
	}

}

