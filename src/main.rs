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

    {
        // Spawn the current player randomly
        let start_room = world_a.get_any_room();

        let char_a_id = world_a.add_character(char_a);
        let player_a_id = world_a.add_player(player_a);

        let char_a = world_a.get_character(char_a_id).unwrap();
        char_a.borrow_mut().set_current_room(start_room.clone());

        let player_a = world_a.get_player(player_a_id).unwrap();
        player_a.borrow_mut().set_main_char(char_a.clone());
        println!(">>> player_a after SPAWN = {:#?}", player_a);

        // TODO Add &char to the room's list.
    }

    loop {
        // Process events

        // Execute a user action
        let command = action::ask_command_as_action();
        command.execute(&world_a, 1);
    }
}
