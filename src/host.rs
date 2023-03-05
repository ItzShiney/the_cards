use std::mem::take;

use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};

use crate::acts::ActiveType;
use crate::chrs::CharacterType;
use crate::described::Described;
use crate::game_state::act_id::ActiveID;
use crate::game_state::act_info::ActiveInfo;
use crate::game_state::chr_id::CharacterID;
use crate::game_state::chr_info::CharacterInfo;
use crate::game_state::group::Group;
use crate::game_state::player_id::PlayerID;
use crate::game_state::{GameState, Subturner};
use crate::stats::{Stat, Stat0, StatValue};

pub struct Host {
    pub callbacks: GameCallbacks,
    state: GameState,
}

impl Host {
    pub fn new(state: GameState) -> Self {
        let mut res = Self { callbacks: Default::default(), state };
        res.init_cards();
        res
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    // TODO: remove
    pub fn state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }
}

impl Host {
    const INIT_CHARACTERS_PER_HAND: usize = 3;
    const INIT_ACTIVES_PER_HAND: usize = 6;

    const CHARACTERS_GAINED_AFTER_TURN: usize = 2;
    const ACTIVES_GAINED_AFTER_TURN: usize = 4;

    const TOTAL_GAINS_PER_PLAYER: usize = 4;

    fn init_cards(&mut self) {
        self.init_gain_pile();

        for player_id in self.state.players_map.keys().copied() {
            self.state.chrs.pick_n(player_id, Self::INIT_CHARACTERS_PER_HAND);
            self.state.acts.pick_n(player_id, Self::INIT_ACTIVES_PER_HAND);
        }
    }

    fn init_gain_pile(&mut self) {
        let players_count = self.state.players_map.len();

        let total_chrs_count = players_count
            * (Self::INIT_CHARACTERS_PER_HAND
                + Self::CHARACTERS_GAINED_AFTER_TURN * Self::TOTAL_GAINS_PER_PLAYER);

        let total_acts_count = players_count
            * (Self::INIT_ACTIVES_PER_HAND
                + Self::ACTIVES_GAINED_AFTER_TURN * Self::TOTAL_GAINS_PER_PLAYER);

        for _ in 1..=total_chrs_count {
            let chr_type = CharacterType::all()
                .into_iter()
                .filter(|&chr_type| !chr_type.groups().contains(&Group::Undrawable))
                .choose(&mut thread_rng())
                .unwrap();

            let chr = CharacterInfo::new(chr_type);

            let chr_id = self.state.add_chr(chr);
            self.state.chrs.add_to_drawpile(chr_id);
        }

        for _ in 1..=total_acts_count {
            let act_type = ActiveType::all()
                .into_iter()
                .filter(|&act_type| !act_type.groups().contains(&Group::Undrawable))
                .choose(&mut thread_rng())
                .unwrap();

            let act = ActiveInfo::new(act_type);

            let act_id = self.state.add_act(act);
            self.state.acts.add_to_drawpile(act_id);
        }
    }
}

// TODO: вынести в отдельный файл
pub enum Chain<Continue, Result = ()> {
    Continue(Continue),
    Break(Result),
}

// TODO: вынести в отдельный файл всё ниже?
macro_rules! callbacks {
    (
        $(
            $(#[@$self_namespace:ident])?
            $(#[pre($pre_value:expr)])?
            pub fn $name:ident(
                &mut $self:ident
                $(
                    , $arg_name:ident : $ArgType:ty
                )* $(,)?
            ) $(-> $Return:ty)? $callback_action:block
        )*
    ) => {paste::paste! {
        $(
            #[derive(Clone)]
            pub struct [<$name:camel Args>] {
                $(pub $arg_name: $ArgType,)*
            }

            pub type [<$name:camel Callback>] = fn(&mut Host, [<$name:camel Args>]) -> Chain<[<$name:camel Args>], $($Return)?>;
            pub type [<Post $name:camel Callback>] = fn(&mut Host, &[<$name:camel Args>]);
        )*

        #[derive(Default)]
        pub struct GameCallbacks {
            $(
                pub $name: Option<$crate::described::Described<[<$name:camel Callback>]>>,
                pub [<post_ $name>]: Option<$crate::described::Described<[<Post $name:camel Callback>]>>,
            )*
        }

        impl ::std::fmt::Display for GameCallbacks {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                $(
                    if let Some($crate::described::Described { ref description, .. }) = self.$name {
                        writeln!(f, "{}", description)?;
                    }
                )*

                Ok(())
            }
        }

        impl Host {
            $(
                pub fn [<$name:camel:snake _args>] (&mut $self, #[allow(unused_mut)] mut args: [<$name:camel Args>] ) $(-> $Return)? {
                    $(
                        const _: () = assert!($pre_value);

                        while let Some($crate::described::Described { value: callback, .. }) = $self.callbacks.$name {
                            match (callback)($self, args) {
                                Chain::Continue(new_args) => {
                                    args = new_args;
                                }

                                Chain::Break(result) => return result,
                            }
                        }
                    )?

                    #[allow(unused)] let id = ($(args.$arg_name,)* 0,).0;
                    $(
                        if let Some($crate::described::Described { value: callback, .. }) = $self.state.$self_namespace.get(id).type_.abilities().$name {
                            match (callback)($self, args) {
                                Chain::Continue(new_args) => {
                                    args = new_args;
                                }

                                Chain::Break(result) => return result,
                            }
                        }
                    )?
                    #[allow(unused)] let id = ();

                    #[allow(clippy::redundant_closure_call)]
                    let res = (|| {
                        #[allow(unused)]
                        let [<$name:camel Args>] { $($arg_name,)* } = args.clone();

                        $callback_action
                    })();

                    $self.[<post_ $name:camel:snake _args>](&args);
                    res
                }

                pub fn [<post_ $name:camel:snake _args>] (&mut $self, args: &[<$name:camel Args>] ) {
                    while let Some($crate::described::Described { value: callback, .. }) = $self.callbacks.[<post_ $name>] {
                        (callback)($self, args);
                    }

                    #[allow(unused)] let id = ($(args.$arg_name,)* 0,).0;
                    $(
                        if let Some($crate::described::Described { value: callback, .. }) = $self.state.$self_namespace.get(id).type_.abilities().[<post_ $name>] {
                            (callback)($self, args);
                        }
                    )?
                    #[allow(unused)] let id = ();
                }

                pub fn $name (&mut $self, $($arg_name: $ArgType,)* ) $(-> $Return)? {
                    $self.[<$name:camel:snake _args>]([<$name:camel Args>] { $($arg_name,)* })
                }
            )*
        }
    }};
}

callbacks! {
    #[@acts]
    #[pre(true)]
    pub fn use_on_field(
        &mut self,
        act_id: ActiveID,
    ) {
        todo!()
    }

    #[@acts]
    #[pre(true)]
    pub fn use_on_character(
        &mut self,
        act_id: ActiveID,
        target_id: CharacterID,
    ) -> Result<(), ()> {
        let Some(Described { value: callback, .. }) =
            self.state.act(act_id).type_.abilities().use_on_character else { return Err(()) };

        (callback)(self, UseOnCharacterArgs { act_id, target_id });

        self.state.acts.remove_from_some_player(act_id);
        Ok(())
    }

    #[@chrs]
    #[pre(true)]
    pub fn stat(
        &mut self,
        chr_id: CharacterID,
        stat_type: Stat,
    ) -> Stat0 {
        self.state.chr(chr_id).stats.stat(stat_type).into_value()
    }

    #[@chrs]
    #[pre(true)]
    pub fn modify_stat(
        &mut self,
        chr_id: CharacterID,
        stat_type: Stat,
        val: Stat0,
    ) {
        let phy = self.state.chr_mut(chr_id).stats.phy.0.into_value();
        let vit = &mut self.state.chr_mut(chr_id).stats.vit;
        let new_vit = (vit.0.into_value() + val).max(0).min(phy);
        vit.0 = StatValue::Var(new_vit);
    }

    #[@chrs]
    #[pre(true)]
    pub fn attack(
        &mut self,
        attacker_id: CharacterID,
        target_id: CharacterID,
    ) -> Result<(), ()> {
        let dmg = self.state.chr(attacker_id).stats.dmg.0.into_value();
        self.hurt(target_id, dmg)
    }

    #[@chrs]
    #[pre(true)]
    pub fn hurt(
        &mut self,
        chr_id: CharacterID,
        dmg: Stat0,
    ) -> Result<(), ()> {
        let old_def = self.state.chr(chr_id).stats.def.0.into_value();
        self.modify_stat(chr_id, Stat::Defence, dmg);
        let new_def = self.state.chr(chr_id).stats.def.0.into_value();

        let def_dmg_taken = old_def - new_def;
        let vit_dmg_to_take = dmg - def_dmg_taken;

        if vit_dmg_to_take > 0 {
            self.modify_stat(chr_id, Stat::Vitality, vit_dmg_to_take);
        }
        Ok(())
    }

    #[@chrs]
    #[pre(true)]
    pub fn place(
        &mut self,
        chr_id: CharacterID,
    ) -> Result<(), ()> {
        let Some(player_id) = self.state.chrs.try_find_owner(chr_id) else { return Err(()) };

        if player_id == self.state.attacker.player_id {
            let attacker_chr_id = &mut self.state.attacker.chr_id;

            if attacker_chr_id.is_some() {
                return Err(());
            }

            *attacker_chr_id = Some(chr_id);
            Ok(())
        } else if player_id == self.state.defender.player_id {
            let defender_chr_id = &mut self.state.defender.chr_id;

            if defender_chr_id.is_some() {
                return Err(());
            }

            *defender_chr_id = Some(chr_id);
            Ok(())
        } else {
            return Err(());
        }
    }

    #[@chrs]
    #[pre(true)]
    pub fn kill(
        &mut self,
        chr_id: CharacterID,
    ) -> Result<(), ()> {
        self.force_kill(chr_id);
        Ok(())
    }

    #[pre(true)]
    pub fn random(
        &mut self,
        min: Stat0,
        max: Stat0,
    ) -> Stat0 {
        thread_rng().gen_range(min..=max)
    }

    pub fn end_subturn(&mut self) {
        self.state.end_subturn()
    }

    pub fn end_turn(&mut self) {
        self.force_remove_from_field(self.state.current_subturner);
        self.force_remove_from_field(self.state.current_subturner.other());

        self.state.change_turner();
    }

    pub fn force_kill(&mut self, chr_id: CharacterID) {
        self.state.chr_mut(chr_id).stats.max_vit();
        self.state.chrs.add_to_wastepile(chr_id);
    }

    pub fn force_remove_from_field(&mut self, subturner: Subturner) {
        let subturner_on_field = self.state.subturner_on_field_mut(subturner);
        let chr_id = subturner_on_field.chr_id.take().expect("expected chr to be on field");
        let used_act_ids = take(&mut subturner_on_field.used_act_ids);

        if self.state.is_dead(chr_id) {
            self.force_kill(chr_id);
            return;
        }

        self.state.chr_mut(chr_id).stats.max_vit();

        let owner_id = self.state.chrs.find_owner(chr_id);
        self.state.chrs.add_to_player(chr_id, owner_id);

        for act_id in used_act_ids {
            self.state.acts.add_to_wastepile(act_id);
        }
    }

    pub fn force_set_stat(&mut self, chr_id: CharacterID, stat_type: Stat, value: Stat0) {
        self.state.chr_mut(chr_id).stats.stat_mut(stat_type).set(value)
    }

    pub fn force_set_phy_vit(&mut self, chr_id: CharacterID, value: Stat0) {
        self.force_set_stat(chr_id, Stat::Physique, value);
        self.force_set_stat(chr_id, Stat::Vitality, value);
    }
}

impl Host {
    pub fn choose_hand_act(&mut self, player_id: PlayerID) -> ActiveID {
        // TODO: просить игрока выбрать
        self.state.acts.hand(player_id)[0]
    }

    pub fn choose_hand_chr(&mut self, player_id: PlayerID) -> CharacterID {
        // TODO: просить игрока выбрать
        self.state.chrs.hand(player_id)[0]
    }
}
