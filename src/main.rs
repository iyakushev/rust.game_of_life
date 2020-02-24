mod cell;
mod game_field;
mod render;

extern crate num;

use num::Num;
use std::str::FromStr;
use clap::{Arg, App};
use std::ops::{Add, Sub};
use render::play;
use phf::{phf_map};

// --------------------==CONSTANTS==-------------------- \\
const RESOLUTION: [u32;2] = [640,480];
const VERSION:    &str    = "v0.4s";
const CELL_SIZE:  u32     = 1;
const GRADIENT_INT: usize = 500;
const COLOR: phf::Map<&'static str, [u8; 4]> = phf_map!{
        "black" => [0,0,0,255],
        "red"   => [255,0,0,255],
        "blue"  => [0,255,0,255],
        "teal"  => [0,128,128,255],
        "white" => [255,255,255,255]
};

/// Converts from hexadecimal color format
// pub fn hex(hex: &str) -> Color {

//     for ch in hex.chars() {
//         match ch {
//             "#" => (),

//         }
//     }


//     let color = match a {
//         None => [rgb[0], rgb[1], rgb[2], 255],
//         Some(a) => [rgb[0], rgb[1], rgb[2], a]
//     };
//     (
//         color[0] as f32 * inv_255,
//         color[1] as f32 * inv_255,
//         color[2] as f32 * inv_255,
//         color[3] as f32 * inv_255
//     )
// }

fn hex(hex: &str) -> [u8;4] {
    COLOR["black"]
}

fn parse_color(color: Option<&str>) -> [u8;4] {
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

fn parse_value<T: FromStr, Num>(val: Option<&str>, default: T) -> T {
    match val {
        Some(v) => {
            let v = v.parse::<T>();
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
