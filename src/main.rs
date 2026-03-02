extern crate libxdo;

use libxdo::XDo;

fn main() {
    let display = String::from(":1");
    let xdo = XDo::new(Some(&display)).unwrap();

    // xdo.send_keysequence_down("a", 0);
    // xdo.send_keysequence_up("a", 0);
    // xdo.enter_text("", 250_000).unwrap();
}
