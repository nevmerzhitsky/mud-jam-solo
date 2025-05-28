use std::cell::RefCell;
use crate::action::WorldAction;
use crate::area::World;
use crate::socium::{Character, Player, PlayerRef};
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub struct Game {
    worlds: HashMap<u32, World>,
    players: HashMap<u32, PlayerRef>,
    actions_queue: VecDeque<WorldAction>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            worlds: HashMap::new(),
            players: HashMap::new(),
            actions_queue: VecDeque::new(),
        }
    }

    pub fn add_world(&mut self, world: World) -> u32 {
        let id = world.get_id();
        self.worlds.insert(id, world);

        id
    }

    pub fn get_world(&self, id: u32) -> Option<&World> {
        self.worlds.get(&id)
    }

    pub fn get_world_mut(&mut self, id: u32) -> Option<&mut World> {
        self.worlds.get_mut(&id)
    }

    pub fn add_player(&mut self, player: Player) -> u32 {
        let id = player.get_id();
        self.players.insert(id, Rc::new(RefCell::new(player)));

        id
    }

    pub fn get_player(&self, id: u32) -> Option<&PlayerRef> {
        self.players.get(&id)
    }

    pub fn spawn_player_character(&mut self, world_id: u32, player_id: u32, character: Character) {
        let world = self.get_world_mut(world_id).unwrap();
        let char_id = world.spawn_character(character);

        let char = world.get_character(char_id).unwrap();
        let char_ref = char.clone();

        let player = self.get_player(player_id).unwrap();
        player.borrow_mut().set_main_char(char_ref);
    }

    pub fn queue_action(&mut self, action: WorldAction) {
        self.actions_queue.push_back(action);
    }

    pub fn process_actions_queue(&mut self) {
        //for action in self.actions_queue.drain(..) {}
    }
}
