use std::{thread, sync::{mpsc::{self, Sender}, atomic::{AtomicUsize, Ordering}, Mutex, Arc}, collections::HashMap, net::TcpStream};

use websocket::{sync::Writer, OwnedMessage};


static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub enum SocketMessage {
    NewClient(usize, Writer<TcpStream>),
}

#[derive(Clone)]
pub struct SocketManager {
    clients: Arc<Mutex<HashMap<usize, Writer<TcpStream>>>>,
    tx: Sender<SocketMessage>,
}

impl SocketManager {
    pub fn new() -> Self {
        let clients = Arc::new(Mutex::new(HashMap::<usize, Writer<TcpStream>>::new()));

        let (tx, rx) = mpsc::channel::<SocketMessage>();

        let thread_clients = clients.clone();
        thread::spawn(move || {
            for message in rx {
                match message {
                    SocketMessage::NewClient(id, client) => {

                        thread_clients.lock().unwrap().insert(id, client);
                    },
                }
            }
        });

        SocketManager {
            clients,
            tx,
        }
    }

    pub fn add_client(&self, client: Writer<TcpStream>) -> usize {
        let id = get_id();
        self.tx
            .send(SocketMessage::NewClient(id, client))
            .expect("Failed to add client");

        return id;
    }

    pub fn send_all(&self, message: OwnedMessage, except: Option<usize>) {
        let mut clients = self.clients.lock().unwrap();
        clients.iter_mut().for_each(|(id, sender)| {
            if let Some(except_id) = except {
                if except_id == *id {
                    return;
                }
            }

            let result = sender.send_message(&message);

            if result.is_err() {
                eprintln!("Failed to send message to {id:}");
            }

        });
    }

    pub fn clients(&self) -> Vec<usize> {
        let clients = self.clients.lock().unwrap();
        let ids = clients.keys().map(|&x| x).collect();

        return ids;
    }
}

