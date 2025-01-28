use crate::vm::{Machine, Register};
use raylib::prelude::*;

const TILE_SIZE: i32 = 40;

pub fn run(program: [u8; 256]) {
    let keys = [
        KeyboardKey::KEY_LEFT,
        KeyboardKey::KEY_RIGHT,
        KeyboardKey::KEY_DOWN,
        KeyboardKey::KEY_UP,
    ];
    let (mut rl, thread) = raylib::init()
        .size(16 * TILE_SIZE, 16 * TILE_SIZE)
        .title("leq8")
        .build();
    rl.set_target_fps(8);
    //rl.disable_cursor();

    let mut machine = Machine::new();
    machine.load_program(program);

    while !rl.window_should_close() {
        let mut inp = 0u8;
        for (i, key) in keys.into_iter().enumerate() {
            if rl.is_key_down(key) {
                inp |= 1 << i;
            }
        }

        machine.write_reg(Register::INP, inp);
        let Ok(pixels) = machine.loop_till_flush() else {
            break;
        };

        let mut draw = rl.begin_drawing(&thread);

        draw.clear_background(Color::BLACK);

        for (x, y) in (0..16).flat_map(|x| (0..16).map(move |y| (x, y))) {
            let pix = pixels[x as usize + 16 * y as usize];
            let r = ((pix & 0b0011_0000) >> 4) * 85;
            let g = ((pix & 0b0000_1100) >> 2) * 85;
            let b = (pix & 0b0000_0011) * 85;
            let color = Color { r, g, b, a: 255 };
            draw.draw_rectangle(
                x * TILE_SIZE,
                (15 - y) * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
                color,
            );
        }
    }
}
