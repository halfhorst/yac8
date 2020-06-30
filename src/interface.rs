/*
    All SDL related audio/video and windowed input.
*/
use sdl2::pixels::Color;
use sdl2::Sdl;
use sdl2::EventPump;
use sdl2::render;
use sdl2::video::Window;
use sdl2::rect::Point;

/*
    The audio-video context for the emulator. It's all SDL hidden in this
    struct.
*/
pub struct AVInterface {
    pub sdl_context: Sdl,
    pub event_pump: EventPump,
    pub canvas: render::Canvas<Window>,
    width: u32,
    height: u32
}

impl AVInterface {
    pub fn new(width: u32, height: u32) -> AVInterface {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("yac8", width * 10, height * 10)
                                    .position_centered()
                                    .opengl()
                                    .build()
                                    .unwrap();

        let mut canvas = window.into_canvas()
                               .build()
                               .unwrap();

        canvas.set_logical_size(width, height).expect("Failed to set logical size of SDL2 renderer.");

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();

        AVInterface {
            sdl_context: sdl_context,
            event_pump: event_pump,
            canvas: canvas,
            width: width,
            height: height
        }
    }

    pub fn draw(&mut self, buffer: &[u8]) {
        for (num, &bit) in buffer.iter().enumerate() {
            if bit == 1 {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            } else {
                self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            let y = (num as u32) / self.width;
            let x = (num as u32) % self.width;
            self.canvas.draw_point(Point::new(x as i32, y as i32)).expect("Failed to draw");
        }
    }
}
