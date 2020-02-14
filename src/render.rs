extern crate piston_window;

use piston_window::*;
use crate::game_field::GAMEFIELD;

fn opposite_color(mut color: [f32;4]) -> [f32;4] {
    for channel in 0..3 {
        color[channel] = 1.0 - color[channel];
    }
    color
}

fn breathe(color: [f32;4], n: u32) -> Vec<[f32;4]> {
    let mut res = linear_gradient(color, opposite_color(color), n);
    res.append(&mut linear_gradient(opposite_color(color), color, n));
    res
}


/// Calculates a linear color interpolation for n steps
fn linear_gradient(color_source: [f32;4], color_destination: [f32;4], n: u32) -> Vec<[f32;4]>{
    let mut result = Vec::new();
    result.push(color_source);
    for t in 1..n {
        let mut color = [0.0,0.0,0.0,1.0];
        for channel in 0..3 {
            color[channel] = color_source[channel] + ((t as f32/(n-1)as f32) * (color_destination[channel] - color_source[channel]))
        }
        result.push(color);
    }
    result
}

// TODO
fn polylinear_gradient(colors: Vec<[f32;4]>) {
    // let mut result = linear_gradient();
}



// TODO Scaling
// TODO GGEZ port
pub fn play(filename: &str, 
            cell_size: u64, 
            color: [f32;4], 
            bg: [f32;4], 
            dimensions: [u32; 2], 
            rainbow: bool, breathing: bool) -> std::io::Result<()> {
    let mut gf = GAMEFIELD::new(dimensions);
    gf.read_file(filename.to_string())?;
    let mut run = false;

    let cell_color = match rainbow {
        true  => {
            linear_gradient(color, opposite_color(color), 500)
        },
        false => match breathing {
            true  => {
                breathe(color, 500)
            },
            false => vec![color]
        }
    };

    let mut window: PistonWindow = WindowSettings::new("RGOL", dimensions).exit_on_esc(true).build().unwrap();
    let mut cell_color_iter = cell_color.iter().cycle();
    // Event loop
    let mut new_color = *cell_color_iter.next().unwrap();
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
                rectangle(new_color,
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
            new_color = *cell_color_iter.next().unwrap();
        }
    }
    Ok(())
}