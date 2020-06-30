/*
    The CHIP-8 display at the original 64x48 resolution. This display supports
    drawing binary sprite data and is used as a display buffer.
*/
pub struct Display {
    pub buffer: [u8; Display::SIZE],
 }

 impl Display {
     pub const WIDTH: u16 = 64;
     pub const HEIGHT: u16 = 32;
     pub const SIZE: usize = (Display::WIDTH * Display::HEIGHT) as usize;

     pub fn new() -> Display {
         Display {
             buffer: [0x0; Display::SIZE]
         }
     }

     pub fn clear(&mut self) {
         self.buffer = [0x0; Display::SIZE];
     }

     pub fn draw(&mut self, x: u8, y: u8, sprite_data: &[u8]) -> bool {
         let mut erased = false;

         for (y_iter, byte) in sprite_data.iter().enumerate() {
             let current_y = (y + y_iter as u8) as u16 % Display::HEIGHT;

             for bit_num in 0..8 {
                 let current_x = (x + bit_num as u8) as u16 % Display::WIDTH;
                 let buffer_index = ((current_y * Display::WIDTH) + current_x) as usize;

                 let old_pixel = self.buffer[buffer_index];
                 let current_bit = (byte >> (7 - bit_num)) & 1;  // isolate the nth bit
                 let new_pixel = current_bit ^ old_pixel;

                 self.buffer[buffer_index] = new_pixel;

                 if old_pixel == 1 && new_pixel == 0 {
                     erased = true;
                 }
             }
         }

         erased
     }
 }
