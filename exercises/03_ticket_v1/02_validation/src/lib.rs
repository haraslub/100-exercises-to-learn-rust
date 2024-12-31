struct Ticket {
    title: String,
    description: String,
    status: String,
}

const MAX_TITLE_LENGTH: usize = 50;
const MAX_DESCRIPTION_LENGTH: usize = 500;

impl Ticket {
    // TODO: implement the `new` function.
    //  The following requirements should be met:
    //   - Only `To-Do`, `In Progress`, and `Done` statuses are allowed.
    //   - The `title` and `description` fields should not be empty.
    //   - the `title` should be at most 50 bytes long.
    //   - the `description` should be at most 500 bytes long.
    //  The method should panic if any of the requirements are not met.
    //  You can find the needed panic messages in the tests.
    //
    // You'll have to use what you learned in the previous exercises,
    // as well as some `String` methods. Use the documentation of Rust's standard library
    // to find the most appropriate options -> https://doc.rust-lang.org/std/string/struct.String.html
    fn new(title: String, description: String, status: String) -> Self {
        Self::validate_title(&title);
        Self::validate_description(&description);
        Self::validate_status(&status);
        
        Self {
            title,
            description,
            status,
        }
    }

    fn validate_title(title: &String) {
        if title.is_empty() {
            panic!("Title cannot be empty")
        }
        if title.len() > MAX_TITLE_LENGTH {
            panic!("Title cannot be longer than 50 bytes")
        }
    }

    fn validate_description(description: &String) {
        if description.is_empty() {
            panic!("Description cannot be empty")
        }
        if description.len() > MAX_DESCRIPTION_LENGTH {
            panic!("Description cannot be longer than 500 bytes")
        }
    }

    fn validate_status(status: &str) {
        match status {
            "To-Do" | "Done" | "In Progress" => (),
            _ => panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
