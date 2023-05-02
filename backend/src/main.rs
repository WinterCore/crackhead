use websocket::sync::Server;

mod socket;

use socket::{init_socket, SocketMessage};


fn main() {
    let server = Server::bind("127.0.0.1:8080").unwrap();
    println!("Up and running on port 8080");

    let sender = init_socket();

    for request in server.filter_map(Result::ok) {
        let client = request.use_protocol("kek").accept().unwrap();
        sender.send(SocketMessage::NewClient(client)).expect("Failed to add client");

    }
}
