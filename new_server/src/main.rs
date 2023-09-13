#[macro_use] 
extern crate magic_crypt;

use magic_crypt::MagicCryptTrait;

extern crate crypto_hash;
use crypto_hash::{hex_digest, Algorithm};

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

fn client_handler(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>, sender: mpsc::Sender<String>) {
    let mut buffer = [0; 1024];
    let client_addr = stream.peer_addr().unwrap();

    println!("Client connected: {:?}", client_addr);

    // Add the client's stream to the list of clients
    clients.lock().unwrap().push(stream.try_clone().unwrap());

    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    // Connection closed
                    break;
                }

                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received from {:?}: {}", client_addr, message);
		let mcrypt = new_magic_crypt!("magickey", 256);
		
		let msgs : Vec<&str> = message.split(":").collect();
		
		if msgs.len() != 3 {
                    println!("Invalid message format: {:?}", message);
                    continue; // Skip processing this message
                }
		
		let s1 = msgs[0];
		
		let s2 = msgs[1];
		
		let s3 = msgs[2];
		
    		let decrypted_string = mcrypt.decrypt_base64_to_string(s1).unwrap(); //Decrypts the string so we can read it.
    		println!("Decrypted String: {}", decrypted_string);
                
                println!("Received message from {:?}: {}", client_addr, s2);
                
                println!("Received hashed value from {:?}: {}", client_addr, s3);
                
                
                let hashed_value = hex_digest(Algorithm::SHA256,s2.as_bytes());
    	println!("Hashed value : {}",hashed_value);
              
                if hashed_value == s3 {
                println!("Authentication successful\n");
                }
                else {
                println!("Authentication failed\n");
                }
                
                
                   let mut s=String::new();
                   s.push_str(&client_addr.to_string());
                   s.push_str(":");
                   //s.push_str(&message.to_string());
                   s.push_str(&s1);
                   let _ = sender.send(s);

            }
            Err(_) => {
                // An error occurred, terminate the connection
                break;
            }
        }
    }

    
    clients.lock().unwrap().retain(|client| {
        client.peer_addr().unwrap() != client_addr
    });

    println!("Client disconnected: {:?}", client_addr);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind");
    println!("Server listening on port 7878...");

    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));

    let (sender, receiver) = mpsc::channel::<String>();

   let clients_for_broadcast = clients.clone();
    thread::spawn(move || {
       for message in receiver {
           let clients = clients_for_broadcast.lock().unwrap();
            for mut client in clients.iter() {
                client.write_all(message.as_bytes()).unwrap();
               // println!("new one : {}",message);
            }
       }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let clients_for_thread = clients.clone();
                let sender_for_thread = sender.clone();

                stream.write_all("Welcome to the chat!!".as_bytes()).unwrap();
                
                // Spawn a new thread to handle the client
                thread::spawn(|| {
                    client_handler(stream, clients_for_thread, sender_for_thread);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}









