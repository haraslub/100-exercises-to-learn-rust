use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Let's start sketching our ticket store!
//  First task: implement `IntoIterator` on `TicketStore` to allow iterating over all the tickets
//  it contains using a `for` loop.
//
// Hint: you shouldn't have to implement the `Iterator` trait in this case.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
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
        }
    }
    
    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn open_tickets(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets.iter().filter(|t| t.status == Status::ToDo)
    }
}

// SOLUTION:
impl IntoIterator for TicketStore {
    type Item = Ticket; // a type that each iteration produces
    type IntoIter = std::vec::IntoIter<Ticket>; // the actual type of the iterator actually returned

    fn into_iter(self) -> Self::IntoIter {
        // because tickets is Vec and it has already implemented IntoIterator trait, we use it here
        // i.e., we delegate it
        self.tickets.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }

    #[test]
    fn open_tickets() {
        let mut store = TicketStore::new();

        // Create sample of tickets
        let bug_fix = Ticket {
            title: TicketTitle::try_from("Fix critical bug".to_string()).unwrap(),
            description: TicketDescription::try_from("Emergency production bug".to_string()).unwrap(),
            status: Status::ToDo,
        };

        let refactor = Ticket {
            title: TicketTitle::try_from("Code refactoring".to_string()).unwrap(),
            description: TicketDescription::try_from("Technical debt reduction".to_string()).unwrap(),
            status: Status::InProgress,
        };

        store.add_ticket(bug_fix);
        store.add_ticket(refactor);

        for ticket in store.open_tickets() {
            assert_eq!(ticket.status, Status::ToDo)
        }
    }
}
