#![allow(clippy::uninlined_format_args)]
pub mod acts;
pub mod chrs;
pub mod custom_string;
pub mod default_formatted;
pub mod effect;
pub mod game_state;
pub mod group;
pub mod host;
pub mod stats;

use acts::ActiveType;
use chrs::CharacterType;
use game_state::{GameState, Player};
use host::Host;

use crate::game_state::{act_info::ActiveInfo, chr_info::CharacterInfo};

pub fn print_chrs() {
    for chr_type in CharacterType::all() {
        println!("{}", CharacterInfo::new(chr_type));
    }
}

pub fn print_acts() {
    for act_type in ActiveType::all() {
        println!("{}", ActiveInfo::new(act_type));
    }
}

fn main() {
    #[allow(unused)]
    let game = Host::new(GameState::new(vec![
        Player { nickname: "Shiney".into() },
        Player { nickname: "maxvog".into() },
    ]));

    println!("{}", ActiveInfo::new(ActiveType::all().pop().unwrap()));
}
