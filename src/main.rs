mod cell;
mod game_field;
mod render;
use render::play;

fn main() -> std::io::Result<()> {
    play("example.map".to_string(), 1, [320, 240])?;
    Ok(())
}
