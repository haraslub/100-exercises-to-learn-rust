// TODO: you have something to do in each of the modules in this crate!
mod description;
mod status;
mod title;

// A common pattern in Rust is to split code into multiple (private) modules
// and then re-export the public parts of those modules at the root of the crate.
//
// This hides the internal structure of the crate from your users, while still
// allowing you to organize your code however you like.
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

use description::TicketDescriptionError;
use status::ParseStatusError;
use title::TicketTitleError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TicketError {
    #[error(transparent)]
    Title(#[from] TicketTitleError),
    #[error(transparent)]
    Description(#[from] TicketDescriptionError),
    #[error(transparent)]
    Status(#[from] ParseStatusError),
}

#[derive(Debug, PartialEq, Clone)]
// We no longer need to make the fields private!
// Since each field encapsulates its own validation logic, there is no risk of
// a user of `Ticket` modifying the fields in a way that would break the
// invariants of the struct.
//
// Careful though: if you had any invariants that spanned multiple fields, you
// would need to ensure that those invariants are still maintained and go back
// to making the fields private.
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketError> {
        Ok(Ticket {
            title: TicketTitle::try_from(title).map_err(TicketError::Title)?,
            description: TicketDescription::try_from(description).map_err(TicketError::Description)?,
            status: Status::try_from(status).map_err(TicketError::Status)?,
        })
    }
    
    // Add a new method that takes &str references instead of Strings
    pub fn new_from_str(title: &str, description: &str, status: &str) -> Result<Self, TicketError> {
        Ok(Ticket {
            title: TicketTitle::try_from(title).map_err(TicketError::Title)?,
            description: TicketDescription::try_from(description).map_err(TicketError::Description)?,
            status: Status::try_from(status).map_err(TicketError::Status)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ticket_new_success() {
        let ticket = Ticket::new(
            "Fix login bug".to_string(),
            "Users cannot log in using the forgot password flow".to_string(),
            "todo".to_string()
        ).unwrap();
        
        assert_eq!(ticket.status, Status::ToDo);
    }
    
    #[test]
    fn test_ticket_new_invalid_title() {
        let result = Ticket::new(
            "".to_string(),
            "Users cannot log in using the forgot password flow".to_string(),
            "todo".to_string()
        );
        
        assert!(result.is_err());
        if let Err(TicketError::Title(err)) = result {
            assert_eq!(err.to_string(), "The title cannot be empty");
        } else {
            panic!("Expected TicketError::Title");
        }
    }
    
    #[test]
    fn test_ticket_new_invalid_description() {
        let result = Ticket::new(
            "Fix login bug".to_string(),
            "".to_string(),
            "todo".to_string()
        );
        
        assert!(result.is_err());
        if let Err(TicketError::Description(err)) = result {
            assert_eq!(err.to_string(), "The description cannot be empty");
        } else {
            panic!("Expected TicketError::Description");
        }
    }
    
    #[test]
    fn test_ticket_new_invalid_status() {
        let result = Ticket::new(
            "Fix login bug".to_string(),
            "Users cannot log in using the forgot password flow".to_string(),
            "invalid_status".to_string()
        );
        
        assert!(result.is_err());
        if let Err(TicketError::Status(_)) = result {
            // Success
        } else {
            panic!("Expected TicketError::Status");
        }
    }
    
    #[test]
    fn test_update_ticket_status() {
        let mut ticket = Ticket::new(
            "Fix login bug".to_string(),
            "Users cannot log in using the forgot password flow".to_string(),
            "todo".to_string()
        ).unwrap();
        
        assert_eq!(ticket.status, Status::ToDo);
        
        // Update status
        ticket.status = Status::try_from("inprogress").unwrap();
        assert_eq!(ticket.status, Status::InProgress);
        
        // Update status again
        ticket.status = Status::try_from("done").unwrap();
        assert_eq!(ticket.status, Status::Done);
    }
    
    #[test]
    fn test_ticket_new_from_str() {
        let ticket = Ticket::new_from_str(
            "Fix login bug",
            "Users cannot log in using the forgot password flow",
            "todo"
        ).unwrap();
        
        assert_eq!(ticket.status, Status::ToDo);
    }
}