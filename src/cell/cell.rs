use std::collections::HashSet;

#[derive(Copy,Clone,Debug,Eq,Hash,PartialEq)]
pub enum STATUS {
    ALIVE = 49,
    DEAD  = 48,
}

#[repr(C)]
#[derive(Copy,Clone,Hash,Eq,PartialEq,Debug)]
pub struct CELL {
    status: STATUS,
    pos_x: u64,
    pos_y: u64
}

impl CELL {
    pub fn new(status: STATUS, pos_x: u64, pos_y: u64) -> Self {
        CELL {
            status: status,
            pos_x: pos_x,
            pos_y: pos_y
        }
    }

    pub fn kill(&mut self) {
        self.status = STATUS::DEAD;
    }

    fn get_min_pos(&self) -> (u64, u64) {
        let mut x = self.pos_x;
        let mut y = self.pos_y;
        if self.pos_x != 0 {
            x -= 1;
        }
        if self.pos_y != 0 {
            y -= 1;
        }
        (x, y)
    }


    pub fn check(&mut self, cells: &HashSet<Self>, child: Option<bool>) -> Vec<CELL> {
        let mut cx: u8 = 0;
        let mut new_cells = Vec::new();
        let start = self.get_min_pos();
        

        for x in start.0..=self.pos_x+1 {
            for y in start.1..=self.pos_y+1 {
                let mut cell = CELL::new(STATUS::ALIVE, x, y);
                if cells.contains(&cell) {
                    cx += 1;
                } 
                else {
                    match child {
                        Some(_) => (),
                        None => {
                            cell.check(cells, Some(true));
                            match cell.get_status() {
                                STATUS::ALIVE => new_cells.push(cell),
                                STATUS::DEAD  => (),
                            }
                        }
                    }
                }
            }
        }

        match cx {
            0|1 => self.kill(),
            2|3 => (),
            _   => self.kill()
        }

        new_cells
    }

    pub fn get_pos(&self) -> (u64,u64) {
        (self.pos_x, self.pos_y)
    }

    pub fn set_x(&mut self, x: u64) {
        self.pos_x = x;
    }

    pub fn set_y(&mut self, y: u64) {
        self.pos_y = y;
    }

    pub fn get_x(&self) -> u64 {
        self.pos_x
    }

    pub fn get_y(&self) -> u64 {
        self.pos_y
    }

    pub fn get_status(&self) -> &STATUS {
        &self.status
    }
}