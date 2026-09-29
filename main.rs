use std::{collections::HashMap, io::{self, BufRead}};

#[derive(Debug)]
pub struct IOregisters {
    pub register_map: HashMap<usize, i32>,
}

impl IOregisters {
    pub fn new() -> Self {
        Self {
            register_map: [(0, 0)].into_iter().collect(),
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

fn main() {
    let stdin = io::stdin();
    let mut io_register = IOregisters::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() { continue; }

        match io_register.register(l) {
            Ok(Some(result)) => println!("{}", result),
            Ok(None) => {},
            Err(e) => println!("{}", e)
        }
    }
}