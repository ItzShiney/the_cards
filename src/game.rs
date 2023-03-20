mod _macro;

use crate::card_uses::ActiveID;
use crate::card_uses::CharacterID;
use crate::card_uses::GameState;
use crate::card_uses::Stat0;
use crate::card_uses::StatType;
use crate::card_uses::Subturner;
use crate::game_chaining_methods;
use crate::game_input::ChooseCardArgs;
use crate::game_input::ChooseCardArgsP;
use crate::game_input::GameInput;
use std::mem::take;

pub struct Game<'state, 'input> {
    pub state: &'state mut GameState,
    pub input: &'input mut dyn GameInput,
}

game_chaining_methods! {
    #[chain(act_id)]
    try use_on_field(
        &mut self,
        act_id: ActiveID,
    ) {
        can {
            self.state.act(act_id).type_.abilities().force_use_on_field.is_some()
        }

        force {
            self.state.acts.remove_from_some_player(act_id);
        }
    }

    #[chain(act_id)]
    try use_on_chr(
        &mut self,
        act_id: ActiveID,
        target_id: CharacterID,
    ) {
        can {
            self.state.act(act_id).type_.abilities().force_use_on_chr.is_some()
        }

        force {
            self.state.acts.remove_from_some_player(act_id);
        }
    }

    #[chain(chr_id)]
    fn stat_map(
        &mut self,
        chr_id: CharacterID,
        stat_type: StatType,
        val: Stat0
    ) -> Stat0 {
        val
    }

    #[chain(chr_id)]
    fn stat_add(
        &mut self,
        chr_id: CharacterID,
        stat_type: StatType,
        val: Stat0,
    ) {
        let mut res = self.state.chr(chr_id).stats.stat(stat_type).into_value();
        res += val;
        res = res.max(0);

        if stat_type == StatType::Vitality {
            let phy = self.state.chr(chr_id).stats.phy.0.into_value();
            res = res.min(phy);
        }

        self.state.chr_mut(chr_id).stats.stat_mut(stat_type).set(res);
    }

    #[chain(attacker_id)]
    try attack(
        &mut self,
        attacker_id: CharacterID,
        target_id: CharacterID,
        dmg: Stat0,
    ) {
        can {
            self.can_get_hurt(target_id, dmg)
        }

        force {
            _ = self.try_get_hurt(target_id, dmg);
        }
    }

    #[chain(chr_id)]
    try get_hurt(
        &mut self,
        chr_id: CharacterID,
        dmg: Stat0,
    ) {
        can {
            dmg != 0
        }

        force {
            let old_def = self.state.chr(chr_id).stats.def.0.into_value();
            self.stat_add(chr_id, StatType::Defence, dmg);
            let new_def = self.state.chr(chr_id).stats.def.0.into_value();

            let def_dmg_taken = old_def - new_def;
            let vit_dmg_to_take = dmg - def_dmg_taken;

            if vit_dmg_to_take > 0 {
                self.stat_add(chr_id, StatType::Vitality, vit_dmg_to_take);
            }
        }
    }

    #[chain(chr_id)]
    try place(
        &mut self,
        chr_id: CharacterID,
    ) {
        can {
            let Some(owner_id) = self.state.try_find_owner_of_chr(chr_id) else {
                return true;
            };

            (owner_id == self.state.attacker.player_id && self.state.attacker.chr_id.is_none())
                || (owner_id == self.state.defender.player_id && self.state.defender.chr_id.is_none())
        }

        force {
            let Some(player_id) = self.state.try_find_owner_of_chr(chr_id) else { return };

            if player_id == self.state.attacker.player_id {
                let attacker_chr_id = &mut self.state.attacker.chr_id;

                if attacker_chr_id.is_some() {
                    return;
                }

                self.state.chrs.remove_from_player(chr_id, player_id);
                *attacker_chr_id = Some(chr_id);
            } else if player_id == self.state.defender.player_id {
                let defender_chr_id = &mut self.state.defender.chr_id;

                if defender_chr_id.is_some() {
                    return;
                }

                self.state.chrs.remove_from_player(chr_id, player_id);
                *defender_chr_id = Some(chr_id);
            }
        }
    }

    #[chain(chr_id)]
    try die(
        &mut self,
        chr_id: CharacterID,
    ) {
        can {
            true
        }

        force {
            self.state.chr_mut(chr_id).stats.max_vit();
            self.state.chrs.add_to_wastepile(chr_id);

            self.force_end_turn();
        }
    }

    #[chain(chr_id)]
    fn is_dead(
        &mut self,
        chr_id: CharacterID,
    ) -> bool {
        self.state.chr(chr_id).stats.vit.into_value() == 0
    }

    fn random(
        &mut self,
        min: Stat0,
        max: Stat0,
    ) -> Stat0 {
        self.input.random(min, max)
    }

    fn random_bool(
        &mut self,
        true_prob: f64,
    ) -> bool {
        self.input.random_bool(true_prob)
    }

    try end_turn(
        &mut self,
    ) {
        can {
            true
        }

        force {
            self.force_remove_from_field(self.state.current_subturner);
            self.force_remove_from_field(self.state.current_subturner.other());

            self.state.change_turner();
        }
    }

    fn force_remove_from_field(
        &mut self,
        subturner: Subturner
    ) {
        let subturner_on_field = self.state.subturner_on_field_mut(subturner);
        let chr_id = subturner_on_field.chr_id.take().expect("expected chr to be on field");
        let used_act_ids = take(&mut subturner_on_field.used_act_ids);

        if self.is_dead(chr_id) {
            self.force_die(chr_id);
            return;
        }

        self.state.chr_mut(chr_id).stats.max_vit();

        let owner_id = self.state.find_owner_of_chr(chr_id);
        self.state.chrs.add_to_player(chr_id, owner_id);

        for act_id in used_act_ids {
            self.state.acts.add_to_wastepile(act_id);
        }
    }

    fn force_set_stat(
        &mut self,
        chr_id: CharacterID,
        stat_type: StatType,
        value: Stat0
    ) {
        self.state.chr_mut(chr_id).stats.stat_mut(stat_type).set(value)
    }

    fn force_set_phy_vit(
        &mut self,
        chr_id: CharacterID,
        value: Stat0
    ) {
        self.force_set_stat(chr_id, StatType::Physique, value);
        self.force_set_stat(chr_id, StatType::Vitality, value);
    }

    fn replace(
        &mut self,
        replaced_chr_id: CharacterID,
        replacing_chr_id: CharacterID
    ) {
        todo!()
    }
}

impl Game<'_, '_> {
    pub fn can_use_in_any_way(&mut self, act_id: ActiveID) -> bool {
        self.can_use_on_own_chr(act_id)
            || self.can_use_on_enemy_chr(act_id)
            || self.can_use_on_field(act_id)
    }

    pub fn can_use_on_own_chr(&self, act_id: ActiveID) -> bool {
        let Some(chr_id) = self.state.current_subturner_on_field().chr_id else {
            return false;
        };

        self.can_use_on_chr(act_id, chr_id)
    }

    pub fn can_use_on_enemy_chr(&self, act_id: ActiveID) -> bool {
        let Some(chr_id) = self.state.other_subturner_on_field().chr_id else {
            return false;
        };

        self.can_use_on_chr(act_id, chr_id)
    }

    pub fn stat(&self, chr_id: CharacterID, stat_type: StatType) -> Stat0 {
        let val = self.state.chr(chr_id).stats.stat(stat_type).into_value();
        self.stat_map(chr_id, stat_type, val)
    }

    /* pub fn attack(
        &mut self,
        attacker_id: CharacterID,
        target_id: CharacterID,
    ) {
        let dmg = self.chr(attacker_id).stats.dmg.0.into_value();
        self.attack_map(attacker_id, target_id, dmg)
    } */

    pub fn choose_chr_in_hand(
        &mut self,
        args: ChooseCardArgsP<'_, CharacterID>,
    ) -> Option<CharacterID> {
        self.input.choose_chr_in_hand(&mut self.state, args)
    }

    pub fn choose_act_in_hand(&mut self, args: ChooseCardArgsP<'_, ActiveID>) -> Option<ActiveID> {
        self.input.choose_act_in_hand(self.state, args)
    }

    pub fn choose_chr_in_hand_any(&mut self, args: ChooseCardArgs) -> Option<CharacterID> {
        let p = |_, _| true;
        self.input.choose_chr_in_hand(self.state, ChooseCardArgsP::new(args, &p))
    }

    pub fn choose_act_in_hand_any(&mut self, args: ChooseCardArgs) -> Option<ActiveID> {
        let p = |_, _| true;
        self.input.choose_act_in_hand(self.state, ChooseCardArgsP::new(args, &p))
    }

    pub fn choose_chr_on_field(
        &mut self,
        args: ChooseCardArgsP<'_, CharacterID>,
    ) -> Option<CharacterID> {
        self.input.choose_chr_on_field(self.state, args)
    }

    pub fn choose_act_on_field(&mut self, args: ChooseCardArgsP<'_, ActiveID>) -> Option<ActiveID> {
        self.input.choose_act_on_field(self.state, args)
    }

    pub fn choose_chr_on_field_any(&mut self, args: ChooseCardArgs) -> Option<CharacterID> {
        let p = |_, _| true;
        self.input.choose_chr_on_field(self.state, ChooseCardArgsP::new(args, &p))
    }

    pub fn choose_act_on_field_any(&mut self, args: ChooseCardArgs) -> Option<ActiveID> {
        let p = |_, _| true;
        self.input.choose_act_on_field(self.state, ChooseCardArgsP::new(args, &p))
    }

    pub fn end_subturn(&mut self) {
        self.state.current_subturner.switch()
    }
}
