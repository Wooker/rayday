use crate::{app::InputMode, event::Event};

use super::input::PopupInput;
use anyhow::Result;

#[derive(Debug)]
pub struct PopupState {
    pub input: PopupInput,
    pub visible: bool,
}

impl PopupState {
    pub fn new(input: PopupInput) -> Self {
        Self {
            input,
            visible: false,
        }
    }

    pub fn parse(&self) -> Result<Event> {
        self.input.parse()
    }

    pub fn clear(&mut self) {
        self.input = PopupInput::default();
    }
}
