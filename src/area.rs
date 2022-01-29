use std::rc::Rc;
use std::rc::Weak;

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

    pub fn feed_area(&mut self) {
        let mut r1 = Room::new_in_void(1);
        let mut r2 = Room::new_in_void(2);

        let r2_rc = Rc::new(r2);

        r1.north_room = Option::from(Rc::downgrade(&r2_rc));
        r1.south_room = Option::from(Rc::downgrade(&r2_rc));
        r1.west_room = Option::from(Rc::downgrade(&r2_rc));
        r1.east_room = Option::from(Rc::downgrade(&r2_rc));

        let r1_rc = Rc::new(r1);

        self.area.push(r1_rc);
        self.area.push(r2_rc);
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
    north_room: Option<Weak<Room>>,
    south_room: Option<Weak<Room>>,
    west_room: Option<Weak<Room>>,
    east_room: Option<Weak<Room>>,
    up_room: Option<Weak<Room>>,
    down_room: Option<Weak<Room>>,
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
