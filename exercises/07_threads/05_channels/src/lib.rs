use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

pub mod data;
pub mod store;

pub enum Command {
    Insert(data::TicketDraft),
    Shutdown,
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> (Sender<Command>, JoinHandle<()>) {
    let (sender, receiver) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || server(receiver));
    (sender, handle)
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    let mut ticket_store = store::TicketStore::new();

    for command in receiver {
        match command {
            Command::Insert(draft) => ticket_store.add_ticket(draft),
            Command::Shutdown => break
        };
    }
}
