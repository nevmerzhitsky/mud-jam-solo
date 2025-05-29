use derive_more::From;
use std::cell::RefCell;
use std::rc::Rc;

use crate::area::Room;
use crate::utils::BuildRef;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, From)]
pub struct CharacterId(u32);

#[derive(Debug)]
pub struct Character {
    id: CharacterId,
    owner: Option<PlayerRef>,
    current_room: Option<Rc<Room>>,
}

impl Character {
    pub fn new(id: CharacterId) -> Self {
        Self {
            id,
            owner: None,
            current_room: None,
        }
    }

    pub fn get_id(&self) -> CharacterId {
        self.id
    }

    pub fn get_owner(&self) -> &Option<PlayerRef> {
        &self.owner
    }

    pub fn set_owner(&mut self, owner: PlayerRef) {
        self.owner = Some(owner);
    }

    pub fn unset_owner(&mut self) {
        self.owner = None;
    }

    pub fn get_current_room(&self) -> &Option<Rc<Room>> {
        &self.current_room
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

impl BuildRef for Character {
    fn build_ref(self) -> CharacterRef {
        Rc::new(RefCell::new(self))
    }
}

pub type CharacterRef = Rc<RefCell<Character>>;

// ----------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, From)]
pub struct PlayerId(u32);

#[derive(Debug)]
pub struct Player {
    id: PlayerId,
    main_char: Option<CharacterRef>,
}

impl Player {
    pub fn new(id: PlayerId) -> Self {
        Self {
            id,
            main_char: None,
        }
    }

    pub fn get_id(&self) -> PlayerId {
        self.id
    }

    pub fn get_main_char(&self) -> &Option<CharacterRef> {
        &self.main_char
    }

    pub fn set_main_char(&mut self, char: CharacterRef) {
        self.main_char = Some(char);
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

impl BuildRef for Player {
    fn build_ref(self) -> PlayerRef {
        Rc::new(RefCell::new(self))
    }
}

pub type PlayerRef = Rc<RefCell<Player>>;
