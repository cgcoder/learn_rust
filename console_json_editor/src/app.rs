use std::collections::HashMap;

pub enum CurrentScreen {
    Main, Editing, Exiting,
}

pub enum CurrentlyEditing {
    Key, Value,
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: Option::None,
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs.insert(std::mem::replace(&mut self.key_input, String::new()), std::mem::replace(&mut self.value_input, String::new()));
        self.currently_editing = Option::None;
    }

    pub fn toggle_editing(&mut self) {
        
    }
}