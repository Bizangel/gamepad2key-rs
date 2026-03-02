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
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let game_controller_subsystem = sdl_context.game_controller()?;

    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;

    println!("{} joysticks available", available);

    // Iterate over all available joysticks and look for game controllers.
    let controller = (0..available)
        .find_map(|id| {
            if !game_controller_subsystem.is_game_controller(id) {
                println!("{} is not a game controller", id);
                return None;
            }

            println!("Attempting to open controller {}", id);

            match game_controller_subsystem.open(id) {
                Ok(c) => {
                    // We managed to find and open a game controller,
                    // exit the loop
                    println!("Success: opened \"{}\"", c.name());
                    Some(c)
                }
                Err(e) => {
                    println!("failed: {:?}", e);
                    None
                }
            }
        })
        .expect("Couldn't open any controller");

    println!("Controller mapping: {}", controller.mapping());
    for event in sdl_context.event_pump()?.wait_iter() {
        match event {
            Event::ControllerButtonDown { button, .. } => println!("Button {:?} down", button),
            Event::ControllerButtonUp { button, .. } => println!("Button {:?} up", button),
            Event::Quit { .. } => break,
            _ => (),
        }
    }

    Ok(())
}
