/*
    The CHIP-8 main memory module and program counter, including offset.

    In the CHIP-8 system, the program data exists in main memory beginning
    at address 0x200. Some built-in sprite data is also stored in memory.

    This module transforms addresses using the 0x200 offset, so external to
    this module all addresses should be as-is, untransformed.
*/
pub struct MainMemory {
    pub program_length: usize,

    memory: Vec<u8>,
    program_counter: usize,
}

impl MainMemory {
    const MEMORY_SIZE: usize = 4 * 1024;
    const PROGRAM_OFFSET: u16 = 0x200;
    const FONT_SPRITES: [u8; 80] = [0xF0, 0x90, 0x90, 0x90, 0xF0,   // 0
                                    0x20, 0x60, 0x20, 0x20, 0x70,   // 1
                                    0xF0, 0x10, 0xF0, 0x80, 0xF0,   // 2
                                    0xF0, 0x10, 0xF0, 0x10, 0xF0,   // 3
                                    0x90, 0x90, 0xF0, 0x10, 0x10,   // 4
                                    0xF0, 0x80, 0xF0, 0x10, 0xF0,   // 5
                                    0xF0, 0x80, 0xF0, 0x90, 0xF0,   // 6
                                    0xF0, 0x10, 0x20, 0x40, 0x40,   // 7
                                    0xF0, 0x90, 0xF0, 0x90, 0xF0,   // 8
                                    0xF0, 0x90, 0xF0, 0x10, 0xF0,   // 9
                                    0xF0, 0x90, 0xF0, 0x90, 0x90,   // A
                                    0xE0, 0x90, 0xE0, 0x90, 0xE0,   // B
                                    0xF0, 0x80, 0x80, 0x80, 0xF0,   // C
                                    0xE0, 0x90, 0x90, 0x90, 0xE0,   // D
                                    0xF0, 0x80, 0xF0, 0x80, 0xF0,   // E
                                    0xF0, 0x80, 0xF0, 0x80, 0x80];  // F

    pub fn new(mut program_data: Vec<u8>) -> MainMemory {
        let program_length = program_data.len() / 2;
        program_data.resize(MainMemory::MEMORY_SIZE, 0x0);
        MainMemory {
            memory: program_data,
            program_counter: 0,
            program_length: program_length,
        }
    }

    pub fn fetch_opcode(&mut self) -> Option<u16> {
        if (self.program_counter + 2) >= self.memory.len() {
            return None;
        }
        let big_end = self.memory[self.program_counter];
        let little_end = self.memory[self.program_counter + 1];
        let instr = ((big_end as u16) << 8) + (little_end as u16);
        self.program_counter += 2;
        Some(instr)
    }

    pub fn set_program_counter(&mut self, address: u16) {
        self.program_counter = (address - MainMemory::PROGRAM_OFFSET) as usize;
    }

    pub fn peek_program_counter(&self) -> usize {
        self.program_counter + MainMemory::PROGRAM_OFFSET as usize
    }

    pub fn skip_instruction(&mut self) {
        self.program_counter += 2;
    }

    pub fn load_address(&self, address: u16) -> u8 {
        if address > MainMemory::MEMORY_SIZE as u16 {
            panic!("Invalid memory read at address {:#06X}", address);
        }
        if address < MainMemory::PROGRAM_OFFSET {
            MainMemory::FONT_SPRITES[address as usize]
        } else {
            self.memory[(address - MainMemory::PROGRAM_OFFSET) as usize]
        }
    }

    pub fn write_address(&mut self, address: u16, data: u8) {
        if address > MainMemory::MEMORY_SIZE as u16 {
            panic!("Invalid memory read at address {:#06X}", address);
        }
        self.memory[(address - MainMemory::PROGRAM_OFFSET) as usize] = data;
    }

    pub fn slice_program(&self, start: u16, end: u16) -> &[u8] {
        if end < MainMemory::PROGRAM_OFFSET {
            return &MainMemory::FONT_SPRITES[(start as usize)..(end as usize)];
        } else {
            let shifted_start = (start - MainMemory::PROGRAM_OFFSET) as usize;
            let shifted_end = (end - MainMemory::PROGRAM_OFFSET) as usize;
            return &self.memory[shifted_start..shifted_end]
        }
    }
}
