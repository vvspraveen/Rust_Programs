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

                // Send the received message to all clients (broadcast)
                   let mut s=String::new();
                   s.push_str(&client_addr.to_string());
                   s.push_str(":");
                   s.push_str(&message.to_string());
                   let _ = sender.send(s);

            }
            Err(_) => {
                // An error occurred, terminate the connection
                break;
            }
        }
    }

    // Remove the client's stream from the list of clients
    clients.lock().unwrap().retain(|client| {
        client.peer_addr().unwrap() != client_addr
    });

    println!("Client disconnected: {:?}", client_addr);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind");
    println!("Server listening on port 7878...");

    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));

    // Create a channel for sending messages to clients
    let (sender, receiver) = mpsc::channel::<String>();

     //Spawn a thread for handling message broadcasting to clients
   let clients_for_broadcast = clients.clone();
    thread::spawn(move || {
       for message in receiver {
           let clients = clients_for_broadcast.lock().unwrap();
            for mut client in clients.iter() {
                client.write_all(message.as_bytes()).unwrap();
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

