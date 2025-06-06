mod action;
mod area;
mod game;
mod socium;
mod utils;

use crate::area::WorldId;
use crate::game::{Game, Player, PlayerId};
use crate::socium::{Character, CharacterId};

fn main() {
    let mut game = Game::new();

    let world_a = area::World::new(WorldId::from(1), String::from("First world"));

    let world_a_id = game.add_world(world_a);
    game.fill_world(world_a_id);
    println!(">>> world = {:#?}", game.get_world(world_a_id).unwrap());

    let char_a = Character::new(CharacterId::from(11));
    let player_a = Player::new(PlayerId::from(21));
    println!(">>> player_a in VOID = {:#?}", player_a);

    let player_a_id = game.add_player(player_a);
    game.spawn_player_character(world_a_id, player_a_id, char_a);

    let player_a = game.get_player(player_a_id).unwrap();
    println!(">>> player_a after SPAWN = {:#?}", player_a);
    let player_a = player_a.borrow().get_id();

    loop {
        // Process events

        // Execute a user action
        let command = action::ask_command_as_action();
        command.execute(&mut game, player_a);
    }
}
