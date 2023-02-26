pub mod acts;
pub mod chrs;
pub mod custom_string;
pub mod default_formatted;
pub mod game_state;
pub mod gendered;
pub mod host;
pub mod stats;
pub mod trigger_trait;

use game_state::{GameState, Player};
use host::Host;

fn main() {
    let mut game = Host::new(GameState::new(vec![
        Player { nickname: "Shiney".into() },
        Player { nickname: "maxvog".into() },
    ]));

    let player_id = game.state().current_subturner_on_field().player_id;

    let chr_id = game.state().chrs.hand(player_id)[0];
    println!("{}", game.state().chr(chr_id));

    let act_id = game.state().acts.hand(player_id)[0];
    println!("{}", game.state().act(act_id));

    if false {
        {
            let player_id = game.state().current_subturner_on_field().player_id;

            let chr_id = game.state().chrs.hand(player_id)[0];
            game.place(chr_id);

            game.end_subturn();
        };

        {
            let player_id = game.state().current_subturner_on_field().player_id;

            let chr_id = game.state().chrs.hand(player_id)[0];
            game.place(chr_id);

            game.end_subturn();
        };

        {
            game.end_turn();
        }
    }
}
