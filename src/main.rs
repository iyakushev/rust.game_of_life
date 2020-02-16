mod cell;
mod game_field;
mod render;

use clap::{Arg, App};
use render::play;
use phf::{phf_map};
use piston_window::color::hex;

// --------------------==CONSTANTS==-------------------- \\
// const RESOLUTION: [u32;2] = [320,240];
const VERSION:    &str    = "v0.3";
const CELL_SIZE:  usize   = 1;
const GRADIENT_INT: usize = 500;
const COLOR: phf::Map<&'static str, [f32;4]> = phf_map!{
        "black" => [0.0,0.0,0.0,1.0],
        "red" => [1.0,0.0,0.0,1.0],
        "blue" => [0.0,1.0,0.0,1.0],
        "teal" => [0.0,0.5,0.5,1.0],
        "white" => [1.0; 4]
};

fn parse_color(color: Option<&str>) -> [f32;4] {
    match color {
        Some(c) => {
            if COLOR.contains_key(c) {
                COLOR[c]
            } else {
                match Some(hex(c)) {
                    Some(res) => res,
                    _ => {
                        println!("Error while parsing HEX value. Going black");
                        COLOR["black"]
                    }
                }
            }
        },
        _ => COLOR["black"],
    }
}

fn parse_value(val: Option<&str>, default: usize) -> usize {
    match val {
        Some(v) => {
            let v = v.parse::<usize>();
            if let Ok(v_) = v {
                v_
            } else {
                default
            }
        },
        _ => default
    }
}

//TODO add width and height
fn main() -> std::io::Result<()> {
    let cell_size = CELL_SIZE.to_string();
    let grad_int  = GRADIENT_INT.to_string();
    let input = App::new("Rust.GoL")
                    .version_short("v")
                    .version(VERSION)
                    .about("Rust GoL implementation")
                    .arg(
                        Arg::with_name("file")
                        .long("file")
                        .short("f")
                        .takes_value(true)
                        .value_name("FILE")
                        .required_unless("random")
                        .help("A path to the map with patterns (can contain only '0' and '1')")
                    ).arg(
                        Arg::with_name("size")
                        .long("size")
                        .short("s")
                        .takes_value(true)
                        .value_name("SIZE")
                        .default_value(&cell_size)
                        .help("Sets cell size")
                    ).arg(
                        Arg::with_name("rainbow")
                        .long("rainbow")
                        .help("Makes everything ✨🌈")
                    ).arg(
                        Arg::with_name("random")
                        .long("random")
                        .help("Generates random patterns across the field")
                    ).arg(
                        Arg::with_name("interpolation")
                        .long("interpolation")
                        .short("i")
                        .takes_value(true)
                        .value_name("INTERPOLATION_STEPS")
                        .default_value(&grad_int)
                        .help("Gradient interpolation steps")
                    ).arg(
                        Arg::with_name("breathe")
                        .long("breathe")
                        .help("Does a 'breathing' effect between two colors (color and 255-color)")
                    ).arg(
                        Arg::with_name("color")
                        .long("color")
                        .short("c")
                        .takes_value(true)
                        .value_name("COLOR")
                        .default_value("white")
                        .help("Sets cell color. (Can be: black, red, blue, teal, white or HEX)")
                    ).arg(
                        Arg::with_name("background")
                        .long("background")
                        .short("bg")
                        .takes_value(true)
                        .value_name("BG")
                        .default_value("black")
                        .help("Sets background color. (Can be: black, red, blue, teal, white or HEX)")
                    )
                    .get_matches();
    let breathe = input.is_present("breathe");
    let rainbow = input.is_present("rainbow");
    let random  = input.is_present("random");
    let mut file= "";
    if !random {
        file    = input.value_of("file").expect("Please provide a path to the map file.\n(Use '-f str')");
    }
    let color   = parse_color(input.value_of("color"));
    let bg_clr  = parse_color(input.value_of("background"));
    let c_size  = parse_value(input.value_of("size"), CELL_SIZE);
    let grad_int= parse_value(input.value_of("interpolation"), GRADIENT_INT);
    play(file, c_size, color, bg_clr, [640, 480], random, rainbow, breathe, grad_int)?;
    Ok(())
}
