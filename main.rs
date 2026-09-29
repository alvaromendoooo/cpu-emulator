use std::{collections::HashMap, io::{self, BufRead}};

#[derive(Debug)]
pub struct IOregisters {
    pub register_map: HashMap<usize, i32>,
}

#[derive(Debug)]
pub struct MemoryBus {
    pub memory: Vec<u8>,
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

impl MemoryBus {
    pub fn new(size: usize) -> Self {
        Self {
            memory: vec![0; size],
        }
    }

    // Ensure that memory has enough size to acces required_size
    fn ensure_capacity(&mut self, required_size: usize) {
        if self.memory.len() < required_size {
            self.memory.resize(required_size, 0);
        }
    }

    // WRITE <addr> <byte>
    pub fn write_byte(&mut self, addr: usize, byte: u8) {
        self.ensure_capacity(addr + 1); // Make space for a byte to be written
        self.memory[addr] = byte;
    } 

    // WRITEW <addr> <word>
    pub fn write_word(&mut self, addr: usize, value: u32) {
        self.ensure_capacity(addr + 4); // Make space of a word which is 4 bytes to be written
        let bytes = value.to_le_bytes(); // Convert the word into Little endian bytes [u8;4]
        self.memory[addr..addr + 4].copy_from_slice(&bytes); // Copies all elements of &bytes into &mut self.memory without panic for overflow
    }

    // READW <addr>
    pub fn read_word(&mut self, addr: usize) -> u32 {
        self.ensure_capacity(addr + 4);
        let slice: [u8; 4] = self.memory[addr..addr + 4]
            .try_into()
            .expect("Slice with exact length of 4.");

        u32::from_le_bytes(slice) // Converts 4 little-endian bytes back into u32
    }
}

pub fn decode_opcode(line: String) -> String {
    
    let instruction = u32::from_str_radix(line.trim(), 16).unwrap();

    let mut result = String::new();

    match instruction & 0x7F {
        0x33 => result.push_str("R"),
        0x13 | 0x03 | 0x67 | 0x73 => result.push_str("I"),
        0x23 => result.push_str("S"),
        0x63 => result.push_str("B"),
        0x37 | 0x17 => result.push_str("U"),
        0x6F => result.push_str("J"),
        _ => result.push_str("UNKNOWN"),
    }

    result
}

fn main() {
    let stdin = io::stdin();
    //let mut io_register = IOregisters::new();
    let mut memory_bus = MemoryBus::new(1024);
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() { continue; }

        /*match io_register.register(l) {
            Ok(Some(result)) => println!("{}", result),
            Ok(None) => {},
            Err(e) => println!("{}", e)
        }*/

        // println!("{}", decode_opcode(l));

        let parts: Vec<&str> = l.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "WRITE" => {
                let address: usize = parts[1].parse().unwrap();
                let byte: u8 = parts[2].parse().unwrap();
                memory_bus.write_byte(address, byte);
            },
            "WRITEW" => {
                let address: usize = parts[1].parse().unwrap();
                let word: u32 = parts[2].parse().unwrap();
                memory_bus.write_word(address, word);
            },
            "READW" => {
                let address: usize = parts[1].parse().unwrap();
                println!("{}", memory_bus.read_word(address));
            }
            _ => {}
        }
    }
}