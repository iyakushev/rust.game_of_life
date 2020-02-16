extern crate piston_window;

use piston_window::*;
use crate::game_field::GAMEFIELD;

fn opposite_color(mut color: [f32;4]) -> [f32;4] {
    for channel in 0..3 {
        color[channel] = 1.0 - color[channel];
    }
    color
}

fn breathe(color: [f32;4], n: usize) -> Vec<[f32;4]> {
    let res = linear_gradient(color, opposite_color(color), n);
    res
}

/// Calculates a linear color interpolation for n steps
fn linear_gradient(color_source: [f32;4], color_destination: [f32;4], n: usize) -> Vec<[f32;4]>{
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
fn polylinear_gradient(n: usize) -> Vec<[f32;4]> {
    let mut result = Vec::from(linear_gradient([1.0,0.0,0.0,1.0],[1.0,1.0,0.0,1.0],n));
    result.extend(linear_gradient([1.0,1.0,0.0,1.0],[0.0,1.0,0.0,1.0],n));
    result.extend(linear_gradient([0.0,1.0,0.0,1.0],[0.0,1.0,1.0,1.0],n));
    result.extend(linear_gradient([0.0,1.0,1.0,1.0],[0.0,0.0,1.0,1.0],n));
    result
}

// TODO Scaling
// TODO GGEZ port
pub fn play(filename: &str, 
            cell_size: usize, 
            color: [f32;4], 
            bg: [f32;4], 
            dimensions: [u32; 2],
            random: bool,
            rainbow: bool, 
            breathing: bool,
            interpolation: usize) -> std::io::Result<()> {
    
    let mut gf = GAMEFIELD::new(dimensions);
    if random {gf.random_field();}
    else {gf.read_file(filename.to_string())?;}
    

    let cell_color = match rainbow {
        true  => {
            polylinear_gradient(interpolation)
        },
        false => match breathing {
            true  => {
                breathe(color, interpolation)
            },
            false => vec![color]
        }
    };
    let mut run = false;
    let mut color_inc = true; // switches the direction of color interpolation 


    let mut window: PistonWindow = WindowSettings::new("RGOL", dimensions).exit_on_esc(true).build().unwrap();
    let cell_ceiling: usize = cell_color.len();
    
    // Event loop
    let mut i: usize = 0;
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
                rectangle(cell_color[i],
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
            match color_inc {
                true => {
                    if i >= cell_ceiling-1 {color_inc = false;}
                    else {i+=1}
                },
                false => {
                    if i <= 1 {color_inc = true;}
                    else {i-=1}
                }
            }
        }
    }
    Ok(())
}