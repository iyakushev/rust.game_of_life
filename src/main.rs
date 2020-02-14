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
const CELL_SIZE:  u64     = 1;
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

//TODO add width and height
fn main() -> std::io::Result<()> {
    let cell_size = CELL_SIZE.to_string();
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
                        .required(true)
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
                        Arg::with_name("breathe")
                        .long("breathe")
                        .help("Does a 'breathing' effect between two colors (color and 1-color)")
                    ).arg(
                        Arg::with_name("color")
                        .long("color")
                        .short("c")
                        .takes_value(true)
                        .value_name("COLOR")
                        .default_value("black")
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
    let file    = input.value_of("file").expect("Please provide a path to the map file.\n(Use '-f str')");
    let rainbow = input.is_present("rainbow");
    let breathe = input.is_present("breathe");
    let color   = parse_color(input.value_of("color"));
    let bg_clr  = parse_color(input.value_of("background"));
    let c_size= match input.value_of("size") {
        Some(size) => {
            let size = size.parse::<u64>();
            if let Ok(size_) = size {
                size_
            } else {
                CELL_SIZE
            }
        },
        _ => CELL_SIZE
    };
    play(file, c_size, color, bg_clr, [640, 480], rainbow, breathe)?;
    Ok(())
}
