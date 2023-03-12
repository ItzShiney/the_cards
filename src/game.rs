pub mod _macro;
pub mod chain;
pub mod input;
pub mod state;

use self::input::ChooseCardArgs;
use self::input::ChooseCardArgsP;
use crate::acts::ActiveType;
use crate::callbacks;
use crate::chrs::CharacterType;
use crate::game::input::GameInput;
use crate::game::state::act_id::ActiveID;
use crate::game::state::act_info::ActiveInfo;
use crate::game::state::chr_id::CharacterID;
use crate::game::state::chr_info::CharacterInfo;
use crate::game::state::GameState;
use crate::game::state::Subturner;
use crate::group::Group;
use crate::stats::Stat0;
use crate::stats::StatType;
use crate::stats::StatValue;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::mem::take;
use std::result::Result;

pub struct Game {
    pub state: GameState,
    pub input: Box<dyn GameInput>,
}

impl Game {
    pub fn new(state: GameState, input: Box<dyn GameInput>) -> Self {
        let mut res = Self { state, input };
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

    // TODO сделать конструктор принимающим GameInitInfo или GameConfig со всеми этими параметрами
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
            let chr_type = self.random_chr_type();
            let chr = CharacterInfo::new(chr_type);

            let chr_id = self.state.add_chr(chr);
            self.state.chrs.add_to_drawpile(chr_id);
        }

        for _ in 1..=total_acts_count {
            let act_type = self.random_act_type();
            let act = ActiveInfo::new(act_type);

            let act_id = self.state.add_act(act);
            self.state.acts.add_to_drawpile(act_id);
        }
    }
}

#[derive(Debug)]
pub struct Finished;

#[derive(Debug)]
pub struct Terminated;

pub type ChainResult<Ok = Finished> = Result<Ok, Terminated>;

// TODO переместить в state.rs, как-то переделать инпут?
callbacks! {
    #[ping(acts)]
    #[pre(true)]
    pub fn use_on_field(
        &mut self,
        act_id: ActiveID,
    ) -> ChainResult {
        todo!()
    }

    #[ping(acts)]
    #[pre(true)]
    pub fn use_on_chr(
        &mut self,
        act_id: ActiveID,
        target_id: CharacterID,
    ) -> ChainResult {
        let Some(callback) =
            self.state.act(act_id).type_.abilities().use_on_chr else { return Err(Terminated) };

        (callback)(self, UseOnChrArgs { act_id, target_id });

        self.state.acts.remove_from_some_player(act_id);
        Ok(Finished)
    }

    #[ping(chrs)]
    #[pre(true)]
    pub fn stat_map(
        &mut self,
        chr_id: CharacterID,
        stat_type: StatType,
        val: Stat0
    ) -> Stat0 {
        val
    }

    #[ping(chrs)]
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

    #[ping(chrs)]
    #[pre(true)]
    pub fn attack_map(
        &mut self,
        attacker_id: CharacterID,
        target_id: CharacterID,
        dmg: Stat0,
    ) -> ChainResult {
        self.hurt(target_id, dmg)
    }

    #[ping(chrs)]
    #[pre(true)]
    pub fn hurt(
        &mut self,
        target_id: CharacterID,
        dmg: Stat0,
    ) -> ChainResult {
        let old_def = self.state.chr(target_id).stats.def.0.into_value();
        self.stat_add(target_id, StatType::Defence, dmg);
        let new_def = self.state.chr(target_id).stats.def.0.into_value();

        let def_dmg_taken = old_def - new_def;
        let vit_dmg_to_take = dmg - def_dmg_taken;

        if vit_dmg_to_take > 0 {
            self.stat_add(target_id, StatType::Vitality, vit_dmg_to_take);
        }
        Ok(Finished)
    }

    #[ping(chrs)]
    #[pre(true)]
    pub fn place(
        &mut self,
        chr_id: CharacterID,
    ) -> ChainResult {
        let Some(player_id) = self.state.try_find_owner_chr(chr_id) else { return Err(Terminated) };

        if player_id == self.state.attacker.player_id {
            let attacker_chr_id = &mut self.state.attacker.chr_id;

            if attacker_chr_id.is_some() {
                return Err(Terminated)
            }

            self.state.chrs.remove_from_player(chr_id, player_id);
            *attacker_chr_id = Some(chr_id);
            Ok(Finished)
        } else if player_id == self.state.defender.player_id {
            let defender_chr_id = &mut self.state.defender.chr_id;

            if defender_chr_id.is_some() {
                return Err(Terminated)
            }

            self.state.chrs.remove_from_player(chr_id, player_id);
            *defender_chr_id = Some(chr_id);
            Ok(Finished)
        } else {
            Err(Terminated)
        }
    }

    #[ping(chrs)]
    #[pre(true)]
    pub fn die(
        &mut self,
        chr_id: CharacterID,
    ) -> ChainResult {
        self.force_die(chr_id);
        Ok(Finished)
    }

    #[pre(true)]
    pub fn random(
        &mut self,
        min: Stat0,
        max: Stat0,
    ) -> Stat0 {
        self.input.random(min, max)
    }

    #[pre(true)]
    pub fn random_bool(
        &mut self,
        true_prob: f64,
    ) -> bool {
        self.input.random_bool(true_prob)
    }

    #[pre(true)]
    pub fn random_chr_type(
        &mut self,
    ) -> CharacterType {
        CharacterType::all()
                .into_iter()
                .filter(|&chr_type| !chr_type.groups().contains(&Group::Нераздаваемая))
                .choose(&mut thread_rng())
                .unwrap()
    }

    #[pre(true)]
    pub fn random_act_type(
        &mut self,
    ) -> ActiveType {
        ActiveType::all()
                .into_iter()
                .filter(|&chr_type| !chr_type.groups().contains(&Group::Нераздаваемая))
                .choose(&mut thread_rng())
                .unwrap()
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

        let owner_id = self.state.find_owner_chr(chr_id);
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

impl Game {
    pub fn stat(&mut self, chr_id: CharacterID, stat_type: StatType) -> Stat0 {
        let val = self.state.chr(chr_id).stats.stat(stat_type).into_value();
        self.stat_map(chr_id, stat_type, val)
    }

    pub fn attack(&mut self, attacker_id: CharacterID, target_id: CharacterID) -> ChainResult {
        let dmg = self.state.chr(attacker_id).stats.dmg.0.into_value();
        self.attack_map(attacker_id, target_id, dmg)
    }

    pub fn choose_chr_in_hand<'game_state>(
        &'game_state mut self,
        args: ChooseCardArgsP<'_, 'game_state, '_, CharacterID>,
    ) -> Option<CharacterID> {
        self.input.choose_chr_in_hand(&self.state, args)
    }

    pub fn choose_act_in_hand<'game_state>(
        &'game_state mut self,
        args: ChooseCardArgsP<'_, 'game_state, '_, ActiveID>,
    ) -> Option<ActiveID> {
        self.input.choose_act_in_hand(&self.state, args)
    }

    pub fn choose_chr_in_hand_any(&mut self, args: ChooseCardArgs) -> Option<CharacterID> {
        let p = |_, _| true;
        self.input.choose_chr_in_hand(&self.state, ChooseCardArgsP::new(args, &p))
    }

    pub fn choose_act_in_hand_any(&mut self, args: ChooseCardArgs) -> Option<ActiveID> {
        let p = |_, _| true;
        self.input.choose_act_in_hand(&self.state, ChooseCardArgsP::new(args, &p))
    }

    pub fn choose_chr_on_field<'game_state>(
        &'game_state mut self,
        args: ChooseCardArgsP<'_, 'game_state, '_, CharacterID>,
    ) -> Option<CharacterID> {
        self.input.choose_chr_on_field(&self.state, args)
    }

    pub fn choose_act_on_field<'game_state>(
        &'game_state mut self,
        args: ChooseCardArgsP<'_, 'game_state, '_, ActiveID>,
    ) -> Option<ActiveID> {
        self.input.choose_act_on_field(&self.state, args)
    }

    pub fn choose_chr_on_field_any(&mut self, args: ChooseCardArgs) -> Option<CharacterID> {
        let p = |_, _| true;
        self.input.choose_chr_on_field(&self.state, ChooseCardArgsP::new(args, &p))
    }

    pub fn choose_act_on_field_any(&mut self, args: ChooseCardArgs) -> Option<ActiveID> {
        let p = |_, _| true;
        self.input.choose_act_on_field(&self.state, ChooseCardArgsP::new(args, &p))
    }
}
