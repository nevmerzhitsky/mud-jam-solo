use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

type RoomConnection = RefCell<Weak<Room>>;

pub trait Entity {
    fn get_id(&self) -> u32;
}

#[derive(Debug)]
pub struct World {
    id: u32,
    name: String,
    area: Vec<Rc<Room>>,
}

impl World {
    pub fn new(id: u32, name: String) -> World {
        let area: Vec<Rc<Room>> = Vec::new();
        World { id, name, area }
    }

    pub fn fill_area(&mut self) {
        let room1 = Rc::new(Room::new_in_void(1));
        let room2 = Rc::new(Room::new_in_void(2));

        *room1.north_room.borrow_mut() = Rc::downgrade(&room2);
        *room1.south_room.borrow_mut() = Rc::downgrade(&room2);
        *room1.west_room.borrow_mut() = Rc::downgrade(&room1);
        *room1.east_room.borrow_mut() = Rc::downgrade(&room1);

        *room2.north_room.borrow_mut() = Rc::downgrade(&room1);
        *room2.south_room.borrow_mut() = Rc::downgrade(&room1);
        *room2.west_room.borrow_mut() = Rc::downgrade(&room2);
        *room2.east_room.borrow_mut() = Rc::downgrade(&room2);

        self.area.push(room1);
        self.area.push(room2);
    }
}

impl Entity for World {
    fn get_id(&self) -> u32 {
        self.id
    }
}

pub struct Room {
    id: u32,
    // number: u32,
    north_room: RoomConnection,
    south_room: RoomConnection,
    west_room: RoomConnection,
    east_room: RoomConnection,
    up_room: RoomConnection,
    down_room: RoomConnection,
}

impl Room {
    pub fn new_in_void(id: u32) -> Room {
        Room {
            id,
            north_room: RefCell::new(Weak::new()),
            south_room: RefCell::new(Weak::new()),
            east_room: RefCell::new(Weak::new()),
            west_room: RefCell::new(Weak::new()),
            up_room: RefCell::new(Weak::new()),
            down_room: RefCell::new(Weak::new()),
        }
    }
}

impl Entity for Room {
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl fmt::Debug for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn get_connection_caption(rc: &RoomConnection) -> String {
            match rc.borrow().upgrade() {
                None => String::from("X"),
                Some(r) => r.id.to_string(),
            }
        }

        write!(
            f,
            "Room {{ id: {}, exists (NSWEUD): {}-{}-{}-{}-{}-{} }}",
            self.id,
            get_connection_caption(&self.north_room),
            get_connection_caption(&self.south_room),
            get_connection_caption(&self.west_room),
            get_connection_caption(&self.east_room),
            get_connection_caption(&self.up_room),
            get_connection_caption(&self.down_room)
        )
    }
}
