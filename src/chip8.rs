use std::time::Duration;
use rand::{thread_rng, Rng};

use log::{info, warn};

use crate::instructions;
use crate::instructions::Instruction;
use crate::display::Display;
use crate::main_memory::MainMemory;
use crate::registers::Registers;
use crate::stack::Stack;

/*
    The VM proper. This holds all of the VM structures and provides a cycle
    function for progressing the CPU. It also provides a hook for updating
    keystroke information.

    There is currently no support for alternate key mappings. The original hex
    pad layout is mapped tp the upper left region of the keyboard as follows:

    keyboard     hexpad input
    1 2 3 4   |   1 2 3 C
    Q W E R   |   4 5 6 D
    A S D F   |   7 8 9 E
    Z X C V   |   A 0 B F

*/
pub struct Chip8 {
    // Access required for drawing to the screen
    pub display: Display,

    registers: Registers,
    stack: Stack,
    main_memory: MainMemory,
    waiting_on_key: i8,
    key_pressed: [bool; Chip8::NUM_KEYS as usize],
    micros_per_cycle: u32,
    micros_since_cycle: u128,
    micros_per_timer: u32,
    micros_since_timer: u128,
}

impl Chip8 {
    const NUM_KEYS: u8 = 16;
    const TIMER_RATE_HZ: f64 = 60.0;

    pub fn new(program_data: Vec<u8>, clock_speed_hz: f64) -> Chip8 {
        let micros_per_cycle = ((1e6) * (1. / clock_speed_hz)).round() as u32;
        let micros_per_timer = ((1e6) * (1. / Chip8::TIMER_RATE_HZ)).round() as u32;

        Chip8 {
            registers: Registers::new(),
            stack: Stack::new(),
            main_memory: MainMemory::new(program_data),
            display: Display::new(),
            waiting_on_key: -1,  // Stores the register where the keypress is to be stored
            key_pressed: [false; Chip8::NUM_KEYS as usize],
            micros_per_cycle: micros_per_cycle,
            micros_since_cycle: 0,
            micros_per_timer: micros_per_timer,
            micros_since_timer: 0,
        }
    }

    pub fn scan_program(&mut self) {

        for _ in 0..self.main_memory.program_length {
            let opcode = self.main_memory.fetch_opcode();
            match opcode {
                Some(opcode) => {
                    let instruction = instructions::parse_opcode(opcode);
                    println!("{:#06X} => {:X?}", opcode, instruction);
                },
                None => break,
            };
        }
    }

    pub fn cycle(&mut self, elapsed_time: Duration) {
        self.micros_since_cycle += elapsed_time.as_micros();
        self.micros_since_timer += elapsed_time.as_micros();

        if self.micros_since_cycle > self.micros_per_cycle as u128 {
            let cycles = self.micros_since_cycle / (self.micros_per_cycle as u128);
            for _ in 0..cycles {
                if self.waiting_on_key == -1 {
                    let instr = self.fetch();
                    self.execute(instr);
                }
            }
            self.micros_since_cycle = self.micros_since_cycle % (self.micros_per_cycle as u128);
        }

        if self.micros_since_timer > self.micros_per_timer as u128 {
            if self.registers.delay_timer > 0 {
                self.registers.delay_timer -= 1;
            }
            if self.registers.sound_timer > 0 {
                self.registers.sound_timer -= 1;
            }
            self.micros_since_timer = self.micros_since_timer % (self.micros_per_timer as u128);
        }
    }

    fn fetch(&mut self) -> Instruction {
        let opcode = self.main_memory.fetch_opcode();
        let instruction = match opcode {
            Some(opcode) => {
                let instruction = instructions::parse_opcode(opcode);
                info!("{:#06X} => {:X?}", opcode, instruction);
                instruction
            },
            None => panic!("End of ROM."),
        };
        instruction
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ClearScreen => {
                self.display.clear();
            },
            Instruction::Return => {
                let address = self.stack.pop();
                self.main_memory.set_program_counter(address);
            },
            Instruction::Jump(address) => {
                self.main_memory.set_program_counter(address);
            },
            Instruction::Call(address) => {
                let pc = self.main_memory.peek_program_counter();
                self.stack.push(pc as u16);
                self.main_memory.set_program_counter(address);
            },
            Instruction::SkipIfEQData(register, data) => {
                if self.registers.read_data_register(register) == data {
                    self.main_memory.skip_instruction();
                }
            },
            Instruction::SkipIfNEData(register, data) => {
                if self.registers.read_data_register(register) != data {
                    self.main_memory.skip_instruction();
                }
            },
            Instruction::SkipIfEQRegister(register_1, register_2) => {
                if self.registers.read_data_register(register_1) == self.registers.read_data_register(register_2) {
                    self.main_memory.skip_instruction();
                }
            },
            Instruction::LoadData(register, data) => {
                self.registers.write_data_register(register, data)
            },
            Instruction::AddData(register, data) => {
                let register_data = self.registers.read_data_register(register);
                self.registers.write_data_register(register, register_data.wrapping_add(data));
            },
            Instruction::LoadRegister(register_1, register_2) => {
                let data = self.registers.read_data_register(register_2);
                self.registers.write_data_register(register_1, data);
            },
            Instruction::Or(register_1, register_2) => {
                self.registers.write_data_register(register_1,
                    self.registers.read_data_register(register_1) | self.registers.read_data_register(register_2));
            },
            Instruction::And(register_1, register_2) => {
                self.registers.write_data_register(register_1, self.registers.read_data_register(register_1)
                                                               & self.registers.read_data_register(register_2));
            },
            Instruction::Xor(register_1, register_2) => {
                self.registers.write_data_register(register_1,
                    self.registers.read_data_register(register_1) ^ self.registers.read_data_register(register_2));
            },
            Instruction::Add(register_1, register_2) => {
                let register_1_data = self.registers.read_data_register(register_1) as u16;
                let register_2_data = self.registers.read_data_register(register_2) as u16;
                let sum = register_1_data + register_2_data;
                self.registers.write_data_register(0xF, (sum > 255) as u8);
                self.registers.write_data_register(register_1, sum as u8)

            },
            Instruction::Sub(register_1, register_2) => {
                let register_1_data = self.registers.read_data_register(register_1);
                let register_2_data = self.registers.read_data_register(register_2);
                self.registers.write_data_register(0xF, (register_1_data > register_2_data) as u8);
                self.registers.write_data_register(register_1, register_1_data.wrapping_sub(register_2_data));
            },
            Instruction::ShiftRight(register) => {
                let data = self.registers.read_data_register(register);
                self.registers.write_data_register(0xF, data & 0x1);
                self.registers.write_data_register(register, data >> 1);
            },
            Instruction::NegatedSub(register_1, register_2) => {
                let register_1_data = self.registers.read_data_register(register_1);
                let register_2_data = self.registers.read_data_register(register_2);
                self.registers.write_data_register(0xF, (register_2_data > register_1_data) as u8);
                self.registers.write_data_register(register_1, register_2_data.wrapping_sub(register_1_data));
            },
            Instruction::ShiftLeft(register) => {
                let data = self.registers.read_data_register(register);
                self.registers.write_data_register(0xF, data >> 7);
                self.registers.write_data_register(register, data << 1);
            },
            Instruction::SkipIfNERegister(register_1, register_2) => {
                let register_1_data = self.registers.read_data_register(register_1);
                let register_2_data = self.registers.read_data_register(register_2);
                if register_1_data != register_2_data {
                    self.main_memory.skip_instruction();
                }
            },
            Instruction::SetI(value) => {
                self.registers.i_register = value;
            },
            Instruction::JumpFromOffset(address) => {
                let offset = self.registers.read_data_register(0x0);
                self.main_memory.set_program_counter(offset as u16 + address);
            },
            Instruction::Random(register, data) => {
                let mut rng = thread_rng();
                let n: u8 = rng.gen_range(0, 255);
                self.registers.write_data_register(register, n & data);
            },
            Instruction::Draw(x, y, data) => {
                let start_sprite = self.registers.i_register;
                let end_sprite = start_sprite + (data as u16);
                let collision = self.display.draw(self.registers.read_data_register(x),
                                                  self.registers.read_data_register(y),
                                                  self.main_memory.slice_program(start_sprite,
                                                                                 end_sprite));
                self.registers.write_data_register(0xF, collision as u8);
            },
            Instruction::SkipIfPressed(register) => {
                let key = self.registers.read_data_register(register);
                if self.key_pressed[key as usize] == true {
                    self.main_memory.skip_instruction();
                }
            },
            Instruction::SkipIfNotPressed(register) => {
                let key = self.registers.read_data_register(register);
                if key < Chip8::NUM_KEYS {
                    if !(self.key_pressed[key as usize]) {
                        self.main_memory.skip_instruction();
                    }
                } else {
                    panic!("Invalid key expected");
                }
            },
            Instruction::SetRegisterFromDelay(register) => {
                self.registers.write_data_register(register, self.registers.delay_timer)
            },
            Instruction::AwaitPress(register) => {
                self.waiting_on_key = register as i8;
            },
            Instruction::SetDelayFromRegister(register) => {
                self.registers.delay_timer = self.registers.read_data_register(register);
            },
            Instruction::SetSoundFromRegister(_) => {
                warn!("Sound is not implemented.");
            },
            Instruction::AddI(register) => {
                self.registers.i_register += self.registers.read_data_register(register) as u16;
            },
            Instruction::LoadSprite(register) => {
                self.registers.i_register = 5 * self.registers.read_data_register(register) as u16;
            },
            Instruction::SetBCDRepresentation(register) => {
                let data = self.registers.read_data_register(register);
                self.main_memory.write_address(self.registers.i_register, (data / 100) % 10);
                self.main_memory.write_address(self.registers.i_register + 1, (data / 10) % 10);
                self.main_memory.write_address(self.registers.i_register + 2, data % 10);
            },
            Instruction::StoreRegisters(high_register) => {
                // info!("{:X?}", instruction);
                let base = self.registers.i_register;
                for register in 0..(high_register + 1) {
                    self.main_memory.write_address(base + register as u16,
                                                   self.registers.read_data_register(register))
                }
            },
            Instruction::ReadRegisters(high_register) => {
                let base = self.registers.i_register;
                for register in 0..(high_register + 1) {
                    self.registers.write_data_register(register, self.main_memory.load_address(base + register as u16))
                }
            },
            Instruction::NOP(_) => {},
            Instruction::UNKNOWN(data) => panic!("Unknown instruction encountered: {:X?}", data),
        }
    }

    pub fn update_key(&mut self, key: String, is_pressed: bool) {
        info!("Parsing keystroke {}, is_pressed: {}", key, is_pressed);
        let keycode = Chip8::match_key(key);
        match keycode {
            Some(code) => {
                self.key_pressed[code as usize] = is_pressed;
                if self.waiting_on_key != -1 && is_pressed {
                    self.registers.write_data_register(self.waiting_on_key as u8, code);
                    self.waiting_on_key = -1;
                }
            },
            None => {}
        }
    }

    pub fn match_key(key: String) -> Option<u8> {
        match key.as_str() {
            "1" => {
                Some(0x1)
            },
            "2" => {
                Some(0x2)
            },
            "3" => {
                Some(0x3)
            },
            "4" => {
                Some(0xC)
            },
            "Q" => {
                Some(0x4)
            },
            "W" => {
                Some(0x5)
            },
            "E" => {
                Some(0x6)
            },
            "R" => {
                Some(0xD)
            },
            "A" => {
                Some(0x7)
            },
            "S" => {
                Some(0x8)
            },
            "D" => {
                Some(0x9)
            },
            "F" => {
                Some(0xE)
            },
            "Z" => {
                Some(0xA)
            },
            "X" => {
                Some(0x0)
            },
            "C" => {
                Some(0xB)
            },
            "V" => {
                Some(0xF)
            },
            _ => { None }
        }
    }
}
