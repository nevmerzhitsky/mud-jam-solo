use derive_more::From;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::area::Room;
use crate::game::PlayerRef;
use crate::utils::BuildRef;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, From)]
pub struct CharacterId(u32);

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

impl fmt::Debug for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Character")
            .field("id", &self.id)
            .field("main_char", &self.owner.as_ref().map(|char| char.borrow().get_id()))
            .field("current_room", &self.current_room)
            .finish()
    }
}

pub type CharacterRef = Rc<RefCell<Character>>;
