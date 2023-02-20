use enigo::*;
use phf::phf_map;
use log::{error, info};

pub enum ButtonAction {
    Press,
    Release,
    Click,
    None
}
pub enum ParsedCommand {
    LayoutKeyUse(char, ButtonAction),
    SpecialKeyUse(enigo::Key, ButtonAction),
    KeySequence(String),
    MouseClick(MouseButton),
    MouseDown(MouseButton),
    MouseRelease(MouseButton),
    MouseMove{x: i32, y: i32},
    Wait(u64),
    ScreenCompareLayoutKeyClick{layout_key: char, input_file_path: String, start_x: i32, start_y: i32, screen_capture_width: u32, screen_capture_height: u32, match_threshold: f64},
    ScreenCompareFunctionKeyClick{function_key: enigo::Key, input_file_path: String, start_x: i32, start_y: i32, screen_capture_width: u32, screen_capture_height: u32, match_threshold: f64}
}

enum ParseResult {
    Fail,
    Success
}

static STR_TO_ENIGO_KEY_MAP: phf::Map<&str, enigo::Key> = phf_map! {
    "alt" => Key::Alt,
    "back_space" => Key::Backspace,
    "caps_lock" => Key::CapsLock,
    "control" => Key::Control,
    "delete" => Key::Delete,
    "down_arrow" => Key::DownArrow,
    "end" => Key::End,
    "escape" => Key::Escape,
    "f1" => Key::F1,
    "f2" => Key::F2,
    "f3" => Key::F3,
    "f4" => Key::F4,
    "f5" => Key::F5,
    "f6" => Key::F6,
    "f7" => Key::F7,
    "f8" => Key::F8,
    "f9" => Key::F9,
    "f10" => Key::F10,
    "f11" => Key::F11,
    "f12" => Key::F12,
    "f13" => Key::F13,
    "f14" => Key::F14,
    "f15" => Key::F15,
    "f16" => Key::F16,
    "f17" => Key::F17,
    "f18" => Key::F18,
    "f19" => Key::F19,
    "f20" => Key::F20,
    "home" => Key::Home,
    "left_arrow" => Key::LeftArrow,
    "meta" => Key::Meta,
    "option" => Key::Option,
    "page_down" => Key::PageDown,
    "page_up" => Key::PageUp,
    "return" => Key::Return,
    "right_arrow" => Key::RightArrow,
    "shift" => Key::Shift,
    "space" => Key::Space,
    "tab" => Key::Tab,
    "up_arrow" => Key::UpArrow
};


impl ParsedCommand {
    fn parse_key_sequence(cmd_string: &str) -> (ParsedCommand, ParseResult) {
        if !cmd_string.is_empty()
        {
            return (ParsedCommand::KeySequence(String::from(cmd_string)), ParseResult::Success);
        }

        return (ParsedCommand::Wait(1), ParseResult::Fail);
    }

    fn parse_key(cmd_string: &str) -> (ParsedCommand, ParseResult) {
        let split_line_key_and_action: Vec<&str> = cmd_string.split(" ").collect();

        if split_line_key_and_action.len() == 2 {            
            let parsed_button_action = split_line_key_and_action[1];

            let mut button_action: ButtonAction = ButtonAction::None;
            if parsed_button_action == "release" {
                button_action = ButtonAction::Release;
            }
            else if parsed_button_action == "press" {
                button_action = ButtonAction::Press;
            }
            else if parsed_button_action == "click" {
                button_action = ButtonAction::Click;
            }

            match button_action
            {
                ButtonAction::None => {},
                _ => {
                    let parse_char_result = split_line_key_and_action[0].parse::<char>();

                    if let Ok(parsed_char) = parse_char_result {
                        return (ParsedCommand::LayoutKeyUse(parsed_char, button_action), ParseResult::Success);
                    }
                    else if let Some(enigo_key) = STR_TO_ENIGO_KEY_MAP.get(split_line_key_and_action[0]) {
                        return (ParsedCommand::SpecialKeyUse(*enigo_key, button_action), ParseResult::Success);
                    }
                }
            }
        }

        return (ParsedCommand::Wait(1), ParseResult::Fail);
    }

    fn parse_wait(cmd_string: &str) -> (ParsedCommand, ParseResult) {
        let wait_time = cmd_string.parse::<u64>().expect("");

        return (ParsedCommand::Wait(wait_time), ParseResult::Success);
    }

    fn parse_mouse_click(cmd_string: &str) -> (ParsedCommand, ParseResult) {
        let button_specifier = String::from(cmd_string);
        if button_specifier == "left" {
            return (ParsedCommand::MouseClick(MouseButton::Left), ParseResult::Success);
        }
        if button_specifier == "right" {
            return (ParsedCommand::MouseClick(MouseButton::Right), ParseResult::Success);
        }
        if button_specifier == "middle" {
            return (ParsedCommand::MouseClick(MouseButton::Middle), ParseResult::Success);
        }

        return (ParsedCommand::Wait(1), ParseResult::Fail);
    }

    fn parse_mouse_down(cmd_string: &str) -> (ParsedCommand, ParseResult) {
        let button_specifier = String::from(cmd_string);
        if button_specifier == "left" {
            return (ParsedCommand::MouseDown(MouseButton::Left), ParseResult::Success);
        }
        if button_specifier == "right" {
            return (ParsedCommand::MouseDown(MouseButton::Right), ParseResult::Success);
        }
        if button_specifier == "middle" {
            return (ParsedCommand::MouseDown(MouseButton::Middle), ParseResult::Success);
        }

        return (ParsedCommand::Wait(1), ParseResult::Fail);
    }

    fn parse_mouse_release(cmd_string: &str) -> (ParsedCommand, ParseResult) {
        let button_specifier = String::from(cmd_string);
        if button_specifier == "left" {
            return (ParsedCommand::MouseRelease(MouseButton::Left), ParseResult::Success);
        }
        if button_specifier == "right" {
            return (ParsedCommand::MouseRelease(MouseButton::Right), ParseResult::Success);
        }
        if button_specifier == "middle" {
            return (ParsedCommand::MouseRelease(MouseButton::Middle), ParseResult::Success);
        }

        return (ParsedCommand::Wait(1), ParseResult::Fail);
    }

    fn parse_mouse_move(cmd_string: &str) -> (ParsedCommand, ParseResult) {
        let split_line_coordinates: Vec<&str> = cmd_string.split(" ").collect();

        if split_line_coordinates.len() == 2 {
            let x = split_line_coordinates[0].parse::<i32>().expect("");
            let y = split_line_coordinates[1].parse::<i32>().expect("");

            return (ParsedCommand::MouseMove{x, y}, ParseResult::Success);
        }

        return (ParsedCommand::Wait(1), ParseResult::Fail);
    }

    fn parse_screen_compare_key_click(cmd_string: &str) -> (ParsedCommand, ParseResult) {
        let split_line: Vec<&str> = cmd_string.split(" ").collect();

        info!(target: "commands_debug", "parse_screen_compare_key_click: Number of Tokens = {}", split_line.len());

        if split_line.len() >= 7 {
            let start_x = split_line[1].parse::<i32>().expect("start_x read failure");
            let start_y = split_line[2].parse::<i32>().expect("start_y read failure");
            let screen_capture_width = split_line[3].parse::<u32>().expect("screen_capture_width read failure");
            let screen_capture_height = split_line[4].parse::<u32>().expect("screen_capture_height read failure");
            let match_threshold = split_line[5].parse::<f64>().expect("match_threshold read failure");

            // Handle in case of an input path which includes spaces
            let mut input_file_path: String = split_line[6].to_string();
            for i in 7..split_line.len() {
                input_file_path += " ";
                input_file_path += split_line[i];
            }

            let parse_char_result = split_line[0].parse::<char>();

            if let Ok(layout_key) = parse_char_result {
                return (ParsedCommand::ScreenCompareLayoutKeyClick{layout_key, 
                                                                    input_file_path, 
                                                                    start_x, 
                                                                    start_y, 
                                                                    screen_capture_width, 
                                                                    screen_capture_height, 
                                                                    match_threshold}, ParseResult::Success);
            }
            else if let Some(function_key) = STR_TO_ENIGO_KEY_MAP.get(split_line[0]) {

                return (ParsedCommand::ScreenCompareFunctionKeyClick{function_key: *function_key, 
                                                                        input_file_path, 
                                                                        start_x, 
                                                                        start_y, 
                                                                        screen_capture_width,
                                                                        screen_capture_height, 
                                                                        match_threshold}, ParseResult::Success);
            }
        }

        return (ParsedCommand::Wait(1), ParseResult::Fail);
    }

    fn parse_command_substring(line: &String) -> (ParsedCommand, ParseResult) {
        let mut parse_fn: fn(&str) -> (ParsedCommand, ParseResult) = ParsedCommand::parse_wait;
        let mut beginning_sequence: &str = "";
        
        if line.starts_with("key_sequence: ") {
            parse_fn = ParsedCommand::parse_key_sequence;
            beginning_sequence = "key_sequence: ";
        }
        else if line.starts_with("key: ") {
            parse_fn = ParsedCommand::parse_key;
            beginning_sequence = "key: ";
        }
        else if line.starts_with("wait: ") {
            parse_fn = ParsedCommand::parse_wait;
            beginning_sequence = "wait: ";
        }
        else if line.starts_with("mouse_click: ") {
            parse_fn = ParsedCommand::parse_mouse_click;
            beginning_sequence = "mouse_click: ";
        }
        else if line.starts_with("mouse_down: ") {
            parse_fn = ParsedCommand::parse_mouse_down;
            beginning_sequence = "mouse_down: ";
        }
        else if line.starts_with("mouse_release: ") {
            parse_fn = ParsedCommand::parse_mouse_release;
            beginning_sequence = "mouse_release: ";
        }
        else if line.starts_with("mouse_move: ") {
            parse_fn = ParsedCommand::parse_mouse_move;
            beginning_sequence = "mouse_move: ";
        }
        else if line.starts_with("screen_compare_key_click: ") {
            parse_fn = ParsedCommand::parse_screen_compare_key_click;
            beginning_sequence = "screen_compare_key_click: ";
        }
        
        info!(target: "commands_debug", "parse_command_from_line: read {}", beginning_sequence);

        let split_line: Vec<&str> = line.split(beginning_sequence).collect();

        for split in &split_line {
            info!(target: "commands_debug", "{}", split);
        }

        if split_line.len() == 2 {
            return parse_fn(split_line[1]);
        }

        return (ParsedCommand::Wait(1), ParseResult::Fail);
    }

}

pub fn parse_command_from_line(line: &String) -> ParsedCommand{
    let (return_parse, parse_result) = ParsedCommand::parse_command_substring(line);

    match parse_result{
        ParseResult::Fail => {
            error!(target: "commands_debug", "The line is not formatted properly, not using it as a command");
            return ParsedCommand::Wait(1);
        },
        _ => {
            return return_parse;
        }
    }
}