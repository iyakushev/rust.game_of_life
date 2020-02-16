extern crate rand;

use std::collections::HashSet;
use rand::{Rng, distributions::{Distribution, Standard}};

#[derive(Copy,Clone,Debug,Eq,Hash,PartialEq)]
pub enum STATUS {
    ALIVE = 49,
    DEAD  = 48,
}

impl Distribution<STATUS> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> STATUS {
        match rng.gen_range(0, 2) {
            0 => STATUS::DEAD,
            _ => STATUS::ALIVE,
        }
    }
}


#[repr(C)]
#[derive(Copy,Clone,Hash,Eq,PartialEq,Debug)]
pub struct CELL {
    status: STATUS,
    pos_x: u32,
    pos_y: u32
}

impl CELL {
    pub fn new(status: STATUS, pos_x: u32, pos_y: u32) -> Self {
        CELL {
            status: status,
            pos_x: pos_x,
            pos_y: pos_y
        }
    }

    pub fn kill(&mut self) {
        self.status = STATUS::DEAD;
    }

    fn get_range(&self, position: u32, dimension: u32) -> [u32;3] {
        if position == 0 {
            [dimension, 0, 1]
        }
        else if position == dimension {
            [dimension-1, dimension, 0]
        }
        else {
            [position-1, position, position+1]
        }
    }

    pub fn check(&mut self, cells: &HashSet<CELL>, dimensions: (u32, u32), is_child: bool) -> Vec<CELL> {
        let mut cx: u8 = 0;
        let mut new_cells = Vec::new();

        for y in self.get_range(self.pos_y, dimensions.1).iter() {
            for x in self.get_range(self.pos_x, dimensions.0).iter() {
                if self.get_pos() == (*x, *y) {
                    continue;
                }
                let mut cell = CELL::new(STATUS::ALIVE, *x, *y);
                if cells.contains(&cell) {
                    cx += 1;
                } 
                else { // If cell is dead.
                    match is_child {
                        true => (),
                        false => {
                            cell.check(cells, dimensions, true);
                            match cell.get_status() {
                                STATUS::ALIVE => new_cells.push(cell),
                                STATUS::DEAD  => ()
                            }
                        }
                    }
                }
            }
        }
        match cx {
            0|1 => self.kill(),
            2 => if is_child {
                self.kill();
            },
            3 => (),
            _   => self.kill()
        }
        new_cells
    }

    pub fn get_pos(&self) -> (u32,u32) {
        (self.pos_x, self.pos_y)
    }

    pub fn get_x(&self) -> u32 {
        self.pos_x
    }

    pub fn get_y(&self) -> u32 {
        self.pos_y
    }

    pub fn get_status(&self) -> &STATUS {
        &self.status
    }
}