// use libxdo::XDo;
use sdl2::event::Event;

// fn main() {
//     let display = String::from(":1");
//     let xdo = XDo::new(Some(&display)).unwrap();

//     // xdo.send_keysequence_down("a", 0);
//     // xdo.send_keysequence_up("a", 0);
//     // xdo.enter_text("", 250_000).unwrap();
// }

// See full reference exmaple here: https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/game-controller.rs
use sdl2::controller::GameController;
use std::collections::HashMap;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let game_controller_subsystem = sdl_context.game_controller()?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut controllers: HashMap<u32, GameController> = HashMap::new();

    // Enable controller events explicitly (usually enabled by default)
    game_controller_subsystem.set_event_state(true);

    println!("Waiting for controllers...");

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

            Event::ControllerButtonDown { which, button, .. } => {
                println!("Controller {} button {:?} down", which, button);
            }

            Event::ControllerButtonUp { which, button, .. } => {
                println!("Controller {} button {:?} up", which, button);
            }

            Event::Quit { .. } => break,

            _ => {}
        }
    }

    Ok(())
}
