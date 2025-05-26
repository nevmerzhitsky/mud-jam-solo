use crate::socium::{Character, CharacterRef, Player, PlayerRef};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub trait Entity {
    fn get_id(&self) -> u32;
}

pub struct World {
    id: u32,
    name: String,
    characters: HashMap<u32, CharacterRef>,
    players: HashMap<u32, PlayerRef>,
    area: Vec<Rc<Room>>,
}

impl World {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            characters: HashMap::new(),
            players: HashMap::new(),
            area: Vec::new(),
        }
    }

    pub fn add_character(&mut self, char: Character) -> u32 {
        let id = char.get_id();
        self.characters.insert(id, Rc::new(RefCell::new(char)));

        id
    }

    pub fn get_character(&self, id: u32) -> Option<&CharacterRef> {
        self.characters.get(&id)
    }

    pub fn add_player(&mut self, player: Player) -> u32 {
        let id = player.get_id();
        self.players.insert(id, Rc::new(RefCell::new(player)));

        id
    }

    pub fn get_player(&self, id: u32) -> Option<&PlayerRef> {
        self.players.get(&id)
    }

    pub fn fill_area(&mut self) {
        let room1 = Rc::new(Room::new_in_void(1));
        let room2 = Rc::new(Room::new_in_void(2));

        *room1.north_exit.borrow_mut() = RoomExit::Pathway(Rc::downgrade(&room2));
        *room1.south_exit.borrow_mut() = RoomExit::Pathway(Rc::downgrade(&room2));
        *room1.east_exit.borrow_mut() = RoomExit::Pathway(Rc::downgrade(&room1));

        *room2.north_exit.borrow_mut() = RoomExit::Pathway(Rc::downgrade(&room1));
        *room2.south_exit.borrow_mut() = RoomExit::Pathway(Rc::downgrade(&room1));
        *room2.west_exit.borrow_mut() = RoomExit::Pathway(Rc::downgrade(&room2));

        self.area.push(room1);
        self.area.push(room2);
    }

    pub fn get_any_room(&self) -> Rc<Room> {
        match self.area.first() {
            None => panic!("No rooms in the area"),
            Some(r) => r.clone(),
        }
    }

    pub fn spawn_player_character(&mut self, player: Player, char: Character) -> &PlayerRef {
        // TODO Choose a proper room for the spawn: the room of exit or all players hub.
        let spawn_room = self.get_any_room();

        let char_a_id = self.add_character(char);
        let player_a_id = self.add_player(player);

        let char_a = self.get_character(char_a_id).unwrap();
        char_a.borrow_mut().set_current_room(spawn_room.clone());

        let player_a = self.get_player(player_a_id).unwrap();
        player_a.borrow_mut().set_main_char(char_a.clone());

        // TODO Add &char to the room's list.

        player_a
    }
}

impl Entity for World {
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("World")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("area", &self.area)
            .finish()
    }
}

pub enum RoomExit {
    DeadEnd,
    Pathway(Weak<Room>),
}

pub struct Room {
    id: u32,
    // number: u32,
    north_exit: RefCell<RoomExit>,
    south_exit: RefCell<RoomExit>,
    west_exit: RefCell<RoomExit>,
    east_exit: RefCell<RoomExit>,
    up_exit: RefCell<RoomExit>,
    down_exit: RefCell<RoomExit>,
}

impl Room {
    pub fn new_in_void(id: u32) -> Room {
        Room {
            id,
            north_exit: RefCell::new(RoomExit::DeadEnd),
            south_exit: RefCell::new(RoomExit::DeadEnd),
            east_exit: RefCell::new(RoomExit::DeadEnd),
            west_exit: RefCell::new(RoomExit::DeadEnd),
            up_exit: RefCell::new(RoomExit::DeadEnd),
            down_exit: RefCell::new(RoomExit::DeadEnd),
        }
    }
}

impl Entity for Room {
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Room {}

impl fmt::Debug for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn get_exit_caption(rc: &RefCell<RoomExit>) -> String {
            match rc.borrow().deref() {
                RoomExit::DeadEnd => String::from("#"),
                RoomExit::Pathway(wr) => match wr.upgrade() {
                    None => panic!("No reference on a room exit"),
                    Some(r) => r.id.to_string(),
                },
            }
        }

        write!(
            f,
            "Room {{ id: {}, exits (NSWEUD): {}-{}-{}-{}-{}-{} }}",
            self.id,
            get_exit_caption(&self.north_exit),
            get_exit_caption(&self.south_exit),
            get_exit_caption(&self.west_exit),
            get_exit_caption(&self.east_exit),
            get_exit_caption(&self.up_exit),
            get_exit_caption(&self.down_exit)
        )
    }
}
