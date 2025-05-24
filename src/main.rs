use crate::socium::{Character, Player};

mod action;
mod area;
mod socium;

fn main() {
    let char_a = Character::new(1);
    let player_a = Player::new(1);
    println!(">>> player_a in VOID = {:#?}", player_a);

    let mut world_a = area::World::new(1, String::from("First world"));
    world_a.fill_area();
    println!(">>> world = {:#?}", world_a);

    let player_a = world_a.spawn_player_character(player_a, char_a);
    println!(">>> player_a after SPAWN = {:#?}", player_a);

    loop {
        // Process events

        // Execute a user action
        let command = action::ask_command_as_action();
        command.execute(&world_a, 1);
    }
}
