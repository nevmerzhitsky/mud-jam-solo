use mud_jam_solo::action::ask_command;
use mud_jam_solo::area::World;

fn main() {
    let world_a = World::new(1, "First world".to_string());
    println!(">>> world = {:?}", world_a);

    loop {
        // Process events

        // Ask user an action
        let command = ask_command();
        command.execute(&world_a, 1);
    }
}
