mod action;
mod area;
mod socium;

fn main() {
    let mut world_a = area::World::new(1, "First world".to_string());
    world_a.feed_area();
    println!(">>> world = {:?}", world_a);

    loop {
        // Process events

        // Ask user an action
        match action::ask_command() {
            Some(c) => c.execute(&world_a, 1),
            None => println!("Unknown command!"),
        }
    }
}
