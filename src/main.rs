use std::io::Read;
use std::string::String;
use std::fs;
// use num::Num;

#[derive(Debug)]
pub enum STATUS {
    ALIVE = 49,
    DEAD  = 48,
}

#[repr(C)]
#[derive(Debug)]
pub struct CELL {
    value: STATUS,
    pos_x: i32,
    pos_y: i32
}

impl CELL {
    pub fn new(status: STATUS, pos_x: i32, pos_y: i32) -> Self {
        CELL {
            value: status,
            pos_x: pos_x,
            pos_y: pos_y
        }
    }
}



// TODO
// IN ORDER TO SAVE ON MEMORY THERE CAN BE IMPLEMENTED
// AN 8-BIT MAP(u8) TO STORE 8 VALUES
// #[derive(Debug)]
// pub struct BITMAP8 {
//     field: Type
// }

// impl BITMAP8 {
//     pub fn from_vec(arr: Vec<u8>) -> Self {
        
//     }
// }

#[derive(Debug)]
pub struct GAMEFIELD {
    field: Vec<CELL>,
    size_w: i32,
    size_h: i32
}

impl GAMEFIELD {
    pub fn new() -> Self {
        GAMEFIELD {
            field: vec![CELL{value:STATUS::DEAD,pos_x:0,pos_y:0}],
            size_w:0,
            size_h:0
        }
    }

    pub fn read_file(&mut self, filename: String) {
        let f = fs::File::open(filename).expect("Something wrong with the file!");
        let mut pos_y = 0;
        let mut pos_x = 0;

        for byte in f.bytes() {
            match byte.unwrap() {
                10 => pos_y += 1,
                48 => {
                    self.field.push(CELL::new(STATUS::DEAD, pos_x, pos_y));
                    pos_x += 1
                },
                49 => {
                    self.field.push(CELL::new(STATUS::ALIVE, pos_x, pos_y));
                    pos_x += 1
                },
                _ => {
                    println!("Incorrect symbol in the map! Exiting.");
                    break
                }
            }
        }
    }

    
    // A method for BITMAP8
    // pub fn read_file(&mut self, filename: String) {
    //     let mut f = fs::File::open(filename).expect("Something wrong with the file!");
    //     let mut buf = [0u8;10]; // read 8 "bits"/symbols of the map.
    //     loop {
    //         match f.read_exact(&mut buf) {
    //             Ok(bytes) => self.field.append(BITMAP8{value: bytes}),
    //             _ => break
    //         }
    //     }
    // }
}

fn main() {
    println!("Struct: {}", std::mem::size_of::<CELL>());
    let mut gf = GAMEFIELD::new();
    gf.read_file("example.map".to_string());
    println!("GF: {}", std::mem::size_of_val(&gf));
    println!("GF.VAL: {:?}", gf.field);
}
