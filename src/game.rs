use crate::action::WorldAction;
use crate::area::{World, WorldId};
use crate::socium::{Character, Player, PlayerId, PlayerRef};
use std::collections::{HashMap, VecDeque};
use mud_jam_solo::BuildRef;

pub struct Game {
    worlds: HashMap<WorldId, World>,
    players: HashMap<PlayerId, PlayerRef>,
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

    pub fn add_world(&mut self, world: World) -> WorldId {
        let id = world.get_id();
        self.worlds.insert(id, world);

        id
    }

    pub fn get_world(&self, id: WorldId) -> Option<&World> {
        self.worlds.get(&id)
    }

    pub fn get_world_mut(&mut self, id: WorldId) -> Option<&mut World> {
        self.worlds.get_mut(&id)
    }

    pub fn add_player(&mut self, player: Player) -> PlayerId {
        let id = player.get_id();
        self.players.insert(id, player.build_ref());

        id
    }

    pub fn get_player(&self, id: PlayerId) -> Option<&PlayerRef> {
        self.players.get(&id)
    }

    pub fn spawn_player_character(&mut self, world_id: WorldId, player_id: PlayerId, character: Character) {
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
