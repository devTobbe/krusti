use std::collections::{HashMap, HashSet};

/// Represents a note in Anki
#[derive(Debug, Clone)]
pub struct Note {
    id: Option<u64>,
    // TODO: model: Model
    fields: HashMap<String, String>,
    tags: HashSet<String>,
    //TODO: media: Vec<FieldMedia>,
}

impl Note {
    pub fn new() {
        todo!()
    }

    pub fn id() {
        todo!()
    }

    //pub fn model() {
    //    todo!()
    //}

    pub fn field_values() {
        todo!()
    }

    pub fn field_value() {
        todo!()
    }

    pub fn tags() {
        todo!()
    }

    //pub fn media() {
    //    todo!()
    //}
}
