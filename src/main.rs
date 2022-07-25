mod action;
mod area;
mod socium;

fn main() {
    let mut world_a = area::World::new(1, String::from("First world"));
    world_a.fill_area();
    println!(">>> world = {:#?}", world_a);

    loop {
        // Process events

        // Execute a user action
        let command = action::ask_command();
        command.execute(&world_a, 1);
    }
}
