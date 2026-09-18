use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
    number_of_tickets: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TicketId(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
            number_of_tickets: 0,
        }
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        if id.0 > self.number_of_tickets && id.0 < 1 {
            return None
        }

        Some(&self.tickets[id.0 as usize - 1])
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        self.number_of_tickets += 1;
        let ticket_id = TicketId(self.number_of_tickets);

        let ticket_to_add = Ticket {
            id: ticket_id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };

        self.tickets.push(ticket_to_add);

        ticket_id
    }
}

#[cfg(test)]
mod tests {
    use crate::{Status, TicketDraft, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let draft1 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id1 = store.add_ticket(draft1.clone());
        let ticket1 = store.get(id1).unwrap();
        assert_eq!(draft1.title, ticket1.title);
        assert_eq!(draft1.description, ticket1.description);
        assert_eq!(ticket1.status, Status::ToDo);

        let draft2 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id2 = store.add_ticket(draft2);
        let ticket2 = store.get(id2).unwrap();

        assert_ne!(id1, id2);
    }
}
