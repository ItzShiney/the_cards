use crate::game_input::ChooseCardArgs;
use crate::game_input::ChooseCardArgsP;
use crate::game_input::GameInput;
use crate::game_state::act_id::ActiveID;
use crate::game_state::chr_id::CharacterID;
use crate::game_state::GameState;
use crate::game_state::Subturner;
use crate::stats::Stat0;
use crate::stats::StatType;
use derive_more::Constructor;
use macros::GameCallbacks;
use std::mem::take;

pub struct Game<'state, 'input> {
    pub state: &'state mut GameState,
    pub input: &'input mut dyn GameInput,
}

// TODO: разнести по файлам

#[derive(Debug, Clone, Copy)]
pub struct CannotUse;

pub trait CanForce {
    type Output;

    fn can(&self, game: &mut Game) -> bool;
    fn force(&self, game: &mut Game) -> Self::Output;

    fn try_(&self, game: &mut Game) -> Result<Self::Output, CannotUse> {
        if self.can(game) {
            Ok(self.force(game))
        } else {
            Err(CannotUse)
        }
    }
}

pub trait Map {
    type Value;

    fn map(&self, game: &mut Game, value: Self::Value) -> Self::Value;
}

#[derive(Constructor, Clone, Copy)]
pub struct UseOnField {
    pub act_id: ActiveID,
}

#[derive(Constructor, Clone, Copy)]
pub struct UseOnCharacter {
    pub act_id: ActiveID,
    pub target_id: CharacterID,
}

// TODO:
// добавить поле `caller: Caller`
// верифицировать изменения приватных статов проверкой `caller == Caller::Character(chr_id)`
#[derive(Constructor, Clone, Copy)]
pub struct StatAdd {
    pub chr_id: CharacterID,
    pub stat_type: StatType,
    pub val: Stat0,
}

#[derive(Constructor, Clone, Copy)]
pub struct Attack {
    pub attacker_id: CharacterID,
    pub target_id: CharacterID,
    pub dmg: Stat0,
}

#[derive(Constructor, Clone, Copy)]
pub struct GetHurt {
    pub chr_id: CharacterID,
    pub dmg: Stat0,
}

#[derive(Constructor, Clone, Copy)]
pub struct Place {
    pub chr_id: CharacterID,
}

#[derive(Constructor, Clone, Copy)]
pub struct Die {
    pub chr_id: CharacterID,
}

#[derive(Constructor, Clone, Copy)]
pub struct EndTurn;

#[derive(Constructor, Clone, Copy)]
pub struct Replace {
    pub replaced_chr_id: CharacterID,
    pub replacing_chr_id: CharacterID,
}

#[derive(Constructor, Clone, Copy)]
pub struct HealOnFieldLeaveMap {
    pub chr_id: CharacterID,
}

#[derive(Constructor, Clone, Copy)]
pub struct StatMap {
    pub chr_id: CharacterID,
    pub stat_type: StatType,
}

// TODO: перепроверить все реализации. вероятны лишние `.unwrap()`
GameCallbacks! {
    impl CanForce for UseOnField {
        type Output = ();

        fn can(&self, game: &mut Game) -> bool {
            game.state.act(self.act_id).type_.abilities().force_use_on_field.is_some()
        }

        fn force(&self, game: &mut Game) -> Self::Output {
            game.state.acts.remove_from_some_player(self.act_id);
        }
    }

    impl CanForce for UseOnCharacter {
        type Output = ();

        fn can(&self, game: &mut Game) -> bool {
            game.state.act(self.act_id).type_.abilities().force_use_on_chr.is_some()
        }

        fn force(&self, game: &mut Game) -> Self::Output {
            game.state.acts.remove_from_some_player(self.act_id);
        }
    }

    impl CanForce for StatAdd {
        type Output = ();

        fn can(&self, game: &mut Game) -> bool {
            !(game.is_const(self.chr_id, self.stat_type)
                || game.is_private(self.chr_id, self.stat_type))
        }

        fn force(&self, game: &mut Game) -> Self::Output {
            let mut res = game.state.chr(self.chr_id).stats.stat(self.stat_type);
            res += self.val;
            res = res.max(0);

            if self.stat_type == StatType::Vitality {
                let phy = game.state.chr(self.chr_id).stats.phy.0;
                res = res.min(phy);
            }

            *game.state.chr_mut(self.chr_id).stats.stat_mut(self.stat_type) = res;
        }
    }

    impl CanForce for Attack {
        type Output = ();

        fn can(&self, game: &mut Game) -> bool {
            GetHurt::new(self.target_id, self.dmg).can(game)
        }

        fn force(&self, game: &mut Game) -> Self::Output {
            GetHurt::new(self.target_id, self.dmg).try_(game).unwrap();
        }
    }

    impl CanForce for GetHurt {
        type Output = ();

        fn can(&self, _game: &mut Game) -> bool {
            self.dmg != 0
        }

        fn force(&self, game: &mut Game) -> Self::Output {
            let old_def = game.state.chr(self.chr_id).stats.def.0;
            _ = StatAdd::new(self.chr_id, StatType::Defence, self.dmg).try_(game);
            let new_def = game.state.chr(self.chr_id).stats.def.0;

            let def_dmg_taken = old_def - new_def;
            let vit_dmg_to_take = self.dmg - def_dmg_taken;

            if vit_dmg_to_take > 0 {
                _ = StatAdd::new(self.chr_id, StatType::Vitality, vit_dmg_to_take).try_(game);
            }
        }
    }

    impl CanForce for Place {
        type Output = ();

        fn can(&self, game: &mut Game) -> bool {
            if let Some(owner_id) = game.state.try_find_owner_of_chr(self.chr_id) {
                (owner_id == game.state.attacker.player_id && game.state.attacker.chr_id.is_none())
                    || (owner_id == game.state.defender.player_id
                        && game.state.defender.chr_id.is_none())
            } else {
                true
            }
        }

        fn force(&self, game: &mut Game) -> Self::Output {
            if let Some(player_id) = game.state.try_find_owner_of_chr(self.chr_id) {
                if player_id == game.state.attacker.player_id {
                    let attacker_chr_id = &mut game.state.attacker.chr_id;

                    if attacker_chr_id.is_some() {
                        return;
                    }

                    game.state.chrs.remove_from_player(self.chr_id, player_id);
                    *attacker_chr_id = Some(self.chr_id);
                } else if player_id == game.state.defender.player_id {
                    let defender_chr_id = &mut game.state.defender.chr_id;

                    if defender_chr_id.is_some() {
                        return;
                    }

                    game.state.chrs.remove_from_player(self.chr_id, player_id);
                    *defender_chr_id = Some(self.chr_id);
                }
            }
        }
    }

    impl CanForce for Die {
        type Output = ();

        fn can(&self, _game: &mut Game) -> bool {
            true
        }

        fn force(&self, game: &mut Game) -> Self::Output {
            game.heal_on_field_leave(self.chr_id);
            game.state.chrs.add_to_wastepile(self.chr_id);

            EndTurn::new().force(game);
        }
    }

    impl CanForce for EndTurn {
        type Output = ();

        fn can(&self, _game: &mut Game) -> bool {
            true
        }

        fn force(&self, game: &mut Game) -> Self::Output {
            game.force_remove_from_field(game.state.current_subturner);
            game.force_remove_from_field(game.state.current_subturner.other());
            game.state.change_turner();
        }
    }

    impl CanForce for Replace {
        type Output = ();

        fn can(&self, _game: &mut Game) -> bool {
            true
        }

        fn force(&self, _game: &mut Game) -> Self::Output {
            todo!()
        }
    }

    impl Map for HealOnFieldLeaveMap {
        type Value = Stat0;

        fn map(&self, _game: &mut Game, value: Self::Value) -> Self::Value {
            value
        }
    }

    impl Map for StatMap {
        type Value = Stat0;

        fn map(&self, _game: &mut Game, value: Self::Value) -> Self::Value {
            value
        }
    }
}

impl Game<'_, '_> {
    pub fn is_dead(&mut self, chr_id: CharacterID) -> bool {
        self.state.chr(chr_id).stats.vit.0 == 0
    }

    pub fn random(&mut self, min: Stat0, max: Stat0) -> Stat0 {
        self.input.random(min, max)
    }

    pub fn random_bool(&mut self, true_prob: f64) -> bool {
        self.input.random_bool(true_prob)
    }

    pub fn force_remove_from_field(&mut self, subturner: Subturner) {
        let subturner_on_field = self.state.subturner_on_field_mut(subturner);
        let chr_id = subturner_on_field.chr_id.take().expect("expected chr to be on field");
        let used_act_ids = take(&mut subturner_on_field.used_act_ids);

        if self.is_dead(chr_id) {
            Die::new(chr_id).force(self);
            return;
        }

        self.heal_on_field_leave(chr_id);

        let owner_id = self.state.find_owner_of_chr(chr_id);
        self.state.chrs.add_to_player(chr_id, owner_id);

        for act_id in used_act_ids {
            self.state.acts.add_to_wastepile(act_id);
        }
    }

    pub fn force_set_stat(&mut self, chr_id: CharacterID, stat_type: StatType, value: Stat0) {
        *self.state.chr_mut(chr_id).stats.stat_mut(stat_type) = value;
    }

    pub fn force_set_phy_vit(&mut self, chr_id: CharacterID, value: Stat0) {
        self.force_set_stat(chr_id, StatType::Physique, value);
        self.force_set_stat(chr_id, StatType::Vitality, value);
    }

    pub fn is_const(&self, _chr_id: CharacterID, _stat_type: StatType) -> bool {
        false
    }

    pub fn is_private(&self, _chr_id: CharacterID, _stat_type: StatType) -> bool {
        false
    }

    pub fn will_change(&self, _chr_id: CharacterID, _stat_type: StatType) -> bool {
        false
    }
}

impl Game<'_, '_> {
    pub fn can_use_in_any_way(&mut self, act_id: ActiveID) -> bool {
        self.can_use_on_own_chr(act_id)
            || self.can_use_on_enemy_chr(act_id)
            || UseOnField::new(act_id).can(self)
    }

    pub fn can_use_on_own_chr(&mut self, act_id: ActiveID) -> bool {
        let Some(chr_id) = self.state.current_subturner_on_field().chr_id else {
            return false;
        };

        UseOnCharacter::new(act_id, chr_id).can(self)
    }

    pub fn can_use_on_enemy_chr(&mut self, act_id: ActiveID) -> bool {
        let Some(chr_id) = self.state.other_subturner_on_field().chr_id else {
            return false;
        };

        UseOnCharacter::new(act_id, chr_id).can(self)
    }

    pub fn stat(&mut self, chr_id: CharacterID, stat_type: StatType) -> Stat0 {
        let value = self.state.chr(chr_id).stats.stat(stat_type);
        StatMap::new(chr_id, stat_type).map(self, value)
    }

    pub fn end_subturn(&mut self) {
        self.state.current_subturner.switch()
    }

    pub fn heal_on_field_leave(&mut self, _chr_id: CharacterID) {
        todo!()
    }

    /* pub fn attack(
        &mut self,
        attacker_id: CharacterID,
        target_id: CharacterID,
    ) {
        let dmg = self.chr(attacker_id).stats.dmg.0;
        self.attack_map(attacker_id, target_id, dmg)
    } */
}

impl Game<'_, '_> {
    pub fn choose_chr_in_hand(
        &mut self,
        args: ChooseCardArgsP<'_, CharacterID>,
    ) -> Option<CharacterID> {
        self.input.choose_chr_in_hand(self.state, args)
    }

    pub fn choose_act_in_hand(&mut self, args: ChooseCardArgsP<'_, ActiveID>) -> Option<ActiveID> {
        self.input.choose_act_in_hand(self.state, args)
    }

    pub fn choose_chr_in_hand_any(&mut self, args: ChooseCardArgs) -> Option<CharacterID> {
        self.input.choose_chr_in_hand(self.state, ChooseCardArgsP::new(args, &|_, _| true))
    }

    pub fn choose_act_in_hand_any(&mut self, args: ChooseCardArgs) -> Option<ActiveID> {
        self.input.choose_act_in_hand(self.state, ChooseCardArgsP::new(args, &|_, _| true))
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
        self.input.choose_chr_on_field(self.state, ChooseCardArgsP::new(args, &|_, _| true))
    }

    pub fn choose_act_on_field_any(&mut self, args: ChooseCardArgs) -> Option<ActiveID> {
        self.input.choose_act_on_field(self.state, ChooseCardArgsP::new(args, &|_, _| true))
    }
}
