use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "2Line on Modem Zyxel U-90E / IP-Link 100Mbit/s");
        ctx.print(1, 5, "------| Your System Operator is Henry Oehlrich");
        ctx.print(1, 7, "108.61.75.199    Dialup Time: 00:00 - 24:00");
        ctx.print(1, 8, "                     IP Time: EST");
        ctx.print(1, 9, "                         www: https://henryoehlrich.xyz");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .with_fullscreen(true)
        .build()?;

    let gs: State = State {};
    main_loop(context, gs)
}
