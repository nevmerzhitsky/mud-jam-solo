use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::{Rc, Weak};

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

        *room1.north_exit.borrow_mut() = RoomExit::Pathway(Rc::downgrade(&room2));
        *room1.south_exit.borrow_mut() = RoomExit::Pathway(Rc::downgrade(&room2));
        *room1.east_exit.borrow_mut() = RoomExit::Pathway(Rc::downgrade(&room1));

        *room2.north_exit.borrow_mut() = RoomExit::Pathway(Rc::downgrade(&room1));
        *room2.south_exit.borrow_mut() = RoomExit::Pathway(Rc::downgrade(&room1));
        *room2.west_exit.borrow_mut() = RoomExit::Pathway(Rc::downgrade(&room2));

        self.area.push(room1);
        self.area.push(room2);
    }
}

impl Entity for World {
    fn get_id(&self) -> u32 {
        self.id
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
            "Room {{ id: {}, exists (NSWEUD): {}-{}-{}-{}-{}-{} }}",
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
