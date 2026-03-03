use libxdo::XDo;
mod config;
use clap::Parser;
use config::{RemapConfig, load_config};
use sdl2::controller::{Button, GameController};
use sdl2::event::Event;
use std::{collections::HashMap, path::PathBuf, process::ExitCode};

// Based from basic example: https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/game-controller.rs
/// Simple program to inject transform gamepad inputs into keystrokes by injecting into selected
/// X11 display
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Config file to use for remapping binds
    config_file: PathBuf,

    /// Display index to inject keystrokes to - example :1
    #[arg(long)]
    display: String,
}

enum KeyPress {
    Release,
    Press,
}

fn handle_button(button: Button, remap_config: &RemapConfig, xdo_handle: &XDo, keypress: KeyPress) {
    if let Some(key) = remap_config.get_key(&button) {
        let result = match keypress {
            KeyPress::Press => xdo_handle.send_keysequence_down(key, 0),
            KeyPress::Release => xdo_handle.send_keysequence_up(key, 0),
        };

        if let Err(e) = result {
            eprintln!("Failed to send key event: {:?}", e);
        }
    }
}

fn _main() -> Result<(), String> {
    let args = Args::parse();
    let remap_config = load_config(&args.config_file)?;

    // open xdo
    let xdo_handle = XDo::new(Some(&args.display)).map_err(|err| err.to_string())?;

    let sdl_context = sdl2::init()?;
    let game_controller_subsystem = sdl_context.game_controller()?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut controllers: HashMap<u32, GameController> = HashMap::new();
    game_controller_subsystem.set_event_state(true);

    println!("Waiting for gamepad");
    for event in event_pump.wait_iter() {
        match event {
            Event::ControllerDeviceAdded { which, .. } => {
                println!("Controller added at index {}", which);
                if game_controller_subsystem.is_game_controller(which) {
                    match game_controller_subsystem.open(which) {
                        Ok(controller) => {
                            let id = controller.instance_id();
                            println!(
                                "Opened controller: \"{}\" (instance id: {})",
                                controller.name(),
                                id
                            );
                            controllers.insert(id, controller);
                        }
                        Err(e) => println!("Failed to open controller: {}", e),
                    }
                }
            }
            Event::ControllerDeviceRemoved { which, .. } => {
                println!("Controller removed (instance id: {})", which);
                controllers.remove(&which);
            }
            Event::ControllerButtonDown {
                which: _, button, ..
            } => {
                handle_button(button, &remap_config, &xdo_handle, KeyPress::Press);
            }
            Event::ControllerButtonUp {
                which: _, button, ..
            } => {
                handle_button(button, &remap_config, &xdo_handle, KeyPress::Release);
            }
            Event::Quit { .. } => break,
            _ => {}
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    match _main() {
        Ok(()) => return ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}", err);
            return ExitCode::FAILURE;
        }
    }
}
