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

    {
        let player_id = game.state().subturner().player_id;

        let chr_id = game.state().chrs.hand(player_id)[0];
        game.place(chr_id);

        game.end_subturn();
    };

    {
        let player_id = game.state().subturner().player_id;

        let chr_id = game.state().chrs.hand(player_id)[0];
        game.place(chr_id);

        let other_chr_id = game.state().other_subturner().chr_id.unwrap();
        println!("{}", game.state().chr(other_chr_id));
        /*
        println!("{}\n+", game.state().chr(other_chr_id));
        {
            let act_id = game.state().acts.hand(player_id)[0];
            println!("{}\n=", game.state().act(act_id));
            game.use_on_character(act_id, other_chr_id);
        }
        println!("{}", game.state().chr(other_chr_id));
        */

        game.end_subturn();
    };

    {
        game.end_turn();
    }
}
