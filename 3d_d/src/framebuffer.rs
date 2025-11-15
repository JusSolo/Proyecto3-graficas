use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Color>,
    background_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Framebuffer {
            width,
            height,
            pixels: vec![Color::BLACK; (width * height) as usize],
            background_color: Color::BLACK,
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(self.background_color);
    }

    pub fn clear_with_color(&mut self, color: Color) {
        self.background_color = color;
        self.pixels.fill(color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            let index = (y as u32 * self.width + x as u32) as usize;
            self.pixels[index] = color;
        }
    }

    pub fn draw_to(&self, d: &mut RaylibDrawHandle) {
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let index = (y as u32 * self.width + x as u32) as usize;
                d.draw_pixel(x, y, self.pixels[index]);
            }
        }
    }
}
