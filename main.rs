use std::{collections::HashMap, io::{self, BufRead}};

#[derive(Debug)]
pub struct IOregisters {
    pub register_map: HashMap<usize, i32>,
}

impl IOregisters {
    pub fn new() -> Self {
        let mut register_map = HashMap::new();
        register_map.insert(0, 0);

        Self {
            register_map,
        }
    }

    pub fn register(&mut self, line: String) -> Result<Option<i32>, String> {
        let mut split_whitespace = line.split_whitespace();
        let instruction = match split_whitespace.next() {
            Some(inst) => inst,
            None => return Err("Empty line".to_string())
        };
        let register_ref: usize = match split_whitespace.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => return Err("Missing or invalid register reference".to_string()),
        };

        match instruction {
            "WRITE" => {
                let register_value: i32 = match split_whitespace.next().and_then(|s| s.parse().ok()) {
                    Some(val) => val,
                    None => return Err("Missing or invalid register value for WRITE".to_string())
                };
                if register_ref != 0 {
                    self.register_map.insert(register_ref, register_value);
                }
                Ok(None)
            },
            "READ" => {
                if let Some(&value) = self.register_map.get(&register_ref) {
                    Ok(Some(value))
                } else {
                    Err(format!("err: register {} has no value referred", register_ref))
                }
            }
            _ => Err(format!("mismatch: {} instruction not supported", instruction)),
        }
    }
}

pub fn decode_opcode(line: String) -> String {
    let clean_line = line.trim();
    if clean_line.is_empty() {
        return String::new();
    }

    let instruction = match u32::from_str_radix(clean_line, 16) {
        Ok(val) => val,
        Err(_) => return "UNKNOWN".to_string(),
    };

    let format_str = match instruction & 0x7F {
        0x33 => "R",
        0x13 | 0x03 | 0x67 | 0x73 => "I",
        0x23 => "S",
        0x63 => "B",
        0x37 | 0x17 => "U",
        0x6F => "J",
        _ => "UNKNOWN",
    };

    format_str.to_string()
}

fn main() {
    let stdin = io::stdin();
    //let mut io_register = IOregisters::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() { continue; }

        /*match io_register.register(l) {
            Ok(Some(result)) => println!("{}", result),
            Ok(None) => {},
            Err(e) => println!("{}", e)
        }*/

        println!("{}", decode_opcode(l));
    }
}