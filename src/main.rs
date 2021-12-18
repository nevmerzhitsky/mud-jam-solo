mod action;
mod area;
mod socium;

fn main() {
    let world_a = area::World::new(1, "First world".to_string());
    println!(">>> world = {:?}", world_a);

    loop {
        // Process events

        // Ask user an action
        let command = action::ask_command();
        command.execute(&world_a, 1);
    }
}
