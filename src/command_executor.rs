use enigo::*;
use crate::command_types::*; 

pub fn execute_commands(command_vector: &Vec<ParsedCommand>) {
    let mut enigo = Enigo::new();

    for parsed_command in command_vector{
        match parsed_command{
            ParsedCommand::LayoutKeyUse(key, button_action) => { 
                match button_action {
                    ButtonAction::Press => {
                        enigo.key_down(Key::Layout(*key));
                    },
                    ButtonAction::Release => {
                        enigo.key_up(Key::Layout(*key));
                    },
                    ButtonAction::Click => {
                        enigo.key_click(Key::Layout(*key));
                    },
                    ButtonAction::None => { println!("This should not happen! Doing nothing"); }
                }
             },
            ParsedCommand::SpecialKeyUse(key, button_action) => { 
                match button_action {
                    ButtonAction::Press => {
                        enigo.key_down(*key);
                    },
                    ButtonAction::Release => {
                        enigo.key_up(*key);
                    },
                    ButtonAction::Click => {
                        enigo.key_click(*key);
                    },
                    ButtonAction::None => { println!("This should not happen! Doing nothing"); }
                } 
            },
            ParsedCommand::KeySequence(sequence) => {
                enigo.key_sequence(sequence.as_str());
            },
            ParsedCommand::MouseClick(mouse_button) => {
                enigo.mouse_click(*mouse_button);
            },
            ParsedCommand::MouseDown(mouse_button) => {
                enigo.mouse_down(*mouse_button);
            },
            ParsedCommand::MouseRelease(mouse_button) => {
                enigo.mouse_up(*mouse_button);
            },
            ParsedCommand::MouseMove{x, y} => {
                enigo.mouse_move_to(*x, *y);
            },
            ParsedCommand::Wait(wait_time_ms) => {
                let wait_duration = std::time::Duration::from_millis(*wait_time_ms);
                std::thread::sleep(wait_duration);
            }
        }
    }
}