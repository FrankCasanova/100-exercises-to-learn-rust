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
    pub fn new(title: TicketTitle, description: TicketDescription) -> Ticket {
        Ticket {
            title,
            description,
            status: Status::ToDo,
        }
    }

    pub fn title(&self) -> &TicketTitle {
        &self.title
    }

    pub fn description(&self) -> &TicketDescription {
        &self.description
    }
    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn change_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn change_title(&mut self, title: TicketTitle) {
        self.title = title;
    }

    pub fn change_description(&mut self, description: TicketDescription) {
        self.description = description;
    }

    pub fn is_done(&self) -> bool {
        self.status == Status::Done
    }

    pub fn is_in_progress(&self) -> bool {
        self.status == Status::InProgress
    }

    pub fn is_to_do(&self) -> bool {
        self.status == Status::ToDo
    }
}



