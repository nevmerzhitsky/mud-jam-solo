pub trait Object {
    fn get_id(&self) -> u32;
}

#[derive(Debug)]
pub struct World {
    id: u32,
    name: String,
}

impl World {
    pub fn new(id: u32, name: String) -> World {
        World { id, name }
    }
}

impl Object for World {
    fn get_id(&self) -> u32 {
        self.id
    }
}
