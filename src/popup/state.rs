use crate::{app::InputMode, event::Event};

use super::input::PopupInput;
use anyhow::Result;

pub struct PopupState {
    pub input: PopupInput,
}

impl PopupState {
    pub fn new(input: PopupInput) -> Self {
        Self { input }
    }

    pub fn parse(&self) -> Result<Event> {
        self.input.parse()
    }

    pub fn clear(&mut self) {
        self.input = PopupInput::default();
    }
}
