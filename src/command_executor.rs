use enigo::*;
use log::{error, info};

use crate::command_types::*; 
use crate::screen_compare::*;

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
                    ButtonAction::None => { error!(target: "commands_debug", "This should not happen! Doing nothing"); }
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
                    ButtonAction::None => { error!(target: "commands_debug", "This should not happen! Doing nothing"); }
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
            ParsedCommand::MouseMoveRelative{x, y} => {
                enigo.mouse_move_relative(*x, *y);
            },
            ParsedCommand::Wait(wait_time_ms) => {
                let wait_duration = std::time::Duration::from_millis(*wait_time_ms);
                std::thread::sleep(wait_duration);
            },
            ParsedCommand::ScreenCompareLayoutKeyClick{layout_key, 
                input_file_path, 
                start_x,
                start_y,
                screen_capture_width,
                screen_capture_height,
                match_threshold} => {
                
                    let match_percentage = compare_screen_to_image_file(input_file_path, 
                        *start_x,
                        *start_y,
                        *screen_capture_width,
                        *screen_capture_height);
                
                    if *match_threshold <= (match_percentage * 100.0) { 
                        info!(target: "commands_debug", "successful match_percentage = {}, match_threadhold_percentage = {}", (match_percentage * 100.0), match_threshold); 
                        enigo.key_click(Key::Layout(*layout_key));
                    }
            },
            ParsedCommand::ScreenCompareFunctionKeyClick{function_key, 
                input_file_path, 
                start_x,
                start_y,
                screen_capture_width,
                screen_capture_height,
                match_threshold} => {
                
                    let match_percentage = compare_screen_to_image_file(input_file_path, 
                        *start_x,
                        *start_y,
                        *screen_capture_width,
                        *screen_capture_height);

                    if *match_threshold <= (match_percentage * 100.0) { 
                        info!(target: "commands_debug", "successful match_percentage = {}, match_threadhold_percentage = {}", (match_percentage * 100.0), match_threshold); 
                        enigo.key_click(*function_key);
                    }
            }
        }
    }
}