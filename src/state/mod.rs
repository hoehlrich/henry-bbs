use bracket_lib::prelude::*;
use chrono::Utc;

pub struct State {}

impl State {
    fn home(&mut self, ctx: &mut BTerm) {
        ctx.print(2, 1, " %%  %% %%%%%% %%  %% %%%%%  %%  %%  %%%%  %%%%%% %%  %% %%     %%%%%  %%%%%%  %%%%  %%  %%        %%  %% %%  %% %%%%%% ");
        ctx.print(2, 2, " %%  %% %%     %%% %% %%  %%  %%%%  %%  %% %%     %%  %% %%     %%  %%   %%   %%  %% %%  %%         %%%%   %%%%     %%  ");
        ctx.print(2, 3, " %%%%%% %%%%   %% %%% %%%%%    %%   %%  %% %%%%   %%%%%% %%     %%%%%    %%   %%     %%%%%%          %%     %%     %%   ");
        ctx.print(2, 4, " %%  %% %%     %%  %% %%  %%   %%   %%  %% %%     %%  %% %%     %%  %%   %%   %%  %% %%  %%   %%    %%%%    %%    %%    ");
        ctx.print(2, 5, " %%  %% %%%%%% %%  %% %%  %%   %%    %%%%  %%%%%% %%  %% %%%%%% %%  %% %%%%%%  %%%%  %%  %%   %%   %%  %%   %%   %%%%%% ");
        ctx.print(2, 6, "                                                                                                                        ");
        ctx.print(1, 7, "-".repeat(122));
        ctx.printer(40,
                    8,
                    "#[dark_green]2#[white]Line#[] o#[white]n#[]#[] #[green]Modem Zyxel U-90E #[dark_aqua]/#[] IP-Link 100Mbit/s#[]",
                    TextAlign::Left,
                    None,
                    );
        ctx.printer(1,
                    9,
                    format!("#[white]{}| #[dark_aqua]Your System Operator is #[aqua]Henry Oehlrich#[]#[]#[] |{}", "-".repeat(42), "-".repeat(38)),
                    TextAlign::Left,
                    None,
                    );

        ctx.printer(50,
                    10,
                    "#[purple]Dialup Time#[]#[dark_aqua]: 00:00 - 24:00",
                    TextAlign::Left,
                    None,
                    );
        ctx.printer(59,
                    11,
                    "#[purple]IP#[]#[dark_aqua]: 108.61.75.199",
                    TextAlign::Left,
                    None,
                    );
        ctx.printer(58,
                    12,
                    "#[purple]www#[]#[dark_aqua]: https://henryoehlrich.xyz",
                    TextAlign::Left,
                    None,
                    );
        ctx.printer(1,
                    13,
                    format!("#[white]{}|    #[dark_green](#[purple]UTC#[]) {}#[]   |{}", "-".repeat(42), Utc::now(), ("-".repeat(38))),
                    TextAlign::Left,
                    None,
                    );

        let words = vec!["GLOBAL", "SEARCH"];
        for y in 0..7 {
            ctx.print_color(1 as i32, y, RGB::named(WHITE), RGB::named(BLUE), words[0].chars().nth(y).unwrap_or(' '));
            ctx.print_color(123 as i32, y, RGB::named(WHITE), RGB::named(BLUE), words[1].chars().nth(y).unwrap_or(' '));
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.home(ctx);
    }

}
