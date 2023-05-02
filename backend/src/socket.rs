use std::{thread, sync::{mpsc::{self, Sender}, atomic::{AtomicUsize, Ordering}}, collections::HashMap, net::TcpStream};

use websocket::{sync::Client, OwnedMessage};


static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub enum SocketMessage {
    NewClient(Client<TcpStream>),
}

pub fn init_socket() -> Sender<SocketMessage> {
    let (tx, rx) = mpsc::channel::<SocketMessage>();

    thread::spawn(|| {
        let mut clients = HashMap::<usize, Client<TcpStream>>::new();

        for message in rx {
            match message {
                SocketMessage::NewClient(mut client) => {
                    let ip = client.peer_addr().unwrap();
                    let id = get_id();

                    println!("[INFO]: Client connected {id:} → {ip:?}");
                    client.send_message(&OwnedMessage::Text(String::from("Hi"))).unwrap();
                    clients.insert(id, client);
                },
            }
        }
    });

    tx
}
