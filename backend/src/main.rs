use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;


fn main() {
    let server = Server::bind("127.0.0.1:8080").unwrap();

    for request in server.filter_map(Result::ok) {
        thread::spawn(|| {

            let client = request.use_protocol("kek").accept().unwrap();
            let ip = client.peer_addr().unwrap();
            println!("Client {ip} connected");

            let (mut receiver, mut sender) = client.split().unwrap();

			for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
						sender.send_message(&message).unwrap();
						println!("Client {ip} disconnected");
						return;
                    },
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    },
					_ => sender.send_message(&message).unwrap(),
                }
            }
        });
    }

    println!("Hello, world!");
}
