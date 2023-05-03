use std::{thread, sync::{mpsc::{self, Sender}, atomic::{AtomicUsize, Ordering}, Mutex, Arc}, collections::HashMap, net::TcpStream};

use websocket::sync::Writer;


static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub enum SocketMessage {
    NewClient(Writer<TcpStream>),
}

pub struct SocketManager {
    clients: Arc<Mutex<HashMap<usize, Writer<TcpStream>>>>,
    pub tx: Sender<SocketMessage>,
}

impl SocketManager {
    pub fn new() -> Self {
        let clients = Arc::new(Mutex::new(HashMap::<usize, Writer<TcpStream>>::new()));

        let (tx, rx) = mpsc::channel::<SocketMessage>();

        let thread_clients = clients.clone();
        thread::spawn(move || {
            for message in rx {
                match message {
                    SocketMessage::NewClient(client) => {
                        let id = get_id();

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

    pub fn clients(&self) -> Vec<usize> {
        let clients = self.clients.lock().unwrap();
        let ids = clients.keys().map(|&x| x).collect();

        return ids;
    }
}

