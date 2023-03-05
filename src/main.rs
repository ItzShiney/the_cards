#![allow(clippy::uninlined_format_args)]
pub mod acts;
pub mod chrs;
pub mod custom_string;
pub mod default_formatted;
pub mod effect;
pub mod game_state;
pub mod host;
pub mod stats;

use acts::ActiveType;
use chrs::CharacterType;
use game_state::{GameState, Player};
use host::Host;

use crate::game_state::{act_info::ActiveInfo, chr_info::CharacterInfo};

fn main() {
    #[allow(unused)]
    let game = Host::new(GameState::new(vec![
        Player { nickname: "Shiney".into() },
        Player { nickname: "maxvog".into() },
    ]));

    for chr_type in CharacterType::all() {
        println!("{}", CharacterInfo::new(chr_type));
    }

    for act_type in ActiveType::all() {
        println!("{}", ActiveInfo::new(act_type));
    }
}
