type Register = u8;
type Data = u8;
type Address = u16;

#[derive(Debug)]
pub enum Instruction {
    ClearScreen,
    Return,
    Jump(Address),
    Call(Address),
    SkipIfEQData(Register, Data),
    SkipIfNEData(Register, Data),
    SkipIfEQRegister(Register, Register),
    LoadData(Register, Data),
    AddData(Register, Data),
    LoadRegister(Register, Register),
    Or(Register, Register),
    And(Register, Register),
    Xor(Register, Register),
    Add(Register, Register),
    Sub(Register, Register),
    ShiftRight(Register),
    NegatedSub(Register, Register),
    ShiftLeft(Register),
    SkipIfNERegister(Register, Register),
    SetI(Address),
    JumpFromOffset(Address),
    Random(Register, Data),
    Draw(Register, Register, Data),
    SkipIfPressed(Register),
    SkipIfNotPressed(Register),
    SetRegisterFromDelay(Register),
    AwaitPress(Register),
    SetDelayFromRegister(Register),
    SetSoundFromRegister(Register),
    AddI(Register),
    LoadSprite(Register),
    SetBCDRepresentation(Register),
    StoreRegisters(Register),
    ReadRegisters(Register),
    NOP(u16),
    UNKNOWN(u16),
}

/*
    parse a big endian, 2-byte opcode into its corresponding CHIP-8
    instruction.
*/
pub fn parse_opcode(bytes: u16) -> Instruction {
    match bytes & 0xF000 {
        0x0000 => match bytes & 0x00FF {
            0x00E0 => Instruction::ClearScreen,
            0x00EE => Instruction::Return,
            // 0x0nnn is `jump to machine code routine`, ignored
            _ => Instruction::NOP(bytes)
        }

        0x1000 => Instruction::Jump(mask_address(bytes)),
        0x2000 => Instruction::Call(mask_address(bytes)),
        0x3000 => Instruction::SkipIfEQData(mask_high_register(bytes),
                                            mask_data(bytes)),
        0x4000 => Instruction::SkipIfNEData(mask_high_register(bytes),
                                            mask_data(bytes)),
        0x5000 => match bytes & 0x000F {
            0x0000 => Instruction::SkipIfEQRegister(mask_high_register(bytes),
                                                    mask_low_register(bytes)),
            _ => Instruction::UNKNOWN(bytes),
        },
        0x6000 => Instruction::LoadData(mask_high_register(bytes),
                                        mask_data(bytes)),
        0x7000 => Instruction::AddData(mask_high_register(bytes),
                                       mask_data(bytes)),
        0x8000 => match bytes & 0x000F {
            0x0000 => Instruction::LoadRegister(mask_high_register(bytes),
                                                mask_low_register(bytes)),
            0x0001 => Instruction::Or(mask_high_register(bytes),
                                      mask_low_register(bytes)),
            0x0002 => Instruction::And(mask_high_register(bytes),
                                       mask_low_register(bytes)),
            0x0003 => Instruction::Xor(mask_high_register(bytes),
                                       mask_low_register(bytes)),
            0x0004 => Instruction::Add(mask_high_register(bytes),
                                       mask_low_register(bytes)),
            0x0005 => Instruction::Sub(mask_high_register(bytes),
                                       mask_low_register(bytes)),
            0x0006 => Instruction::ShiftRight(mask_high_register(bytes)),
            0x0007 => Instruction::NegatedSub(mask_high_register(bytes),
                                              mask_low_register(bytes)),
            0x000E => Instruction::ShiftLeft(mask_high_register(bytes)),
            _ => Instruction::UNKNOWN(bytes),
        },
        0x9000 => match bytes & 0x000F {
            0x0000 => Instruction::SkipIfNERegister(mask_high_register(bytes),
                                                    mask_low_register(bytes)),
            _ => Instruction::UNKNOWN(bytes),
        },
        0xA000 => Instruction::SetI(mask_address(bytes)),
        0xB000 => Instruction::JumpFromOffset(mask_address(bytes)),
        0xC000 => Instruction::Random(mask_high_register(bytes),
                                      mask_data(bytes)),
        0xD000 => Instruction::Draw(mask_high_register(bytes),
                                    mask_low_register(bytes),
                                    mask_data(bytes & 0x000F)),
        0xE000 => match bytes & 0x00FF {
            0x009E => Instruction::SkipIfPressed(mask_high_register(bytes)),
            0x00A1 => Instruction::SkipIfNotPressed(mask_high_register(bytes)),
            _ => Instruction::UNKNOWN(bytes),
        },
        0xF000 => match bytes & 0x00FF {
            0x0007 => Instruction::SetRegisterFromDelay(mask_high_register(bytes)),
            0x000A => Instruction::AwaitPress(mask_high_register(bytes)),
            0x0015 => Instruction::SetDelayFromRegister(mask_high_register(bytes)),
            0x0018 => Instruction::SetSoundFromRegister(mask_high_register(bytes)),
            0x001E => Instruction::AddI(mask_high_register(bytes)),
            0x0029 => Instruction::LoadSprite(mask_high_register(bytes)),
            0x0033 => Instruction::SetBCDRepresentation(mask_high_register(bytes)),
            0x0055 => Instruction::StoreRegisters(mask_high_register(bytes)),
            0x0065 => Instruction::ReadRegisters(mask_high_register(bytes)),
            _ => Instruction::UNKNOWN(bytes),
        }
        _ => Instruction::UNKNOWN(bytes)
    }
}

// rylev has a super clean way of going about this
// -> https://github.com/rylev/Rust-8/blob/master/src/instruction.rs

fn mask_address(bytes: u16) -> Address {
    bytes & 0x0FFF
}

fn mask_high_register(bytes: u16) -> Register {
    ((bytes & 0x0F00) >> 8) as u8
}

fn mask_low_register(bytes: u16) -> Register {
    ((bytes & 0x00F0) >> 4) as u8
}

fn mask_data(bytes: u16) -> Data {
    (bytes & 0x00FF) as u8
}
