use std::f32::consts::PI;

use raylib::{
    ffi::Color,
    prelude::{RaylibDraw, RaylibTextureModeExt},
};

use lsystems::{draw::TextureDrawer, parser};

fn main() {
    let width = 1920;
    let height = 1080;

    let (mut rl, th) = raylib::init().size(width, height).build();

    let mut texture = rl
        .load_render_texture(&th, width as u32, height as u32)
        .unwrap();

    let mut executor = parser::new(100, PI / 2.0);

    let mut draw_guard = TextureDrawer {
        mode: rl.begin_texture_mode(&th, &mut texture),
    };

    draw_guard.mode.clear_background(Color::WHITE);

    for _ in 0..100 {
        match executor.execute("F+F-", &mut draw_guard) {
            Ok(_) => (),
            Err(_) => panic!("AAAAH"),
        }
    }

    drop(draw_guard);

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing(&th);
        draw.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}
