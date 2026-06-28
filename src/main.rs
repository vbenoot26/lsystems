use std::f32::consts::PI;

use raylib::{
    ffi::{Color, Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibTextureModeExt},
};

use lsystems::{
    draw::TextureDrawer,
    lsystem::{self},
    parser,
};

fn main() {
    let width = 10000;

    let koch_system = lsystem::new(&[("X", "F+[[X]-X]-F[-FX]+X"), ("F", "FF")], "-X");

    let expanded = koch_system.expand(8);

    let (mut rl, th) = raylib::init().size(1920, 1080).fullscreen().build();

    let dest_width = rl.get_screen_width();
    let dest_height = rl.get_screen_height();

    let ratio = dest_height as f32 / dest_width as f32;
    let height = width as f32 * ratio;

    let mut texture = rl
        .load_render_texture(&th, width as u32, height as u32)
        .unwrap();

    let mut executor = parser::new(10, 5.0 * PI / 36.0, (100, 100), PI / 2.0);

    let mut draw_guard = TextureDrawer {
        mode: rl.begin_texture_mode(&th, &mut texture),
    };

    draw_guard.mode.clear_background(Color::WHITE);

    executor.execute(&expanded, &mut draw_guard).unwrap();

    drop(draw_guard);

    while !rl.window_should_close() {
        let src = Rectangle {
            x: 0.0,
            y: 0.0,
            width: width as f32,
            height: height as f32,
        };

        let dest = Rectangle {
            x: 0.0,
            y: 0.0,
            width: dest_width as f32,
            height: dest_height as f32,
        };

        let mut draw = rl.begin_drawing(&th);
        draw.draw_texture_pro(
            &texture,
            src,
            dest,
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        );
    }
}
