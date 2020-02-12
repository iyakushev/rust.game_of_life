pub use crate::cell::{CELL, STATUS};
use std::collections::HashSet;
use std::string::String;
use std::io::Read;
use std::fs;


#[derive(Debug)]
pub struct GAMEFIELD {
    pub cells: HashSet<CELL>
}

impl GAMEFIELD {
    pub fn new() -> Self {
        GAMEFIELD {
            cells: HashSet::new()
        }
    }

    // pub fn get_cells(&self) -> &Vec<CELL> {                  |___|_X_|_X_|___|___|___|
    //     &self.cells.into_iter().collect::<Vec<_>>()          |___|___|_X_|___|_X_|___|
    // }                                                        |___|___|_X_|___|___|_X_|

    pub fn get_cells(&self) -> &HashSet<CELL> {
        &self.cells
    }

    pub fn next_generation(&mut self) {
        let prev_gen = self.cells.clone(); // Snapshot of the current generation
        let mut next_gen: HashSet<CELL> = HashSet::new();
        for mut cell in self.cells.drain() {
            for c in cell.check(&prev_gen, None).drain(..) {
                next_gen.insert(c);
            }
            match cell.get_status() {
                STATUS::ALIVE => next_gen.insert(cell),
                STATUS::DEAD  => false,
            };
        }
        self.cells = next_gen;
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
                10 => {pos_y += 1; pos_x = 0}, // New line
                48 => pos_x += 1,              // Does not store dead cells
                49 => {
                    self.cells.insert(CELL::new(STATUS::ALIVE, pos_x, pos_y));
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