use crate::action::WorldAction;
use crate::area::{World, WorldId, WorldRef};
use crate::socium::{Character, CharacterId, CharacterRef};
use crate::utils::BuildRef;
use derive_more::From;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub struct Game {
    worlds: HashMap<WorldId, WorldRef>,
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
        self.worlds.insert(id, world.build_ref());

        id
    }

    pub fn get_world(&self, id: WorldId) -> Option<&WorldRef> {
        self.worlds.get(&id)
    }

    pub fn add_player(&mut self, player: Player) -> PlayerId {
        let id = player.get_id();
        self.players.insert(id, player.build_ref());

        id
    }

    pub fn get_player(&self, id: PlayerId) -> Option<&PlayerRef> {
        self.players.get(&id)
    }

    pub fn spawn_player_character(
        &mut self,
        world_id: WorldId,
        player_id: PlayerId,
        character: Character,
    ) {
        let world = self.get_world(world_id).unwrap();
        let char_id = world.borrow_mut().spawn_character(character);

        self.set_player_character(world_id, player_id, char_id);
    }

    pub fn set_player_character(
        &mut self,
        world_id: WorldId,
        player_id: PlayerId,
        char_id: CharacterId,
    ) {
        let world = self.get_world(world_id).unwrap();

        let world_ref = world.borrow();
        let char = world_ref.get_character(char_id).unwrap();
        let char_ref = char.clone();

        let player = self.get_player(player_id).unwrap();
        let player_ref = player.clone();

        player.borrow_mut().set_main_char(char_ref);
        char.borrow_mut().set_owner(player_ref);
    }

    pub fn unset_player_character(
        &mut self,
        world_id: WorldId,
        player_id: PlayerId,
        char_id: CharacterId,
    ) {
        let world = self.get_world(world_id).unwrap();

        let world_ref = world.borrow();
        let char = world_ref.get_character(char_id).unwrap();
        let player = self.get_player(player_id).unwrap();

        player.borrow_mut().unset_main_char();
        char.borrow_mut().unset_owner();
    }

    pub fn queue_action(&mut self, action: WorldAction) {
        self.actions_queue.push_back(action);
    }

    pub fn process_actions_queue(&mut self) {
        //for action in self.actions_queue.drain(..) {}
    }
}

// ----------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, From)]
pub struct PlayerId(u32);

#[derive(Debug)]
pub struct Player {
    id: PlayerId,
    main_char: Option<CharacterRef>,
}

impl Player {
    pub fn new(id: PlayerId) -> Self {
        Self {
            id,
            main_char: None,
        }
    }

    pub fn get_id(&self) -> PlayerId {
        self.id
    }

    pub fn get_main_char(&self) -> &Option<CharacterRef> {
        &self.main_char
    }

    pub fn set_main_char(&mut self, char: CharacterRef) {
        self.main_char = Some(char);
    }

    pub fn unset_main_char(&mut self) {
        self.main_char = None;
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Player {}

impl BuildRef for Player {
    fn build_ref(self) -> PlayerRef {
        Rc::new(RefCell::new(self))
    }
}

pub type PlayerRef = Rc<RefCell<Player>>;
