use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use crate::game_field::GAMEFIELD;

fn opposite_color(mut color: [u8;4]) -> [u8;4] {
    color[0] = 255 - color[0];
    color[1] = 255 - color[1];
    color[2] = 255 - color[2];
    color
}

fn breathe(color: [u8;4], n: usize) -> Vec<[u8;4]> {
    let res = linear_gradient(color, opposite_color(color), n);
    res
}

/// Calculates a linear color interpolation for n steps
fn linear_gradient(color_source: [u8;4], color_destination: [u8;4], n: usize) -> Vec<[u8;4]>{
    let mut result = Vec::new();
    result.push(color_source);
    for t in 1..n {
        let mut color = [0,0,0,255];
        for channel in 0..3 {
            color[channel] = color_source[channel] + ((t as u8/(n-1)as u8) * (color_destination[channel] - color_source[channel]))
        }
        result.push(color);
    }
    result
}

fn polylinear_gradient(n: usize) -> Vec<[u8;4]> {
    let mut result = Vec::from(linear_gradient([255,0,0,255],[255,255,0,255],n));
    result.extend(linear_gradient([255,255,0,255],[0,255,0,255],n));
    result.extend(linear_gradient([0,255,0,255],[0,255,255,255],n));
    result.extend(linear_gradient([0,255,255,255],[0,0,255,255],n));
    result
}

// TODO Scaling
// TODO SDL2 port
pub fn play(filename: &str, 
            cell_size: u32, 
            color: [u8;4], 
            bg: [u8;4], 
            dimensions: [i32; 2],
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

    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    let window = video_subsystem.window("rust GOL", 800, 600).build().unwrap();
    let canvas = window.into_canvas().build().unwrap();

    let cell_ceiling: usize = cell_color.len();
    
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_ctx.event_pump().unwrap();


    // Event loop
    let mut i: usize = 0;
    'game: loop {
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { 
                    keycode: Some(Keycode::Q), ..
                } => break 'game,
                _ => {}
            }
        }


        for cell in gf.get_cells() { // drawing cells
            // cell_color[i]
            sdl2::rect::Rect::new(cell.get_x(), cell.get_y(), cell_size, cell_size);
        };

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


        canvas.present();
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));

     }

    Ok(())
}