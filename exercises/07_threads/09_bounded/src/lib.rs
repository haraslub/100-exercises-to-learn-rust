// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, SyncSender, TrySendError};

pub mod data;
pub mod store;

/// Errors that can occur when interacting with the TicketStore client
#[derive(Debug)]
pub enum ClientError {
    /// The command channel is full (bounded channel capacity exceeded)
    ChannelFull,
    /// The server has disconnected or shut down
    ServerDisconnected,
}

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, ClientError> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);
        
        // Send the command and handle potential errors
        self.sender
            .try_send(Command::Insert{ draft, response_channel: response_sender })
            .map_err(|err| match err {
                TrySendError::Full(_) => ClientError::ChannelFull,
                TrySendError::Disconnected(_) => ClientError::ServerDisconnected,
            })?; // -> immediately return the ClientError 
        
        // Wait for response and handle potential errors
        response_receiver
            .recv()
            .map_err(|_| ClientError::ServerDisconnected)
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, ClientError> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);
        
        // Send the command and handle potential errors
        self.sender
            .try_send(Command::Get{ id, response_channel: response_sender })
            .map_err(|err| match err {
                TrySendError::Full(_) => ClientError::ChannelFull,
                TrySendError::Disconnected(_) => ClientError::ServerDisconnected,
            })?; // -> immediately return the ClientError 
        
        // Wait for response and handle potential errors
        response_receiver
            .recv()
            .map_err(|_| ClientError::ServerDisconnected)
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

pub enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                // If we can't send the response, the client may have timed out
                // or disconnected. Log it but continue serving other requests.
                if let Err(e) = response_channel.try_send(id) {
                    eprintln!("Failed to send Insert response: {:?}", e);
                }
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                // If we can't send the response, the client may have timed out
                // or disconnected. Log it but continue serving other requests.
                if let Err(e) = response_channel.try_send(ticket.cloned()) {
                    eprintln!("Failed to send Get response: {:?}", e);
                }
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
