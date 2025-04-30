// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, TrySendError, SyncSender};

// Enum representing the different commands that can be sent to the server
enum Command {
    // Command to insert a new ticket draft into the system
    Insert {
        // The ticket draft to be inserted
        draft: TicketDraft,
        // Channel to send the generated TicketId back to the requester
        response_channel: SyncSender<TicketId>,
    },
    // Command to retrieve a ticket by its ID
    Get {
        // The ID of the ticket to retrieve
        id: TicketId,
        // Channel to send the retrieved ticket (if found) back to the requester
        response_channel: SyncSender<Option<Ticket>>,
    },
}

// Error indicating the store is overloaded and cannot accept more requests
#[derive(Debug, thiserror::Error)]
#[error("The store is overloaded")]
pub struct OverloadedError;

// Client for interacting with the ticket store
#[derive(Clone)]
pub struct TicketStoreClient {
    // Channel sender for sending commands to the server
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    // Insert a new ticket draft into the store
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, OverloadedError> {
        // Create a channel for receiving the response
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);
        // Try to send the insert command to the server
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        // Wait for and return the response
        Ok(response_receiver.recv().unwrap())
    }

    // Retrieve a ticket by its ID
    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, OverloadedError> {
        // Create a channel for receiving the response
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);
        // Try to send the get command to the server
        self.sender
            .try_send(Command::Get {
                id,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        // Wait for and return the response
        Ok(response_receiver.recv().unwrap())
    }
}

// Launch the ticket store server and return a client
pub fn launch(capacity: usize) -> TicketStoreClient {
    // Create a channel for communication between client and server
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    // Spawn a new thread to run the server
    std::thread::spawn(move || server(receiver));
    // Return a new client with the sender
    TicketStoreClient { sender }
}

// Server loop that processes incoming commands
pub fn server(receiver: Receiver<Command>) {
    // Create a new ticket store
    let mut store = TicketStore::new();
    // Main server loop
    loop {
        match receiver.recv() {
            // Handle insert command
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                // Add the ticket to the store
                let id = store.add_ticket(draft);
                // Send the ticket ID back to the client
                let _ = response_channel.try_send(id);
            }
            // Handle get command
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                // Get the ticket from the store
                let ticket = store.get(id);
                // Send the ticket back to the client
                let _ = response_channel.try_send(ticket.cloned());
            }
            // Handle channel disconnection
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}

pub mod data;
pub mod store;
