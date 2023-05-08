use std::thread;

use websocket::{sync::Server, OwnedMessage};

mod socket;

use socket::SocketManager;


fn main() {
    let server = Server::bind("127.0.0.1:8080").unwrap();
    println!("Up and running on port 8080");

    let sm = SocketManager::new();

    for request in server.filter_map(Result::ok) {
        let sm_clone = sm.clone();

        thread::spawn(move || {
            let client = request.use_protocol("kek").accept().unwrap();
            let ip = client.peer_addr().unwrap();

            println!("[INFO]: Client connected {ip:?}");
            let (mut receiver, mut sender) = client.split().unwrap();

            sender.send_message(&OwnedMessage::Text(String::from("Hi"))).unwrap();
            let client_id = sm_clone.add_client(sender);

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Binary(bin) => {
                        sm_clone.send_all(OwnedMessage::Binary(bin), Some(client_id));
                    },
                    _ => {
                        println!("Unsupported data type");
                    },
                }
            }
        });

    }
}
