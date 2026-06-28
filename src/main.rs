use std::{collections::HashMap, f32::consts::PI};

use raylib::{
    ffi::Color,
    prelude::{RaylibDraw, RaylibTextureModeExt},
};

use lsystems::{
    draw::TextureDrawer,
    lsystem::{self, LSystem},
    parser,
};

fn main() {
    let width = 1920;
    let height = 1080;

    let koch_system = lsystem::new(&[("F", "F+F-F-F+F")], "F");

    let expanded = koch_system.expand(5);

    let (mut rl, th) = raylib::init().size(width, height).build();

    let mut texture = rl
        .load_render_texture(&th, width as u32, height as u32)
        .unwrap();

    let mut executor = parser::new(10, PI / 2.0);

    let mut draw_guard = TextureDrawer {
        mode: rl.begin_texture_mode(&th, &mut texture),
    };

    draw_guard.mode.clear_background(Color::WHITE);

    executor.execute(&expanded, &mut draw_guard).unwrap();

    drop(draw_guard);

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing(&th);
        draw.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}
