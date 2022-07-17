mod action;
mod area;
mod socium;

fn main() {
    let mut world_a = area::World::new(1, "First world".to_string());
    world_a.feed_area();
    println!(">>> world = {:?}", world_a);

    loop {
        // Process events

        // Execute a user action
        let command = action::ask_command();
        command.execute(&world_a, 1);
    }
}
