#[derive(Debug, PartialEq)]
pub enum STATUS {
    ALIVE = 49,
    DEAD  = 48,
}

#[repr(C)]
#[derive(Debug)]
pub struct CELL {
    status: STATUS,
    pos_x: i32,
    pos_y: i32
}

impl CELL {
    pub fn new(status: STATUS, pos_x: i32, pos_y: i32) -> Self {
        CELL {
            status: status,
            pos_x: pos_x,
            pos_y: pos_y
        }
    }

    pub fn get_pos(&self) -> (i32,i32) {
        (self.pos_x, self.pos_y)
    }

    pub fn get_status(&self) -> &STATUS {
        &self.status
    }
}