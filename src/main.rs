pub mod acts;
pub mod chrs;
pub mod custom_string;
pub mod default_formatted;
pub mod game_state;
pub mod game_state_interactable;
pub mod gendered;
pub mod stats;
pub mod trigger_trait;

use game_state::{GameState, Player};
use game_state_interactable::GameStateInteractable;

fn main() {
    let mut game = GameStateInteractable::new(GameState::new(vec![
        Player { nickname: "Shiney".into() },
        Player { nickname: "maxvog".into() },
    ]));

    {
        let player_id = game.state.subturner().player_id;

        let chr_id = game.state.chrs.hand(player_id)[0];
        game.place(chr_id);

        game.state.end_subturn();
    };

    {
        let player_id = game.state.subturner().player_id;

        let chr_id = game.state.chrs.hand(player_id)[0];
        game.place(chr_id);

        println!(
            "before active use:\n{}\n\n{}",
            game.state.chr(game.state.subturner().chr_id.unwrap()),
            game.state.chr(game.state.other_subturner().chr_id.unwrap())
        );

        {
            let act_id = game.state.acts.hand(player_id)[0];
            game.use_on_character(act_id, chr_id);
        }

        println!(
            "\n\nafter active use:\n{}\n\n{}",
            game.state.chr(game.state.subturner().chr_id.unwrap()),
            game.state.chr(game.state.other_subturner().chr_id.unwrap())
        );

        game.state.end_subturn();
    };

    {
        game.state.end_subturn();
    }
}
