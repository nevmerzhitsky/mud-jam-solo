use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub trait Entity {
    fn get_id(&self) -> u32;
}

#[derive(Debug)]
pub struct World {
    id: u32,
    name: String,
    area: Vec<Rc<RefCell<Room>>>,
}

impl World {
    pub fn new(id: u32, name: String) -> World {
        let area: Vec<Rc<RefCell<Room>>> = Vec::new();
        World { id, name, area }
    }

    pub fn feed_area(&mut self) {
        let r1 = Room::new_in_void(1);
        let r2 = Room::new_in_void(2);

        let r1_cell = RefCell::new(r1);
        let r2_cell = RefCell::new(r2);

        let r1_rc = Rc::new(r1_cell);
        let r2_rc = Rc::new(r2_cell);

        r1_rc.borrow_mut().north_room = Option::from(Rc::downgrade(&r2_rc));
        r1_rc.borrow_mut().south_room = Option::from(Rc::downgrade(&r2_rc));
        r1_rc.borrow_mut().west_room = Option::from(Rc::downgrade(&r1_rc));
        r1_rc.borrow_mut().east_room = Option::from(Rc::downgrade(&r1_rc));

        r2_rc.borrow_mut().north_room = Option::from(Rc::downgrade(&r1_rc));
        r2_rc.borrow_mut().south_room = Option::from(Rc::downgrade(&r1_rc));
        r2_rc.borrow_mut().west_room = Option::from(Rc::downgrade(&r2_rc));
        r2_rc.borrow_mut().east_room = Option::from(Rc::downgrade(&r2_rc));

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
    north_room: Option<Weak<RefCell<Room>>>,
    south_room: Option<Weak<RefCell<Room>>>,
    west_room: Option<Weak<RefCell<Room>>>,
    east_room: Option<Weak<RefCell<Room>>>,
    up_room: Option<Weak<RefCell<Room>>>,
    down_room: Option<Weak<RefCell<Room>>>,
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
