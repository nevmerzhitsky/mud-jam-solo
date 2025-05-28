use crate::area::WorldId;
use crate::game::Game;
use crate::socium::{Character, CharacterId, Player, PlayerId};

mod action;
mod area;
mod game;
mod socium;

fn main() {
    let mut game = Game::new();

    let mut world_a = area::World::new(WorldId::from(1), String::from("First world"));
    world_a.fill_area();
    println!(">>> world = {:#?}", world_a);

    let world_a_id = game.add_world(world_a);

    let char_a = Character::new(CharacterId::from(11));
    let player_a = Player::new(PlayerId::from(21));
    println!(">>> player_a in VOID = {:#?}", player_a);

    let player_a_id = game.add_player(player_a);
    game.spawn_player_character(world_a_id, player_a_id, char_a);

    let player_a = game.get_player(player_a_id).unwrap();
    println!(">>> player_a after SPAWN = {:#?}", player_a);

    loop {
        // Process events

        let world_a = game.get_world(world_a_id).unwrap();
        // Execute a user action
        let command = action::ask_command_as_action();
        command.execute(&world_a, 1);
    }
}
