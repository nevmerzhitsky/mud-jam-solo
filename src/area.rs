use derive_more::{Display, From};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::socium::{Character, CharacterId, CharacterRef};
use crate::utils::BuildRef;

pub trait Entity {}

pub trait Teleportable {
    fn move_from_to(&self, from: &Room, to: &Room) -> bool;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, From)]
pub struct WorldId(u32);

pub struct World {
    id: WorldId,
    name: String,
    characters: HashMap<CharacterId, CharacterRef>,
    area: Vec<Rc<Room>>,
}

impl World {
    pub fn new(id: WorldId, name: String) -> Self {
        Self {
            id,
            name,
            characters: HashMap::new(),
            area: Vec::new(),
        }
    }

    pub fn get_id(&self) -> WorldId {
        self.id
    }

    pub fn add_character(&mut self, char: Character) -> CharacterId {
        let id = char.get_id();
        self.characters.insert(id, char.build_ref());

        id
    }

    pub fn get_character(&self, id: CharacterId) -> Option<&CharacterRef> {
        self.characters.get(&id)
    }

    pub fn add_room(&mut self, room: Rc<Room>) {
        self.area.push(room);
    }

    pub fn get_any_room(&self) -> Rc<Room> {
        match self.area.first() {
            None => panic!("No rooms in the area"),
            Some(r) => r.clone(),
        }
    }

    pub fn spawn_character(&mut self, mut char: Character) -> CharacterId {
        // TODO Choose a proper room for the spawn: the room of exit or all players hub.
        let spawn_room = self.get_any_room();

        char.set_current_room(spawn_room.clone());

        let char_id = self.add_character(char);

        // TODO Add &char to the room's list.

        char_id
    }
}

impl BuildRef for World {
    fn build_ref(self) -> WorldRef {
        Rc::new(RefCell::new(self))
    }
}

pub type WorldRef = Rc<RefCell<World>>;

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

#[derive(Debug, Display, PartialEq, Eq, Hash, Clone, Copy, From)]
pub struct RoomId(u32);

pub struct Room {
    id: RoomId,
    world: WorldRef,
    north_exit: RefCell<RoomExit>,
    south_exit: RefCell<RoomExit>,
    west_exit: RefCell<RoomExit>,
    east_exit: RefCell<RoomExit>,
    up_exit: RefCell<RoomExit>,
    down_exit: RefCell<RoomExit>,
}

impl Room {
    pub fn new_in_void(id: RoomId, world: WorldRef) -> Room {
        Room {
            id,
            world,
            north_exit: RefCell::new(RoomExit::DeadEnd),
            south_exit: RefCell::new(RoomExit::DeadEnd),
            east_exit: RefCell::new(RoomExit::DeadEnd),
            west_exit: RefCell::new(RoomExit::DeadEnd),
            up_exit: RefCell::new(RoomExit::DeadEnd),
            down_exit: RefCell::new(RoomExit::DeadEnd),
        }
    }

    pub fn get_id(&self) -> RoomId {
        self.id
    }

    pub fn get_world(&self) -> &WorldRef {
        &self.world
    }

    pub fn get_exit(&self, direction: &MoveDirection) -> &RefCell<RoomExit> {
        match direction {
            MoveDirection::North => &self.north_exit,
            MoveDirection::South => &self.south_exit,
            MoveDirection::East => &self.east_exit,
            MoveDirection::West => &self.west_exit,
            MoveDirection::Up => &self.up_exit,
            MoveDirection::Down => &self.down_exit,
        }
    }

    pub fn set_exit(&self, direction: &MoveDirection, to_room: Rc<Room>) {
        let exit_ref = self.get_exit(direction);

        *exit_ref.borrow_mut() = RoomExit::Pathway(Rc::downgrade(&to_room));
    }

    pub fn set_dead_end(&self, direction: &MoveDirection) {
        let exit_ref = self.get_exit(direction);

        *exit_ref.borrow_mut() = RoomExit::DeadEnd;
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

#[derive(Debug)]
pub enum MoveDirection {
    North,
    South,
    East,
    West,
    Up,
    Down,
}
