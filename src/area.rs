use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

type AreaRoom = Rc<RefCell<Room>>;
type RoomConnection = Option<Weak<RefCell<Room>>>;

pub trait Entity {
    fn get_id(&self) -> u32;
}

#[derive(Debug)]
pub struct World {
    id: u32,
    name: String,
    area: Vec<AreaRoom>,
}

impl World {
    pub fn new(id: u32, name: String) -> World {
        let area: Vec<AreaRoom> = Vec::new();
        World { id, name, area }
    }

    pub fn feed_area(&mut self) {
        let room1 = Rc::new(RefCell::new(Room::new_in_void(1)));
        let room2 = Rc::new(RefCell::new(Room::new_in_void(2)));

        room1.borrow_mut().north_room = Option::from(Rc::downgrade(&room2));
        room1.borrow_mut().south_room = Option::from(Rc::downgrade(&room2));
        room1.borrow_mut().west_room = Option::from(Rc::downgrade(&room1));
        room1.borrow_mut().east_room = Option::from(Rc::downgrade(&room1));
        room1.borrow_mut().up_room = None;
        room1.borrow_mut().down_room = None;

        room2.borrow_mut().north_room = Option::from(Rc::downgrade(&room1));
        room2.borrow_mut().south_room = Option::from(Rc::downgrade(&room1));
        room2.borrow_mut().west_room = Option::from(Rc::downgrade(&room2));
        room2.borrow_mut().east_room = Option::from(Rc::downgrade(&room2));

        self.area.push(room1);
        self.area.push(room2);
    }
}

impl Entity for World {
    fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug)]
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
            north_room: None,
            south_room: None,
            east_room: None,
            west_room: None,
            up_room: None,
            down_room: None,
        }
    }
}

impl Entity for Room {
    fn get_id(&self) -> u32 {
        self.id
    }
}
