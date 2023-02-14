use enigo::*;

pub enum ButtonAction {
    Press,
    Release,
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
    Wait(u64)
}

enum ParseResult {
    Fail,
    Success
}

impl ParsedCommand {
    fn parse_key_sequence(cmd_string: &str) -> (ParsedCommand, ParseResult) {
        if !cmd_string.is_empty()
        {
            return (ParsedCommand::KeySequence(String::from(cmd_string)), ParseResult::Success);
        }

        return (ParsedCommand::Wait(1), ParseResult::Fail);
    }

    fn parse_layout_key(cmd_string: &str) -> (ParsedCommand, ParseResult) {
        let split_line_key_and_action: Vec<&str> = cmd_string.split(" ").collect();

        if split_line_key_and_action.len() == 2 {
            let parse_result_char = split_line_key_and_action[0].parse::<char>();
            let parsed_button_action = split_line_key_and_action[1];

            let mut button_action: ButtonAction = ButtonAction::None;
            if parsed_button_action == "release" {
                button_action = ButtonAction::Release;
            }
            else if parsed_button_action == "press" {
                button_action = ButtonAction::Press;
            }

            if let Ok(key) = parse_result_char {
                match button_action
                {
                    ButtonAction::None => {},
                    _ => { return (ParsedCommand::LayoutKeyUse(key, button_action), ParseResult::Success);}
                }
            }
        }

        return (ParsedCommand::Wait(1), ParseResult::Fail);
    }

    fn parse_wait(cmd_string: &str) -> (ParsedCommand, ParseResult) {
        let parse_result = cmd_string.parse::<u64>();

        if let Ok(wait_time) = parse_result {
            return (ParsedCommand::Wait(wait_time), ParseResult::Success);
        }

        return (ParsedCommand::Wait(1), ParseResult::Fail);
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
            let parse_result_x = split_line_coordinates[0].parse::<i32>();
            let parse_result_y = split_line_coordinates[1].parse::<i32>();

            if let (Ok(x), Ok(y)) = (parse_result_x, parse_result_y) {
                return (ParsedCommand::MouseMove{x, y}, ParseResult::Success);
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
        else if line.starts_with("layout_key: ") {
            parse_fn = ParsedCommand::parse_layout_key;
            beginning_sequence = "layout_key: ";
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
        
        println!("parse_command_from_line: read {}", beginning_sequence);

        let split_line: Vec<&str> = line.split(beginning_sequence).collect();

        for split in &split_line {
            println!("{}", split);
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
            println!("The line is not formatted properly, not using it as a command");
            return ParsedCommand::Wait(1);
        },
        _ => {
            return return_parse;
        }
    }
}