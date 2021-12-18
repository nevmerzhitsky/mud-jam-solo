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

    pub fn ask_command() -> Box<dyn UserAction> {
        println!("Your action? ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        command_to_action(input)
    }

    fn command_to_action(input: String) -> Box<dyn UserAction> {
        let mut words = input.split_whitespace();
        let command = words
            .next()
            .unwrap_or("")
            .to_ascii_lowercase();
        let params: Vec<&str> = words.collect();

        println!(">>> Command: \"{}\" {:?}", command, params);

        // TODO Use clap lib as the input parser to all available commands
        // TODO Return Command instance: input + action instance
        match command.as_str() {
            "quit" => panic!("You decided to quit"),
            "north" => Box::new(MoveNorth {}),
            "south" => Box::new(MoveSouth {}),
            _ => panic!("Unknown command: {}", command),
        }
    }
}
