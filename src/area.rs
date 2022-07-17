use std::cell::RefCell;
use std::fmt;
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
        let room1 = AreaRoom::new(RefCell::new(Room::new_in_void(1)));
        let room2 = AreaRoom::new(RefCell::new(Room::new_in_void(2)));

        {
            let mut room_mut = RefCell::borrow_mut(&room1);
            room_mut.north_room = Option::from(Rc::downgrade(&room2));
            room_mut.south_room = Option::from(Rc::downgrade(&room2));
            room_mut.west_room = Option::from(Rc::downgrade(&room1));
            room_mut.east_room = Option::from(Rc::downgrade(&room1));
        }

        {
            let mut room_mut = RefCell::borrow_mut(&room2);
            room_mut.north_room = Option::from(Rc::downgrade(&room1));
            room_mut.south_room = Option::from(Rc::downgrade(&room1));
            room_mut.west_room = Option::from(Rc::downgrade(&room2));
            room_mut.east_room = Option::from(Rc::downgrade(&room2));
        }

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

impl fmt::Debug for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn get_connection_caption(rc: &RoomConnection) -> String {
            match rc {
                None => "N".to_string(),
                Some(r) => r.upgrade().unwrap().borrow().id.to_string(),
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
