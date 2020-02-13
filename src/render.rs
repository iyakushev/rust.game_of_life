extern crate piston_window;

use piston_window::*;
use crate::game_field::GAMEFIELD;


// TODO Scaling
// TODO GGEZ port
pub fn play(filename: &str, cell_size: u64, color: [f32;4], bg: [f32;4], dimensions: [u32; 2], rainbow: bool) -> std::io::Result<()> {
    let mut gf = GAMEFIELD::new(dimensions);
    gf.read_file(filename.to_string())?;
    let mut run = false;

    let mut window: PistonWindow =
        WindowSettings::new("RGOL", dimensions)
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        if let Some(e) = event.press_args() {
            match e {
                Button::Keyboard(Key::Space) => {if run {run = false} else {run = true}},
                _ => ()
            }
        };
        window.draw_2d(&event, |context, graphics, _device| {
            clear(bg, graphics);
            for cell in gf.get_cells() {
                rectangle(color,
                          [cell.get_x() as f64,
                           cell.get_y() as f64,
                           cell_size as f64,
                           cell_size as f64],
                           context.transform,
                           graphics);
            }
        });
        if run {
            gf.next_generation();
        }
    }
    Ok(())
}