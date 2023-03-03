use rand::{thread_rng, Rng};

use crate::described::Described;
use crate::game_state::act_id::ActiveID;
use crate::game_state::chr_id::CharacterID;
use crate::game_state::player_id::PlayerID;
use crate::game_state::GameState;
use crate::stats::{Stat, Stat0, StatValue};

pub struct Host {
    pub callbacks: GameCallbacks,
    state: GameState,
}

impl Host {
    pub fn new(state: GameState) -> Self {
        Self { callbacks: Default::default(), state }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    // TODO: remove
    pub fn state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }
}

pub enum Chain<Continue, Result = ()> {
    Continue(Continue),
    Break(Result),
}

macro_rules! callbacks {
    (
        $(
            pub fn $name:ident(
                &mut $self:ident $(@ $self_namespace:ident)?
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
                pub fn [<$name:camel:snake _args>] (&mut $self, mut args: [<$name:camel Args>] ) $(-> $Return)? {
                    while let Some($crate::described::Described { value: callback, .. }) = $self.callbacks.$name {
                        match (callback)($self, args) {
                            Chain::Continue(new_args) => {
                                args = new_args;
                            }

                            Chain::Break(result) => return result,
                        }
                    }

                    #[allow(unused)] let id = ($(args.$arg_name,)* 0,).0;
                    $(
                        if let Some($crate::described::Described { value: callback, .. }) = $self.state().$self_namespace.get(id).type_.abilities().$name {
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
                        if let Some($crate::described::Described { value: callback, .. }) = $self.state().$self_namespace.get(id).type_.abilities().[<post_ $name>] {
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
    pub fn waste_act(
        &mut self @ acts,
        act_id: ActiveID,
    ) {
        self.state.acts.add_to_waste_pile(act_id);
    }

    pub fn use_on_field(
        &mut self @ acts,
        act_id: ActiveID,
    ) {
        todo!()
    }

    pub fn use_on_character(
        &mut self @ acts,
        act_id: ActiveID,
        target_id: CharacterID,
    ) -> Result<(), ()> {
        let Some(Described { value: callback, .. }) =
            self.state().act(act_id).type_.abilities().use_on_character else { return Err(()) };

        (callback)(self, UseOnCharacterArgs { act_id, target_id });

        self.state.acts.remove_from_some_player(act_id);
        Ok(())
    }

    pub fn modify_stat(
        &mut self @ chrs,
        chr_id: CharacterID,
        stat_type: Stat,
        val: Stat0,
    ) {
        let phy = self.state.chr_mut(chr_id).stats.phy.0.into_value().unwrap();
        let vit = &mut self.state.chr_mut(chr_id).stats.vit;
        let new_vit = (vit.0.into_value().unwrap() + val).max(0).min(phy);
        vit.0 = StatValue::Var(new_vit);
    }

    pub fn hurt(
        &mut self @ chrs,
        chr_id: CharacterID,
        dmg: Stat0,
    ) {
        let old_def = self.state().chr(chr_id).stats.def.0.into_value().unwrap();
        self.modify_stat(chr_id, Stat::Defence, dmg);
        let new_def = self.state().chr(chr_id).stats.def.0.into_value().unwrap();

        let def_dmg_taken = old_def - new_def;
        let vit_dmg_to_take = dmg - def_dmg_taken;

        if vit_dmg_to_take > 0 {
            self.modify_stat(chr_id, Stat::Vitality, vit_dmg_to_take);
        }
    }

    pub fn place(
        &mut self @ chrs,
        chr_id: CharacterID,
    ) -> Result<(), ()> {
        let Some(player_id) = self.state.chrs.find_owner(chr_id) else { return Err(()) };

        if player_id == self.state.attacker().player_id {
            let attacker_chr_id = &mut self.state.attacker_mut().chr_id;

            if attacker_chr_id.is_some() {
                return Err(());
            }

            *attacker_chr_id = Some(chr_id);
            Ok(())
        } else if player_id == self.state.defender().player_id {
            let defender_chr_id = &mut self.state.defender_mut().chr_id;

            if defender_chr_id.is_some() {
                return Err(());
            }

            *defender_chr_id = Some(chr_id);
            Ok(())
        } else {
            return Err(());
        }
    }

    pub fn kill(
        &mut self @ chrs,
        chr_id: CharacterID,
    ) {
        todo!()
    }

    pub fn random(
        &mut self,
        min: Stat0,
        max: Stat0,
    ) -> Stat0 {
        thread_rng().gen_range(min..=max)
    }
}

impl Host {
    pub fn set_stat(&mut self, chr_id: CharacterID, stat_type: Stat, value: Stat0) {
        // FIXME
        match self.state.chr_mut(chr_id).stats.stat_mut(stat_type) {
            StatValue::Var(stat) => *stat = value,
            StatValue::Const(_) => panic!("set const"),
            stat @ StatValue::Unknown => *stat = StatValue::Var(value),
        }
    }

    pub fn set_phy_vit(&mut self, chr_id: CharacterID, value: Stat0) {
        self.set_stat(chr_id, Stat::Physique, value);
        self.set_stat(chr_id, Stat::Vitality, value);
    }
}

impl Host {
    pub fn choose_hand_act(&mut self, player_id: PlayerID) -> ActiveID {
        // TODO: просить игрока выбрать
        self.state().acts.hand(player_id)[0]
    }

    pub fn choose_hand_chr(&mut self, player_id: PlayerID) -> CharacterID {
        // TODO: просить игрока выбрать
        self.state().chrs.hand(player_id)[0]
    }

    pub fn end_subturn(&mut self) {
        self.state.end_subturn()
    }

    pub fn end_turn(&mut self) {
        self.state.end_turn()
    }
}
