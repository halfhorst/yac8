/*
    The CHIP-8 data registers, `I` register, and timer registers.
*/
pub struct Registers {
    data: [u8; Registers::NUM_DATA_REGISTERS as usize],
    pub i_register: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Registers {
    const NUM_DATA_REGISTERS: u8 = 16;

    pub fn new() -> Registers {
        Registers {
            data: [0x0; Registers::NUM_DATA_REGISTERS as usize],
            i_register: 0x0,
            delay_timer: 0x0,
            sound_timer: 0x0
        }
    }

    pub fn read_data_register(&self, register: u8) -> u8 {
        Registers::validate_data_register(register);
        self.data[register as usize]
    }

    pub fn write_data_register(&mut self, register: u8, data: u8) {
        Registers::validate_data_register(register);
        self.data[register as usize] = data;
    }

    pub fn validate_data_register(register: u8) {
        if register > Registers::NUM_DATA_REGISTERS {
            panic!("Attempting to access invalid register.");
        }
    }
}
