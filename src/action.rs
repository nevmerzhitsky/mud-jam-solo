use std::io;

use crate::area::WorldRef;

// ----------------------------------------------------------------------------------------------------
// Commands via traits
// ----------------------------------------------------------------------------------------------------

pub trait CharAction {
    fn execute(&self, w: &WorldRef, subject_id: u32);
}

#[derive(Debug)]
pub struct UnknownCommand {}

#[derive(Debug)]
pub struct Empty {}

#[derive(Debug)]
pub struct Quit {}

#[derive(Debug)]
pub struct MoveNorth {}

#[derive(Debug)]
pub struct MoveSouth {}

#[derive(Debug)]
pub struct Say {
    params: Vec<String>,
}

impl CharAction for UnknownCommand {
    fn execute(&self, _w: &WorldRef, _subject_id: u32) {
        println!("Unknown command")
    }
}

impl CharAction for Empty {
    fn execute(&self, _w: &WorldRef, _subject_id: u32) {}
}

impl CharAction for Quit {
    fn execute(&self, _w: &WorldRef, _subject_id: u32) {
        panic!("You decided to quit")
    }
}

impl CharAction for MoveNorth {
    fn execute(&self, _w: &WorldRef, subject_id: u32) {
        println!("#{:?} moving to the north...", subject_id);
    }
}

impl CharAction for MoveSouth {
    fn execute(&self, _w: &WorldRef, subject_id: u32) {
        println!("#{:?} moving to the south...", subject_id);
    }
}

impl CharAction for Say {
    fn execute(&self, _w: &WorldRef, _subject_id: u32) {
        println!("SAY: {:?}", self.params);
    }
}

pub fn ask_command_as_action() -> Box<dyn CharAction> {
    println!("Your action? ");

    let mut input = String::with_capacity(200);
    // input.clear();

    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("> {n} bytes read: {input}");
        }
        Err(error) => println!("error: {error}"),
    }

    command_to_action(input)
}

fn command_to_action(input: String) -> Box<dyn CharAction> {
    let mut words = input.split_whitespace();
    let command = words.next().unwrap_or("").to_ascii_lowercase();
    let params: Vec<String> = words.map(|m| m.to_string()).collect();

    println!(">>> Command: \"{}\" {:?}", command, params);

    // TODO Use clap library as the input parser to all available commands
    // TODO Return Command instance: input + action instance
    match command.as_str() {
        "" => Box::new(Empty {}),
        "quit" => Box::new(Quit {}),
        "north" => Box::new(MoveNorth {}),
        "south" => Box::new(MoveSouth {}),
        "say" => Box::new(Say { params }),
        _ => Box::new(UnknownCommand {}),
    }
}

// ----------------------------------------------------------------------------------------------------
// Commands via enum
// ----------------------------------------------------------------------------------------------------

pub enum Command {
    UnknownCommand,
    Empty,
    Quit,
    MoveNorth,
    MoveSouth,
    Say(Vec<String>),
}

impl Command {
    pub fn execute(&self, _w: &WorldRef, _subject_id: u32) {
        print!("command: ");
        match self {
            Command::UnknownCommand => println!("u"),
            Command::Empty => {}
            Command::Quit => println!("q"),
            Command::MoveNorth => println!("mn"),
            Command::MoveSouth => println!("ms"),
            Command::Say(params) => println!("s: {:?}", params),
        }
    }
}

pub fn ask_command_as_enum() -> Command {
    println!("Your action? ");

    let mut input = String::with_capacity(200);
    // input.clear();

    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("> {n} bytes read: {input}");
        }
        Err(error) => println!("error: {error}"),
    }

    command_to_enum(input)
}

fn command_to_enum(input: String) -> Command {
    let mut words = input.split_whitespace();
    let command = words.next().unwrap_or("").to_ascii_lowercase();
    let params: Vec<String> = words.map(|m| m.to_string()).collect();

    println!(">>> Command: \"{}\" {:?}", command, params);

    // TODO Use clap library as the input parser to all available commands
    // TODO Return Command instance: input + action instance
    match command.as_str() {
        "" => Command::Empty,
        "quit" => Command::Quit,
        "north" => Command::MoveNorth,
        "south" => Command::MoveSouth,
        "say" => Command::Say(params),
        _ => Command::UnknownCommand,
    }
}
