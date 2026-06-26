use raylib::{
    RaylibHandle,
    ffi::Color,
    prelude::{RaylibDraw, RaylibTextureMode},
};

pub trait Draw {
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32);
}

pub struct TextureDrawer<'a> {
    pub mode: RaylibTextureMode<'a, 'a, RaylibHandle>,
}

impl<'a> Draw for TextureDrawer<'a> {
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        self.mode.draw_line(x0, y0, x1, y1, Color::BLACK);
    }
}
