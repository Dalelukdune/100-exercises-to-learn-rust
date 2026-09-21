use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, String> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);

        match self.sender.try_send(
            Command::Insert {
                draft,
                response_channel: response_sender,
            }
        ) {
            Ok(_) => Ok(response_receiver.recv().unwrap()),
            Err(_) => Err("Try send error occurred".into()),
        }
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, String> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);

        match self.sender.try_send(
            Command::Get {
                id,
                response_channel: response_sender,
            }
        ) {
            Ok(_) => Ok(response_receiver.recv().unwrap()),
            Err(_) => Err("Try send error occurred".into()),
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient {
        sender,
    }
}

enum Command {
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
                response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
