use configparser::ini::Ini;
use sdl2::controller::Button;
use std::{collections::HashMap, path::Path};

const CONFIG_TOP_LEVEL_SECTION: &str = "gamepad2key";

pub struct RemapConfig {
    mapping: HashMap<Button, String>,
}

impl RemapConfig {
    pub fn get_key<'a>(&'a self, button: &Button) -> Option<&'a str> {
        self.mapping.get(button).map(|s| s.as_str())
    }
}

// see: for list of keycodes https://github.com/sickcodes/xdotool-gui/blob/master/key_list.csv
pub fn load_config(filepath: &Path) -> Result<RemapConfig, String> {
    let mut config = Ini::new();
    config.load(filepath)?;

    let Some(bindingmap) = config.get_map_ref().get(CONFIG_TOP_LEVEL_SECTION) else {
        return Err(format!(
            "Invalid config file - Toplevel Section \"{}\" not found!",
            CONFIG_TOP_LEVEL_SECTION
        ));
    };

    // transform each binding into a button map should it match
    let buttonmap: HashMap<Button, String> = bindingmap
        .iter()
        .map(|(key, value)| {
            let button = string_to_button(key)
                .ok_or_else(|| format!("Invalid button name in config: {}", key))?;

            let mapkey = value
                .as_ref()
                .ok_or_else(|| format!("No key defined for button: {}", key))?;

            if mapkey.trim().is_empty() {
                return Err(format!("Key for button '{}' is empty", key));
            }

            Ok((button, mapkey.clone()))
        })
        .collect::<Result<HashMap<_, _>, String>>()?;

    // TODO: Somehow validate values? For now it's fine.
    return Ok(RemapConfig { mapping: buttonmap });
}

fn string_to_button(input: &str) -> Option<Button> {
    match input.to_ascii_lowercase().as_str() {
        "a" => Some(Button::A),
        "b" => Some(Button::B),
        "x" => Some(Button::X),
        "y" => Some(Button::Y),
        "back" => Some(Button::Back),
        "select" => Some(Button::Guide),
        "start" => Some(Button::Start),
        "ls" => Some(Button::LeftStick),
        "rs" => Some(Button::RightStick),
        "lb" => Some(Button::LeftShoulder),
        "rb" => Some(Button::RightShoulder),
        "dpadup" => Some(Button::DPadUp),
        "dpaddown" => Some(Button::DPadDown),
        "dpadleft" => Some(Button::DPadLeft),
        "dpadright" => Some(Button::DPadRight),
        "misc1" => Some(Button::Misc1),
        "paddle1" => Some(Button::Paddle1),
        "paddle2" => Some(Button::Paddle2),
        "paddle3" => Some(Button::Paddle3),
        "paddle4" => Some(Button::Paddle4),
        "touchpad" => Some(Button::Touchpad),
        _ => None,
    }
}
