use crate::area::Room;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Character {
    id: u32,
    current_room: Option<Rc<Room>>,
}

impl Character {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            // id_cell,
            current_room: None,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_current_room(&mut self, start_room: Rc<Room>) {
        self.current_room = Some(start_room);
    }

    pub fn unset_current_room(&mut self) {
        self.current_room = None;
    }
}

impl PartialEq for Character {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Character {}

pub type CharacterRef = Rc<RefCell<Character>>;

// ----------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Player {
    id: u32,
    main_char: Option<CharacterRef>,
}

impl Player {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            main_char: None,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_main_char(&mut self, char_a: CharacterRef) {
        self.main_char = Some(char_a);
    }

    pub fn unset_main_char(&mut self) {
        self.main_char = None;
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Player {}

pub type PlayerRef = Rc<RefCell<Player>>;
