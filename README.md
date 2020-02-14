<p align="center">
  <img height="30%" width="30%" src="http://www.euro-langues.org/wp-content/uploads/2019/10/communityIcon_sxcqnw4pxti11.png"/>
</p>
<h1 align="center">GAME OF LIFE IN RUST</h1>
<p align="center"> Yet another implementation of Conway's "Game of Life", but now in Rust.</p>

![](new.gif)

## To run:
1. `cargo build --release`.
2. `./target/release/game_of_life -f FILENAME`

## A list of arguments:
```shell
USAGE:
    game_of_life [FLAGS] [OPTIONS] --file <FILE>

FLAGS:
    -h, --help       Prints help information
        --rainbow    Makes everything ✨🌈
    -v, --version    Prints version information

OPTIONS:
    -b, --background <BG>    Sets background color. (Can be: black, red, blue, teal, white or HEX) [default: black]
    -c, --color <COLOR>      Sets cell color. (Can be: black, red, blue, teal, white or HEX) [default: black]
    -f, --file <FILE>        A path to the map with patterns (can contain only '0' and '1')
    -s, --size <SIZE>        Sets cell size [default: 1]
```

Rendering is done with the [Piston](https://github.com/PistonDevelopers/piston).   
CMD args is done with the [Clap]().     
