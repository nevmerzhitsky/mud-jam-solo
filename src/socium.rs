use crate::area::Room;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Character {
    pub id: u32,
    pub current_room: Option<Rc<Room>>,
}

impl Character {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            // id_cell,
            current_room: None,
        }
    }

    pub fn set_current_room(&mut self, start_room: Rc<Room>) {
        self.current_room = Some(start_room);
    }

    pub fn unset_current_room(&mut self) {
        self.current_room = None;
    }
}

pub type CharacterRef = Rc<RefCell<Character>>;

// ----------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub main_char: Option<CharacterRef>,
}

impl Player {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            main_char: None,
        }
    }

    pub fn set_main_char(&mut self, char_a: CharacterRef) {
        self.main_char = Some(char_a);
    }

    pub fn unset_main_char(&mut self) {
        self.main_char = None;
    }
}

pub type PlayerRef = Rc<RefCell<Player>>;
