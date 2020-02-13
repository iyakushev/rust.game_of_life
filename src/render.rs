extern crate piston_window;

use piston_window::*;
use crate::game_field::GAMEFIELD;

fn opposite_color(color: [f32;4]) -> [f32;4] {
    for channel in 0..3 {
        color[channel] -= 1.0
    }
    color
}

/// Calculates a linear color interpolation for n steps
fn linear_gradient(color_source: [f32;4], color_destination: [f32;4], n: u32) -> Vec<[f32;4]>{
    let result = Vec::new();
    result.push(color_source);
    for t in 1..n {
        let color = [0.0,0.0,0.0,1.0];
        for channel in 0..3 {
            color[channel] = color_source[channel] - ((t/(n-1))as f32 * color_destination[channel] - color_source[channel])
        }
        result.push(color);
    }
    result
}

// TODO
fn polylinear_gradient() {

}



// TODO Scaling
// TODO GGEZ port
pub fn play(filename: &str, cell_size: u64, color: [f32;4], bg: [f32;4], dimensions: [u32; 2], rainbow: bool) -> std::io::Result<()> {
    let mut gf = GAMEFIELD::new(dimensions);
    gf.read_file(filename.to_string())?;
    let mut run = false;

    let cell_color = match rainbow{
        true  => {
        },
        false => color
    }

    let mut window: PistonWindow = WindowSettings::new("RGOL", dimensions).exit_on_esc(true).build().unwrap();
    
    // Event loop
    while let Some(event) = window.next() {
        if let Some(e) = event.press_args() { // Event handler for user input
            match e {
                Button::Keyboard(Key::Space) => {if run {run = false} else {run = true}},
                _ => ()
            }
        };

        window.draw_2d(&event, |context, graphics, _device| {
            clear(bg, graphics);
            for cell in gf.get_cells() { // drawing cells
                rectangle(color,
                          [cell.get_x() as f64,
                           cell.get_y() as f64,
                           cell_size as f64,
                           cell_size as f64],
                           context.transform,
                           graphics);
            }
        });

        if run { // Iterating cell automata
            gf.next_generation();
        }
    }
    Ok(())
}