use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::game_field::GAMEFIELD;

fn opposite_color(mut color: [u8;4]) -> [u8;4] {
    color[0] = 255 - color[0];
    color[1] = 255 - color[1];
    color[2] = 255 - color[2];
    color
}

fn breathe(color: [u8;4], n: u32) -> Vec<(u8,u8,u8,u8)> {
    linear_gradient(color, opposite_color(color), n)
}

/// Calculates a linear color interpolation for n steps
fn linear_gradient(color_source: [u8;4], color_destination: [u8;4], n: u32) -> Vec<(u8,u8,u8,u8)>{
    let mut result = Vec::<(u8,u8,u8,u8)>::new();
    for t in 0..n {
        let mut color = [0,0,0,255];
        for channel in 0..3 {
            color[channel] = normailize_float(normailize_unsgn(color_source[channel]) + ((t as f32/(n-1) as f32) * (normailize_unsgn(color_destination[channel]) - normailize_unsgn(color_source[channel]))));
        }
        result.push((color[0],color[1],color[2],color[3]));
    }
    result
}

fn polylinear_gradient(n: u32) -> Vec<(u8,u8,u8,u8)> {
    let mut result = Vec::from(linear_gradient([255,0,0,255],[255,255,0,255],n));
    result.extend(linear_gradient([255,255,0,255],[0,255,0,255],n));
    result.extend(linear_gradient([0,255,0,255],[0,255,255,255],n));
    result.extend(linear_gradient([0,255,255,255],[0,0,255,255],n));
    result
}

fn normailize_unsgn(num: u8) -> f32 {
    num as f32 / 255.0
}

fn normailize_float(num: f32) -> u8 {
    if num == 1.0 {255} else {(num*256.0) as u8}
}

pub fn play(filename: &str, 
            cell_size: u32, 
            color: [u8;4], 
            bg: [u8;4], 
            dimensions: [u32; 2],
            random: bool,
            rainbow: bool, 
            breathing: bool,
            interpolation: u32,
            fps_lim: u32) -> Result<(), String> {
    
    let mut gf = GAMEFIELD::new(dimensions);
    if random {gf.random_field();}
    else {gf.read_file(filename.to_string()).unwrap();}
    

    let cell_color = match rainbow {
        true  => {
            polylinear_gradient(interpolation)
        },
        false => match breathing {
            true  => {
                breathe(color, interpolation)
            },
            false => vec![(color[0], color[1], color[2], color[3])]
        }
    };
    let mut run = false;
    let mut color_inc = true; // switches the direction of color interpolation 

    let sdl_ctx = sdl2::init()?;
    let video_subsystem = sdl_ctx.video()?;
    let window = video_subsystem.window("rust GOL", dimensions[0], dimensions[1]).build().map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_ctx.event_pump()?;
    
    let cell_ceiling: usize = cell_color.len();
    
    // Event loop
    let mut i: usize = 0;
    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { 
                    keycode: Some(Keycode::Q), ..
                } => break 'game,
                Event::KeyDown {
                    keycode: Some(Keycode::Space), ..
                } => run = !run,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGBA(bg[0], bg[1], bg[2], bg[3]));
        canvas.clear();


        for cell in gf.get_cells() { // drawing cells
            canvas.set_draw_color(Color::from(cell_color[i]));
            canvas.fill_rect(
                Rect::new(
                    cell.get_x(), 
                    cell.get_y(), 
                    cell_size, 
                    cell_size))?
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
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / fps_lim));

     }

    Ok(())
}