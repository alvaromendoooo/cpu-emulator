use std::{collections::HashMap, io::{self, BufRead}, convert::TryInto};

#[derive(Debug)]
pub struct IOregisters {
    pub register_map: HashMap<usize, i32>,
}

#[derive(Debug)]
pub struct MemoryBus {
    pub memory: Vec<u8>,
}

#[derive(Debug)]
pub struct AluInstructions {
    pub registers: Vec<u32>,
    pub memory_data: HashMap<u32, u32>,
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

impl AluInstructions {
    pub fn new(size: usize) -> Self {
        //let mut registers_init_values = vec![0, 10, 20, 30];
        //let mut default_registers_value = vec![0; size - registers_init_values.len()];

        //registers_init_values.append(&mut default_registers_value);
        Self {
            //registers: registers_init_values,
            registers: vec![0; size],
            memory_data: HashMap::new(),
        }
    }

    pub fn write_register_value(&mut self, register_id: usize, register_val: u32) {
        if register_id != 0 && register_id < self.registers.len() {
            self.registers[register_id] = register_val; // Save value of register inside 32 bit memory array
        }
    }

    pub fn operation_display(&mut self, op: &str, rd: usize,  rs1: usize, rs2: usize) -> String {
        let op1_val_ref = self.registers.get(rs1).copied().unwrap_or(0);
        let op2_val_ref = self.registers.get(rs2).copied().unwrap_or(0);

        match op {
            "ADD" => {
                self.add(rd, op1_val_ref, op2_val_ref);
                format!("x{}={}", rd, self.registers.get(rd).copied().unwrap_or(0))
            },
            "SUB" => {
                self.sub(rd, op1_val_ref, op2_val_ref);
                format!("x{}={}", rd, self.registers.get(rd).copied().unwrap_or(0))
            },
            "XOR" => {
                self.xor(rd, op1_val_ref, op2_val_ref);
                format!("x{}={}", rd, self.registers.get(rd).copied().unwrap_or(0))
            },
            "AND" => {
                self.and(rd, op1_val_ref, op2_val_ref);
                format!("x{}={}", rd, self.registers.get(rd).copied().unwrap_or(0))
            },
            "OR" => {
                self.or(rd, op1_val_ref, op2_val_ref);
                format!("x{}={}", rd, self.registers.get(rd).copied().unwrap_or(0))
            }
            _ => String::new()
        }
    }

    pub fn add(&mut self, register_result: usize, op1_val_ref: u32, op2_val_ref: u32) {

        let sum = op1_val_ref.wrapping_add(op2_val_ref); // Operate with u32 to prevent override from usize max size

        if register_result != 0 && register_result < self.registers.len() {
            self.registers[register_result] = sum;
        }
    }

    pub fn addi(&mut self, register_id: usize, imm: u32) {

        if register_id != 0 && register_id < self.registers.len() {
            let sum = self.registers[register_id].wrapping_add(imm);
            self.registers[register_id] = sum;
        }
    } 

    pub fn sub(&mut self, register_result: usize, op1_val_ref: u32, op2_val_ref: u32) {

        let sub = op1_val_ref.wrapping_sub(op2_val_ref); // Operate with u32 to prevent override from usize max size

        if register_result != 0 && register_result < self.registers.len() {
            self.registers[register_result] = sub;
        }
    }

    pub fn xor(&mut self, register_result: usize, op1_val_ref: u32, op2_val_ref: u32) {

        let xor = op1_val_ref ^ op2_val_ref; // Operate with u32 to prevent override from usize max size

        if register_result != 0 && register_result < self.registers.len() {
            self.registers[register_result] = xor;
        }
    }

    pub fn and(&mut self, register_result: usize, op1_val_ref: u32, op2_val_ref: u32) {

        let and = op1_val_ref & op2_val_ref; // Operate with u32 to prevent override from usize max size

        if register_result != 0 && register_result < self.registers.len() {
            self.registers[register_result] = and;
        }
    }

    pub fn or(&mut self, register_result: usize, op1_val_ref: u32, op2_val_ref: u32) {

        let or = op1_val_ref | op2_val_ref; // Operate with u32 to prevent override from usize max size

        if register_result != 0 && register_result < self.registers.len() {
            self.registers[register_result] = or;
        }
    }

    pub fn print(&mut self, register_id: u32) -> String {
        let result = format!("{}", self.registers[register_id as usize]);
        self.registers[register_id as usize] = 0;

        result
    }

    pub fn exit(&self, register_id: u32) -> String {
        format!("{}", self.registers[register_id as usize])
    }

    pub fn dump(&self) -> String {
        self.registers
            .iter()
            .map(|reg| reg.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn load_data_in_mem(&mut self, addr: u32, bytes: &str) {
        let clean_bytes = bytes.trim_start_matches("0x").trim_start_matches("0X");
        let lt_e_bytes = u32::from_str_radix(clean_bytes, 16).unwrap();
        self.memory_data.insert(addr, lt_e_bytes);
    }

    pub fn load_word(&self, addr: u32) -> u32 {
        let mut word : u32 = 0;
        for offset in 0..4 {
            let bytes : u32 = self.memory_data.get(&(addr + offset)).copied().unwrap_or(0);
            word |= bytes << (offset * 8); // Bytes are hex format, don't need to apply 0xFF mask
        }

        word
    }

    pub fn brach_decision(&self, op: &str, rs_1_val: i32, rs_2_val: i32, pc: i32, imm: i32) -> i32 {
        let rs1_u = rs_1_val as u32;
        let rs2_u = rs_2_val as u32;

        let take_branch = match op {
            "BEQ"  => rs_1_val == rs_2_val,
            "BNE"  => rs_1_val != rs_2_val,
            "BLT"  => rs_1_val < rs_2_val,
            "BGE"  => rs_1_val >= rs_2_val,
            "BLTU" => rs1_u < rs2_u,
            "BGEU" => rs1_u >= rs2_u,
            _      => false,
        };

        if take_branch { pc.wrapping_add(imm) } else { pc.wrapping_add(4) }
    }

    pub fn register_argument_values(&mut self, register_id: i32, value: u32,) -> Result<(), &'static str>  {        
        if register_id < 0 || register_id > 7 {
            return Err("Argument registers are setted from a0 to a7");
        }

        self.registers[10 + register_id as usize] = value;

        Ok(())
    }

    pub fn sum_arguments(&self) -> u32 {
        let mut sum: u32 = 0;
        let mut i = 8; // Number of registers reserved to arguments values

        while i > 0 {
            i -= 1;
            let val = self.registers[10 + i as usize];
            sum = sum.wrapping_add(val);
        }

        sum
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
    //let mut memory_bus = MemoryBus::new(1024);
    let mut alu = AluInstructions::new(32);
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

        /*match parts[0] {
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
        }*/
        
        match parts[0] {
            "WRITE" => {
                let register_id: usize = parts[1].parse().unwrap();
                let register_val: u32 = parts[2].parse().unwrap();
                alu.write_register_value(register_id, register_val);
            },
            "ADD" => {
                let register_result: usize = parts[1].parse().unwrap();
                let op1: u32 = parts[2].parse().unwrap();
                let op2: u32 = parts[3].parse().unwrap();
                alu.add(register_result, op1, op2);
            },
            "DUMP" => println!("{}", alu.dump()),
            "MEM" => {
                let mem_addr: u32 = parts[1].parse().unwrap();
                let bytes: &str = parts[2];
                alu.load_data_in_mem(mem_addr, bytes);
            },
            "LW" => {
                let mem_addr: u32 = parts[1].parse().unwrap();
                println!("{}", alu.load_word(mem_addr));
            },
            "BRANCH" => {
                let op = parts[1];
                let rs_1_val: i32 = parts[2].parse().unwrap();
                let rs_2_val: i32 = parts[3].parse().unwrap();
                let pc: i32 = parts[4].parse().unwrap();
                let imm = parts[5].parse().unwrap();

                println!("{}", alu.brach_decision(op, rs_1_val, rs_2_val, pc, imm));
            }
            "ARG" => {
                match alu.register_argument_values(parts[1].parse().unwrap(), parts[2].parse().unwrap()) {
                    Ok(_) => {},
                    Err(e) =>println!("{}", e),
                };
            },
            "ECALL" => {
                match parts[1].parse::<i32>().unwrap() {
                    64 => {
                        let fd: i32 = parts[2].parse().unwrap();
                        let buf: i32 = parts[3].parse().unwrap();
                        let count: i32 = parts[4].parse().unwrap();

                        println!("WRITE fd={} buf={} count={}", fd, buf, count);
                    },
                    93 => {
                        let exit_code: i32 = parts[2].parse().unwrap();

                        println!("EXIT {}", exit_code);
                    },
                    63 => {
                        let fd: i32 = parts[2].parse().unwrap();
                        let buf: i32 = parts[3].parse().unwrap();
                        let count: i32 = parts[4].parse().unwrap();

                        println!("READ fd={} buf={} count={}", fd, buf, count);
                    },
                    _ => println!("UNKNOWN {}", parts[1].parse::<i32>().unwrap()),
                }
            },
            "INST" => {
                match parts[1] {
                    "ADDI" => {
                        alu.addi(
                            parts[2].parse().unwrap(),
                            parts[3].parse().unwrap()
                        );
                    },
                    "PRINT" => println!("{}", alu.print(parts[2].parse().unwrap())),
                    "EXIT" => { println!("EXIT_CODE {}", alu.exit(parts[2].parse().unwrap())); break; },
                    _ => {}
                }
            }
            _ => break
        }

        //println!("{}", alu.operation_display(parts[0], parts[1].parse().unwrap(), parts[2].parse().unwrap(), parts[3].parse().unwrap()));
    }
    //println!("{}", alu.sum_arguments());
}