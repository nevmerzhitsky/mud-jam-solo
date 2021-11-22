use std::io;

trait UserAction {
    fn execute(&self);
}

struct MoveNorth {}

struct MoveSouth {}

impl UserAction for MoveNorth {
    fn execute(&self) {
        println!("Moving to the north...",);
    }
}

impl UserAction for MoveSouth {
    fn execute(&self) {
        println!("Moving to the south...");
    }
}

fn main() {
    loop {
        // Process events

        // Ask user an action
        let command = ask_command();
        command.execute();
    }
}

fn ask_command() -> impl UserAction {
    println!("Your action? ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    println!("You enter: {}", input);

    // TODO Use https://docs.rs/crate/clap/ as mapper of all available command and as input parser
    // TODO Return Command instance: input + action instance
    MoveNorth {}
}
