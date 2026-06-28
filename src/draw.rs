use raylib::{
    RaylibHandle,
    ffi::{Color, Vector2},
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
        self.mode.draw_line_ex(
            Vector2 {
                x: x0 as f32,
                y: y0 as f32,
            },
            Vector2 {
                x: x1 as f32,
                y: y1 as f32,
            },
            3.0,
            Color::BLACK,
        );
        println!("({x0}, {y0}) -> ({x1}, {y1})")
    }
}
