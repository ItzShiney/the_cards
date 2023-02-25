use crate::game_state::ability::active_trigger::{ActiveTrigger, WentActiveTrigger};
use crate::game_state::active_id::ActiveID;
use crate::game_state::character_id::CharacterID;
use crate::game_state::player_id::PlayerID;
use crate::game_state::GameState;
use crate::stats::{Stat0, Stats};

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

pub enum Chain<T = ()> {
    Continue,
    Break(T),
}

macro_rules! callbacks {
    (
        $(
            fn $name:ident(
                $self:ident,
                $(
                    $arg_name:ident : $ArgType:ty,
                )*
            ) $(-> $Return:ty)? $callback_action:block
        )*
    ) => {paste::paste! {
        $(
            pub struct [<$name:camel CallbackArgs>] {
                $(pub $arg_name: $ArgType,)*
            }
        )*

        $(
            pub type [<$name:camel Callback>] = Box<dyn Fn(&mut Host, &[<$name:camel CallbackArgs>]) -> Chain<$($Return)?>>;
        )*

        #[derive(Default)]
        pub struct GameCallbacks {
            $(
                [<$name:camel:snake _callbacks>]: Vec<[<$name:camel Callback>]>,
            )*
        }

        impl GameCallbacks {
            $(
                pub fn [<add_ $name:camel:snake _callback>](&mut self, callback: [<$name:camel Callback>]) {
                    self.[<$name:camel:snake _callbacks>].push(callback);
                }
            )*
        }

        impl Host {
            $(
                pub fn [<$name:camel:snake _args>] (&mut $self, args: [<$name:camel CallbackArgs>] ) $(-> $Return)? {
                    while let Some(callback) = $self.callbacks.[<$name:camel:snake _callbacks>].pop() {
                        match callback($self, &args) {
                            Chain::Continue => {}
                            Chain::Break(x) => return x,
                        }
                    }

                    #[allow(unused)]
                    let [<$name:camel CallbackArgs>] { $($arg_name,)* } = args;

                    $callback_action
                }

                pub fn $name (&mut $self, $($arg_name: $ArgType,)* ) $(-> $Return)? {
                    $self.[<$name:camel:snake _args>] (
                        [<$name:camel CallbackArgs>] {
                            $($arg_name,)*
                        }
                    )
                }
            )*
        }
    }};
}

callbacks! {
    fn waste_act(
        self,
        act_id: ActiveID,
    ) {
        self.state.acts.add_to_waste_pile(act_id);
    }

    fn use_on_character(
        self,
        act_id: ActiveID,
        chr_id: CharacterID,
    ) {
        let abilities = self.state.act(act_id).type_.abilities();
        let Some(ability) = self.state.find_matching_ability(ActiveTrigger::UsedOnCharacter, abilities) else { panic!("{act_id:?} can't be used on {chr_id:?}") };

        let went_trigger = WentActiveTrigger::UsedOnCharacter(chr_id);
        (ability.callback)(self, act_id, went_trigger);

        self.state.acts.remove_from_some_player(act_id);
    }

    fn vit_sub(
        self,
        chr_id: CharacterID,
        val: Stat0,
    ) {
        self.state.chr_mut(chr_id).stats.vit.0 -= val;
    }

    fn def_sub(
        self,
        chr_id: CharacterID,
        val: Stat0,
    ) {
        self.state.chr_mut(chr_id).stats.def.0 -= val;
    }

    fn hurt(
        self,
        chr_id: CharacterID,
        dmg: Stat0,
    ) {
        let old_def = self.state().chr(chr_id).stats.def.0.into_value().unwrap();
        self.def_sub(chr_id, dmg);
        let new_def = self.state().chr(chr_id).stats.def.0.into_value().unwrap();

        let def_dmg_taken = old_def - new_def;
        let vit_dmg_to_take = dmg - def_dmg_taken;

        if vit_dmg_to_take > 0 {
            self.vit_sub(chr_id, vit_dmg_to_take);
        }
    }

    fn init_stats(
        self,
        chr_id: CharacterID,
        stats: Stats,
    ) {
        if self.state.chr(chr_id).stats != Stats::UNINIT {
            panic!("trying to init inited stats");
        }

        self.state.chr_mut(chr_id).stats = stats;
    }

    fn place(
        self,
        chr_id: CharacterID,
    ) {
        let player_id = self.state.chrs.find_owner(chr_id).expect(format!("expected some player to own {:?}", chr_id).as_str());

        if player_id == self.state.attacker().player_id {
            let attacker_chr_id = &mut self.state.attacker_mut().chr_id;
            if attacker_chr_id.is_some() {
                panic!("attacker is already placed");
            }
            *attacker_chr_id = Some(chr_id);
        } else if player_id == self.state.defender().player_id {
            let defender_chr_id = &mut self.state.defender_mut().chr_id;
            if defender_chr_id.is_some() {
                panic!("defender is already placed");
            }
            *defender_chr_id = Some(chr_id);
        } else {
            panic!("{:?} is not in battle", player_id);
        }
    }

    fn choose_hand_act(
        self,
        player_id: PlayerID,
    ) -> ActiveID {
        todo!()
    }

    fn choose_hand_chr(
        self,
        player_id: PlayerID,
    ) -> CharacterID {
        todo!()
    }

    fn end_subturn(
        self,
    ) {
        self.state.end_subturn()
    }

    fn end_turn(
        self,
    ) {
        self.state.end_turn()
    }
}
