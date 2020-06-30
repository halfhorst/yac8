
/*
    The CHIP-8 stack and stack pointer.
*/
pub struct Stack {
    data: [u16; Stack::NUM_FRAMES],
    pointer: usize,
}

impl Stack {
    const NUM_FRAMES: usize = 16;

    pub fn new() -> Stack {
        Stack {
            data: [0x0; Stack::NUM_FRAMES],
            pointer: 0x0,
        }
    }

    pub fn push(&mut self, data: u16) {
        if self.pointer >= Stack::NUM_FRAMES {
            panic!("Stack Overflow!");
        }
        self.data[self.pointer] = data;
        self.pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        if self.pointer == 0 {
            panic!("Attempted pop from empty stack");
        }

        let val = self.data[self.pointer - 1];
        self.pointer -= 1;
        val
    }
}
