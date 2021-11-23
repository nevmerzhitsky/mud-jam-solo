use mud_jam_solo::action::{ask_command, UserAction};

fn main() {
    loop {
        // Process events

        // Ask user an action
        let command = ask_command();
        command.execute();
    }
}
