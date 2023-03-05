pub mod chain;
pub mod macro_;

use std::mem::take;

use rand::{seq::IteratorRandom, thread_rng, Rng};

use crate::{
    acts::ActiveType,
    callbacks,
    chrs::CharacterType,
    game_state::act_id::ActiveID,
    game_state::act_info::ActiveInfo,
    game_state::chr_id::CharacterID,
    game_state::chr_info::CharacterInfo,
    game_state::player_id::PlayerID,
    game_state::{GameState, Subturner},
    group::Group,
    stats::{Stat0, StatType, StatValue},
};

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
                .filter(|&chr_type| !chr_type.groups().contains(&Group::Нераздаваемая))
                .choose(&mut thread_rng())
                .unwrap();

            let chr = CharacterInfo::new(chr_type);

            let chr_id = self.state.add_chr(chr);
            self.state.chrs.add_to_drawpile(chr_id);
        }

        for _ in 1..=total_acts_count {
            let act_type = ActiveType::all()
                .into_iter()
                .filter(|&act_type| !act_type.groups().contains(&Group::Нераздаваемая))
                .choose(&mut thread_rng())
                .unwrap();

            let act = ActiveInfo::new(act_type);

            let act_id = self.state.add_act(act);
            self.state.acts.add_to_drawpile(act_id);
        }
    }
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
        let Some(callback) =
            self.state.act(act_id).type_.abilities().use_on_character else { return Err(()) };

        (callback)(self, UseOnCharacterArgs { act_id, target_id });

        self.state.acts.remove_from_some_player(act_id);
        Ok(())
    }

    #[@chrs]
    #[pre(true)]
    pub fn stat_map(
        &mut self,
        chr_id: CharacterID,
        stat_type: StatType,
        val: Stat0
    ) -> Stat0 {
        val
    }

    #[@chrs]
    #[pre(true)]
    pub fn stat_add(
        &mut self,
        chr_id: CharacterID,
        stat_type: StatType,
        val: Stat0,
    ) {
        let phy = self.state.chr_mut(chr_id).stats.phy.0.into_value();
        let vit = &mut self.state.chr_mut(chr_id).stats.vit;
        let new_vit = (vit.0.into_value() + val).max(0).min(phy);
        vit.0 = StatValue::Var(new_vit);
    }

    #[@chrs]
    #[pre(true)]
    pub fn attack_map(
        &mut self,
        attacker_id: CharacterID,
        target_id: CharacterID,
        dmg: Stat0,
    ) -> Result<(), ()> {
        self.hurt(target_id, dmg)
    }

    #[@chrs]
    #[pre(true)]
    pub fn hurt(
        &mut self,
        target_id: CharacterID,
        dmg: Stat0,
    ) -> Result<(), ()> {
        let old_def = self.state.chr(target_id).stats.def.0.into_value();
        self.stat_add(target_id, StatType::Defence, dmg);
        let new_def = self.state.chr(target_id).stats.def.0.into_value();

        let def_dmg_taken = old_def - new_def;
        let vit_dmg_to_take = dmg - def_dmg_taken;

        if vit_dmg_to_take > 0 {
            self.stat_add(target_id, StatType::Vitality, vit_dmg_to_take);
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
    pub fn die(
        &mut self,
        chr_id: CharacterID,
    ) -> Result<(), ()> {
        self.force_die(chr_id);
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

    #[pre(true)]
    pub fn random_bool(
        &mut self,
        true_probability: f64,
    ) -> bool {
        thread_rng().gen_bool(true_probability)
    }

    pub fn end_subturn(&mut self) {
        self.state.current_subturner.switch()
    }

    pub fn end_turn(&mut self) {
        self.force_remove_from_field(self.state.current_subturner);
        self.force_remove_from_field(self.state.current_subturner.other());

        self.state.change_turner();
    }

    pub fn force_die(&mut self, chr_id: CharacterID) {
        self.state.chr_mut(chr_id).stats.max_vit();
        self.state.chrs.add_to_wastepile(chr_id);
    }

    pub fn force_remove_from_field(&mut self, subturner: Subturner) {
        let subturner_on_field = self.state.subturner_on_field_mut(subturner);
        let chr_id = subturner_on_field.chr_id.take().expect("expected chr to be on field");
        let used_act_ids = take(&mut subturner_on_field.used_act_ids);

        if self.state.is_dead(chr_id) {
            self.force_die(chr_id);
            return;
        }

        self.state.chr_mut(chr_id).stats.max_vit();

        let owner_id = self.state.chrs.find_owner(chr_id);
        self.state.chrs.add_to_player(chr_id, owner_id);

        for act_id in used_act_ids {
            self.state.acts.add_to_wastepile(act_id);
        }
    }

    pub fn force_set_stat(&mut self, chr_id: CharacterID, stat_type: StatType, value: Stat0) {
        self.state.chr_mut(chr_id).stats.stat_mut(stat_type).set(value)
    }

    pub fn force_set_phy_vit(&mut self, chr_id: CharacterID, value: Stat0) {
        self.force_set_stat(chr_id, StatType::Physique, value);
        self.force_set_stat(chr_id, StatType::Vitality, value);
    }

    pub fn replace(&mut self, replaced_chr_id: CharacterID, replacing_chr_id: CharacterID) {
        todo!()
    }
}

impl Host {
    pub fn stat(&mut self, chr_id: CharacterID, stat_type: StatType) -> Stat0 {
        let val = self.state.chr(chr_id).stats.stat(stat_type).into_value();
        self.stat_map(chr_id, stat_type, val)
    }

    pub fn attack(&mut self, attacker_id: CharacterID, target_id: CharacterID) -> Result<(), ()> {
        let dmg = self.state.chr(attacker_id).stats.dmg.0.into_value();
        self.attack_map(attacker_id, target_id, dmg)
    }

    pub fn choose_hand_act(&mut self, player_id: PlayerID) -> ActiveID {
        // TODO: просить игрока выбрать
        self.state.acts.hand(player_id)[0]
    }

    pub fn choose_hand_chr(&mut self, player_id: PlayerID) -> CharacterID {
        // TODO: просить игрока выбрать
        self.state.chrs.hand(player_id)[0]
    }
}
