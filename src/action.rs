use crate::area::{MoveDirection, Room, RoomExit, WorldRef};
use crate::game::{Game, PlayerId};
use crate::socium::CharacterRef;
use std::io;
use std::ops::Deref;
use std::rc::Rc;

// ----------------------------------------------------------------------------------------------------
// Commands via traits
// ----------------------------------------------------------------------------------------------------

pub trait CharAction {
    fn execute(&self, game: &mut Game, subject_id: PlayerId);
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
pub struct MoveWest {}

#[derive(Debug)]
pub struct MoveEast {}

#[derive(Debug)]
pub struct MoveUp {}

#[derive(Debug)]
pub struct MoveDown {}

#[derive(Debug)]
pub struct Say {
    params: Vec<String>,
}

impl CharAction for UnknownCommand {
    fn execute(&self, _game: &mut Game, _subject_id: PlayerId) {
        println!("Unknown command")
    }
}

impl CharAction for Empty {
    fn execute(&self, _game: &mut Game, _subject_id: PlayerId) {}
}

impl CharAction for Quit {
    fn execute(&self, _game: &mut Game, _subject_id: PlayerId) {
        panic!("You decided to quit")
    }
}

impl CharAction for MoveNorth {
    fn execute(&self, game: &mut Game, subject_id: PlayerId) {
        move_to_direction(game, subject_id, MoveDirection::North);
    }
}

impl CharAction for MoveSouth {
    fn execute(&self, game: &mut Game, subject_id: PlayerId) {
        move_to_direction(game, subject_id, MoveDirection::South);
    }
}

impl CharAction for MoveWest {
    fn execute(&self, game: &mut Game, subject_id: PlayerId) {
        move_to_direction(game, subject_id, MoveDirection::West);
    }
}

impl CharAction for MoveEast {
    fn execute(&self, game: &mut Game, subject_id: PlayerId) {
        move_to_direction(game, subject_id, MoveDirection::East);
    }
}

impl CharAction for MoveUp {
    fn execute(&self, game: &mut Game, subject_id: PlayerId) {
        move_to_direction(game, subject_id, MoveDirection::Up);
    }
}

impl CharAction for MoveDown {
    fn execute(&self, game: &mut Game, subject_id: PlayerId) {
        move_to_direction(game, subject_id, MoveDirection::Down);
    }
}


impl CharAction for Say {
    fn execute(&self, _game: &mut Game, _subject_id: PlayerId) {
        println!("SAY: {:?}", self.params);
    }
}

fn move_to_direction(game: &mut Game, subject_id: PlayerId, direction: MoveDirection) {
    let free_char;

    {
        let player = game.get_player(subject_id).unwrap();
        let player_ref = player.borrow();
        let char = player_ref.get_main_char();

        if char.is_none() {
            println!("You have no physical body!");
            return;
        }

        let Some(char) = char else { unreachable!() };
        free_char = char.clone();
    }

    let char_ref = free_char.borrow();

    let current_room = char_ref.get_current_room();

    if current_room.is_none() {
        println!("You are nowhere and cannot step!");
        return;
    }

    let Some(current_room) = current_room else { unreachable!() };

    match current_room.get_exit(&direction).borrow().deref() {
        RoomExit::DeadEnd => {
            println!("You cannot go this way!");
        },
        RoomExit::Pathway(weak_to_room) => {
            match weak_to_room.upgrade() {
                None => println!("The game is failed to moving you :("),
                Some(to_room) => {
                    let world = current_room.get_world().clone();
                    let world_ref = world.borrow();
                    let char = world_ref.get_character(char_ref.get_id()).unwrap();

                    println!("${:?} moving {:?}...", subject_id, direction);
                    game.queue_action(GameAction::WalkFromTo {
                        who: char.clone(),
                        from: current_room.clone(),
                        to: to_room.clone(),
                    });
                }
            }
        },
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

    command_to_character_action(input)
}

fn command_to_character_action(input: String) -> Box<dyn CharAction> {
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
        "west" => Box::new(MoveWest {}),
        "east" => Box::new(MoveEast {}),
        "up" => Box::new(MoveUp {}),
        "down" => Box::new(MoveDown {}),
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
    MoveWest,
    MoveEast,
    MoveUp,
    MoveDown,
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
            Command::MoveWest => println!("mw"),
            Command::MoveEast => println!("me"),
            Command::MoveUp => println!("mu"),
            Command::MoveDown => println!("md"),
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
        "west" => Command::MoveWest,
        "east" => Command::MoveEast,
        "up" => Command::MoveUp,
        "down" => Command::MoveDown,
        "say" => Command::Say(params),
        _ => Command::UnknownCommand,
    }
}

// ----------------------------------------------------------------------------------------------------
// Actions
// ----------------------------------------------------------------------------------------------------

pub enum GameAction {
    WalkFromTo { who: CharacterRef, from: Rc<Room>, to: Rc<Room> },
}
