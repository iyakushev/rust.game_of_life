extern crate piston_window;

mod cell;
pub mod game_field;

use piston_window::*;
use game_field::GAMEFIELD;

fn render() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}


fn main() -> std::io::Result<()> {
    let mut gf = GAMEFIELD::new();
    gf.read_file("example.map".to_string())?;
    println!("GF.VAL: {:?}", gf.get_cells());
    render();
    
    Ok(())
}
