pub use crate::cell::{CELL, STATUS};
use std::io::Read;
use std::fs;

#[derive(Debug)]
pub struct GAMEFIELD {
    cells: Vec<CELL>,
    size_w: i32,
    size_h: i32
}

impl GAMEFIELD {
    pub fn new() -> Self {
        GAMEFIELD {
            cells: vec![CELL::new(STATUS::DEAD,0,0)],
            size_w:0,
            size_h:0
        }
    }

    pub fn get_cells(&self) -> &Vec<CELL> {
        &self.cells
    }

    pub fn scavenge_dead_cells(&mut self) {
        self.cells.retain(|cell| *cell.get_status()==STATUS::ALIVE)
    }

    pub fn read_file(&mut self, filename: String) -> std::io::Result<()> {
        let f = fs::File::open(filename)?;
        let mut pos_y = 0;
        let mut pos_x = 0;

        for byte in f.bytes() {
            match byte.unwrap() {
                10 => pos_y += 1,
                48 => pos_x += 1, // Does not store dead cells
                49 => {
                    self.cells.push(CELL::new(STATUS::ALIVE, pos_x, pos_y));
                    pos_x += 1
                },
                _ => {
                    eprintln!("Incorrect symbol in the map! Exiting.");
                    std::process::exit(1);
                }
            }
        }
        Ok(())
    }
}