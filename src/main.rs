mod cell;
mod game_field;
mod render;

use clap::{Arg, App};
use render::play;

const VERSION:    &str    = "v0.1";
// const RESOLUTION: [u32;2] = [320,240];
const CELL_SIZE:  u64     = 1;


fn main() -> std::io::Result<()> {
    let cell_size = CELL_SIZE.to_string();
    let args = App::new("Rust.GoL")
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
                    )
                    .arg(
                        Arg::with_name("size")
                        .long("size")
                        .short("s")
                        .takes_value(true)
                        .value_name("SIZE")
                        .default_value(&cell_size)
                        .help("Sets cell size")
                    );
                    //TODO add width and height
                    // .arg(
                    //     Arg::with_name("width")
                    //     .long("width")
                    //     .short("w")
                    //     .takes_value(true)
                    //     .value_name("W")
                    //     .default_value(&RESOLUTION[0].to_string())
                    //     .help("Sets screen width")
                    // ).arg(
                    //     Arg::with_name("height")
                    //     .long("height")
                    //     .short("h")
                    //     .takes_value(true)
                    //     .value_name("H")
                    //     .default_value(&RESOLUTION[1].to_string())
                    //     .help("Sets screen height")
                    // );
    let input = args.get_matches();
    let file  = input.value_of("file").expect("Please provide a path to the map file.\n(Use '-f')");
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
    play(file, c_size, [640, 480])?;
    Ok(())
}
