extern crate rand;

pub use crate::cell::{CELL, STATUS};
use std::collections::HashSet;
use std::string::String;
use std::io::Read;
use std::fs;

#[derive(Debug)]
pub struct GAMEFIELD {
    cells: HashSet<CELL>,
    width: u32,
    height: u32,
    c_width: u32,
    c_height: u32
}

//TODO store cell size and redo checks with it
impl GAMEFIELD {
    pub fn new(dimensions: [u32;2]) -> Self {
        GAMEFIELD {
            cells: HashSet::new(),
            width: dimensions[0],
            height: dimensions[1],
            c_width: dimensions[0]/2,
            c_height: dimensions[1]/2
        }
    }

    pub fn get_cells(&self) -> &HashSet<CELL> {
        &self.cells
    }

    // TODO add custom game rule cb to the interface. -> (Renderer, CELL::check?)
    /// Main game rule.
    pub fn next_generation(&mut self) {
        let prev_gen = self.cells.clone(); // Snapshot of the current generation
        let mut next_gen: HashSet<CELL> = HashSet::new();
        for mut cell in self.cells.drain() {
            for c in cell.check(&prev_gen, (self.width as i32, self.height as i32), false).drain(..) {
                next_gen.insert(c);
            }
            match cell.get_status() {
                STATUS::ALIVE => next_gen.insert(cell),
                STATUS::DEAD  => false,
            };
        }
        self.cells = next_gen;
    }

    pub fn random_field(&mut self) {
        let mut cx = 0;
        for x in 0..self.width as i32 {
            for y in 0..self.height as i32 {
                let status: STATUS = rand::random();
                if status == STATUS::DEAD {cx+=1}
                else {
                    self.cells.insert(CELL::new(status, x, y));
                }
            }
        }
        println!("Dead  cells = {}", cx);
        println!("Alive cells = {}", self.cells.len());
        println!("-----------\nTotal: {}", self.width*self.height);
    }

    pub fn read_file(&mut self, filename: String) -> std::io::Result<()> {
        let f = fs::File::open(filename)?;
        let mut pos_y = self.c_height as i32;
        let mut pos_x = self.c_width as i32;

        for byte in f.bytes() {
            match byte.unwrap() {
                10 => {pos_y += 1; pos_x = self.c_width as i32}, // New line
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