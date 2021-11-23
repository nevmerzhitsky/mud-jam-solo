mod area {
    pub trait Object {
        fn get_id(&self) -> u32;
    }

    pub struct World {
        id: u32,
        name: String,
    }

    impl Object for World {
        fn get_id(&self) -> u32 {
            self.id
        }
    }
}

mod socium {
    pub struct Character {
        id: u32,
    }

    pub struct Player {
        char: Character,
    }
}

pub mod action {
    use std::io;

    pub trait UserAction {
        fn execute(&self);
    }

    #[derive(Debug)]
    pub struct MoveNorth {}

    #[derive(Debug)]
    pub struct MoveSouth {}

    impl UserAction for MoveNorth {
        fn execute(&self) {
            println!("Moving to the north...");
        }
    }

    impl UserAction for MoveSouth {
        fn execute(&self) {
            println!("Moving to the south...");
        }
    }

    pub fn ask_command() -> impl UserAction {
        println!("Your action? ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        println!("You enter: {}", input);

        // TODO Use clap lib as the input parser to all available commands
        // TODO Return Command instance: input + action instance
        MoveNorth {}
    }
}
