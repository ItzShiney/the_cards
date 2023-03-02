pub mod acts;
pub mod chrs;
pub mod custom_string;
pub mod default_formatted;
pub mod described;
pub mod effect;
pub mod game_state;
pub mod gendered;
pub mod host;
pub mod stats;

use chrs::CharacterType;
use game_state::{GameState, Player};
use host::Host;

fn main() {
    for chr_type in CharacterType::all() {
        let mut game = Host::new(GameState::new(vec![
            Player { nickname: "Shiney".into() },
            Player { nickname: "maxvog".into() },
        ]));

        let attacker_id = game.state().attacker().player_id;
        let chr_id = game.state().chrs.hand(attacker_id)[0];
        game.state_mut().chrs.get_mut(chr_id).type_ = chr_type;

        game.place(chr_id);
        println!("{}", game.state().chr(chr_id));
    }

    /*
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
    */
}
