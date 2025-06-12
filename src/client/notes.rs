use std::{io::{Error, Result}};

use crate::http::{HttpClient};

pub struct NoteClient {
    sender: HttpClient
}

impl NoteClient {
    pub fn new(sender: HttpClient) -> Self {
        Self{sender}
    }

    pub fn find_notes(&self, deck : &str) -> Result<Vec<u64>, Error> {
        let response = self.sender.send("findNotes", Some(format!("deck: {}", deck).as_str())).map_err(|e| Err(Error) );
        
        todo!()
    }
}
