use std::thread;

use websocket::{sync::Server, OwnedMessage};

mod socket;

use socket::{SocketManager, SocketMessage};


fn main() {
    let server = Server::bind("127.0.0.1:8080").unwrap();
    println!("Up and running on port 8080");

    let sm = SocketManager::new();

    for request in server.filter_map(Result::ok) {
        let sm_send_clone = sm.tx.clone();

        thread::spawn(move || {
            let client = request.use_protocol("kek").accept().unwrap();
            let ip = client.peer_addr().unwrap();

            println!("[INFO]: Client connected {ip:?}");
            let (mut receiver, mut sender) = client.split().unwrap();

            sender.send_message(&OwnedMessage::Text(String::from("Hi"))).unwrap();
            sm_send_clone
                .send(SocketMessage::NewClient(sender))
                .expect("Failed to add client");

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Text(text) => {
                    },
                    _ => {
                    },
                }
            }
        });

    }
}
