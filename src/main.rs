use bracket_lib::prelude::*;

use henry_bbs::State;

const WHITE: (u8, u8, u8) = (255, 255, 255);
const DARK_GRAY: (u8, u8, u8) = (100, 100, 100);
const BROWN: (u8, u8, u8) = (170, 30, 0);
const GREEN: (u8, u8, u8) = (8, 255, 8);
const DARK_GREEN: (u8, u8, u8) = (0, 170, 0);
const BLUE: (u8, u8, u8) = (0, 0, 255);
const AQUA: (u8, u8, u8) = (0, 255, 255);
const DARK_AQUA: (u8, u8, u8) = (36, 157, 159);
const BLACK: (u8, u8, u8) = (0, 0, 0);
const PURPLE: (u8, u8, u8) = (135, 31, 120);

fn main() -> BError {
    register_palette_color("white", RGB::named(WHITE));
    register_palette_color("green", RGB::named(GREEN));
    register_palette_color("brown", RGB::named(BROWN));
    register_palette_color("dark_gray", RGB::named(DARK_GRAY));
    register_palette_color("dark_green", RGB::named(DARK_GREEN));
    register_palette_color("blue", RGB::named(BLUE));
    register_palette_color("aqua", RGB::named(AQUA));
    register_palette_color("dark_aqua", RGB::named(DARK_AQUA));
    register_palette_color("black", RGB::named(BLACK));
    register_palette_color("purple", RGB::named(PURPLE));

    let tw = 11;
    let th = 19;

    let size = (1366 / tw, 768 / th);

    println!("{} {}", size.0, size.1);
    let ctx = BTermBuilder::simple(size.0, size.1)?
        .with_font("vga8x16.png", tw, th)
        .with_advanced_input(true)
        .with_fps_cap(60.0)
        .with_fullscreen(true)
        .with_title("Henry BBS")
        .build()?;

    let gs: State = State { };
    main_loop(ctx, gs)
}
