use {
    crate::{
        act_uses::{
            player_id::{
                PlayerID,
                PlayerOwned,
            },
            CharacterInfo,
            Nested,
            StatChange,
            UseWay,
        },
        card_uses::{
            event::{
                Event,
                SignedEvent,
            },
            Signature,
        },
        chr_uses::{
            Check,
            Sign,
            SignedCheck,
            Stat0,
        },
        game_input::{
            ChooseCardArgs,
            ChooseCardArgsP,
            GameInput,
        },
        game_state::{
            act_id::ActiveID,
            chr_id::CharacterID,
            GameState,
        },
        stats::StatType,
    },
    itertools::Itertools,
    rand::{
        thread_rng,
        Rng,
    },
    std::mem::{
        replace,
        swap,
    },
};

// TODO: move to another file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardID {
    Character(CharacterID),
    Active(ActiveID),
}

impl From<CharacterID> for CardID {
    fn from(value: CharacterID) -> Self {
        Self::Character(value)
    }
}

impl From<ActiveID> for CardID {
    fn from(value: ActiveID) -> Self {
        Self::Active(value)
    }
}

pub struct Game<'state, 'input> {
    pub state: &'state mut GameState,
    pub input: &'input mut dyn GameInput,
}

#[derive(Debug)]
pub struct Cancelled(pub &'static str);

pub type EventResult = Result<SignedEvent, Cancelled>;
pub type CheckResult = Result<SignedCheck, Cancelled>;

impl SignedEvent {
    pub fn can(self, game: &mut Game) -> bool {
        game.can(self)
    }

    pub fn force(self, game: &mut Game) -> SignedEvent {
        game.force(self)
    }

    pub fn try_(self, game: &mut Game) -> EventResult {
        game.try_(self)
    }
}

impl Game<'_, '_> {
    pub fn can(&mut self, signed_event: SignedEvent) -> bool {
        let anchor = self.state.anchor();
        let res = self.try_(signed_event);

        match res {
            Ok(_) => {
                self.state.revert_to(anchor);
                true
            }

            Err(_err) => {
                // eprintln!("{}", _err.0);
                false
            }
        }
    }

    pub fn force(&mut self, signed_event: SignedEvent) -> SignedEvent {
        self.try_(signed_event)
            .expect("expected forced event not to be cancelled")
    }

    pub fn try_(&mut self, signed_event: SignedEvent) -> EventResult {
        let anchor = self.state.anchor();
        let maybe_signed_event = self.chain_event(signed_event);

        match maybe_signed_event {
            Ok(signed_event) => {
                let children = self.state.extract_events_from(anchor);
                let nested_event = Nested {
                    children,
                    value: signed_event,
                };

                self.state.push_event(nested_event);
                Ok(signed_event)
            }

            err @ Err(_) => {
                self.state.revert_to(anchor);
                err
            }
        }
    }

    fn chain_event(&mut self, mut signed_event: SignedEvent) -> EventResult {
        signed_event = self.pre_chain_event(signed_event)?;

        for card_id in self.state.event_handling_card_ids().collect_vec() {
            match card_id {
                CardID::Character(chr_id) => {
                    let type_ = self.state.chr(chr_id).type_;
                    signed_event = type_.handle_event(self, chr_id, signed_event)?;
                }

                CardID::Active(act_id) => {
                    let type_ = self.state.act(act_id).type_;
                    signed_event = type_.handle_event(self, act_id, signed_event)?;
                }
            }
        }

        signed_event = self.post_chain_event(signed_event)?;
        Ok(signed_event)
    }

    // TODO: сделать что-то с возможностью инвалидировать ивент в любом handle_event
    fn pre_chain_event(&mut self, signed_event: SignedEvent) -> EventResult {
        let SignedEvent { value, signature } = signed_event;

        match value {
            Event::StatChange {
                chr_id, stat_type, ..
            } => {
                if self.is_const(chr_id, stat_type, signature) {
                    return Err(Cancelled("changing const stat"));
                }

                if signature != Signature::Character(chr_id)
                    && self.is_private(chr_id, stat_type, signature)
                {
                    return Err(Cancelled("changing else's private stat"));
                }
            }

            Event::Place { chr_id } => {
                let Some(owner_id) = self.state.try_owner_id(chr_id) else {
                    return Err(Cancelled("placing chr without an owner"));
                };

                if self.state.turn_info.subturner_id() != owner_id {
                    return Err(Cancelled("else's subturn"));
                }

                if self.has_chr_placed(owner_id) {
                    return Err(Cancelled("already got a chr on field"));
                }
            }

            Event::EndSubturn => {
                let Signature::Player(player_id) = signature else {
                    return Err(Cancelled("cards temporarily can't end subturn"));
                };

                if self.state.turn_info.subturner_id() != player_id {
                    return Err(Cancelled("else's subturn"));
                }

                if !self.has_chr_placed(player_id) {
                    return Err(Cancelled("can't end subturn without a placed chr"));
                }
            }

            _ => {}
        }

        Ok(signed_event)
    }

    fn post_chain_event(&mut self, mut signed_event: SignedEvent) -> EventResult {
        let &mut SignedEvent {
            ref mut value,
            signature,
        } = &mut signed_event;

        match value {
            &mut Event::Place { chr_id } => {
                let Some(owner_id) = self.state.try_owner_id(chr_id) else {
                    return Err(Cancelled("placing a chr without an owner"));
                };

                if self.state.turn_info.subturner_by_id(owner_id).is_none() {
                    return Err(Cancelled("else's turn"));
                }

                self.state.chrs.remove_from_player(chr_id, owner_id);

                self.state.cards_on_field.push(PlayerOwned {
                    owner_id,
                    value: chr_id.into(),
                });
            }

            &mut Event::StatChange {
                chr_id,
                stat_type,
                stat_change,
                ref mut old_value,
                ref mut old_vit_value,
            } => {
                let stats = &mut self.state.chr_mut(chr_id).stats;
                *old_value = Some(stats.stat(stat_type));

                if stat_type == StatType::Physique {
                    *old_vit_value = Some(stats.vit.0);
                }

                match stat_change {
                    StatChange::Add(value) => {
                        if value == 0 {
                            return Err(Cancelled("stat += 0"));
                        }

                        stats.add(stat_type, value);
                    }

                    StatChange::Set(mut value) => {
                        if value < 0 {
                            value = 0;
                        }

                        if stats.stat(stat_type) == value {
                            return Err(Cancelled("stat = stat"));
                        }

                        stats.set(stat_type, value);
                    }

                    _ => todo!(),
                }
            }

            &mut Event::Random {
                min,
                max,
                ref mut output,
            } => {
                let res = thread_rng().gen_range(min..=max);
                *output = Some(res);
            }

            &mut Event::RandomBool {
                true_p,
                ref mut output,
            } => {
                let res = thread_rng().gen_bool(true_p);
                *output = Some(res);
            }

            &mut Event::Use { act_id, .. } => {
                let Some(owner_id) = self.state.try_owner_id(act_id) else {
                    return Err(Cancelled("placing a chr without an owner"));
                };

                if self.state.turn_info.subturner_by_id(owner_id).is_none() {
                    return Err(Cancelled("else's turn"));
                }

                self.state.acts.remove_from_player(act_id, owner_id);

                self.state.cards_on_field.push(PlayerOwned {
                    owner_id,
                    value: act_id.into(),
                });
            }

            &mut Event::Attack { .. } => {}

            &mut Event::GetHurt { .. } => todo!(),

            &mut Event::MorphCharacter {
                chr_id,
                new_info,
                ref mut old_info,
            } => {
                *old_info = Some(replace(self.state.chr_mut(chr_id), new_info));
            }

            &mut Event::MorphActive {
                act_id,
                new_info,
                ref mut old_info,
            } => {
                *old_info = Some(replace(self.state.act_mut(act_id), new_info));
            }

            &mut Event::TakeCharacter { player_id, chr_id } => todo!(),

            &mut Event::TakeActive { player_id, act_id } => todo!(),

            &mut Event::PutCharacterInDrawpile { chr_id } => todo!(),

            &mut Event::PutActiveInDrawpile { act_id } => todo!(),

            &mut Event::Die { chr_id } => todo!(),

            &mut Event::EndSubturn => {
                self.state.turn_info.subturner.switch();
            }

            &mut Event::EndTurn => todo!(),

            &mut Event::Replace {
                replaced_chr_id,
                replacing_chr_id,
            } => {
                if replaced_chr_id == replacing_chr_id {
                    return Err(Cancelled("replacing with the same chr"));
                }

                let Some((popped_idx, _)) = self
                    .state
                    .cards_on_field
                    .iter()
                    .copied()
                    .find_position(|&card_id| card_id.value == CardID::Character(replaced_chr_id))
                else {
                    return Err(Cancelled("replaced chr not found on field"));
                };

                let popped = self.state.cards_on_field.remove(popped_idx);

                let place_result = Event::Place {
                    chr_id: replacing_chr_id,
                }
                .sign(signature)
                .try_(self);

                match place_result {
                    err @ Err(_) => {
                        self.state.cards_on_field.insert(popped_idx, popped);
                        return err;
                    }

                    Ok(new_signed_event) => {
                        signed_event = new_signed_event;
                    }
                }
            }

            &mut Event::HealOnFieldLeave { chr_id, heal_value } => todo!(),
        }

        Ok(signed_event)
    }

    pub fn check(&self, mut signed_check: SignedCheck) -> CheckResult {
        for card_id in self.state.event_handling_card_ids() {
            match card_id {
                CardID::Character(chr_id) => {
                    let type_ = self.state.chr(chr_id).type_;
                    signed_check = type_.handle_check(self, chr_id, signed_check)?;
                }

                CardID::Active(act_id) => {
                    let type_ = self.state.act(act_id).type_;
                    signed_check = type_.handle_check(self, act_id, signed_check)?;
                }
            }
        }

        Ok(signed_check)
    }

    pub fn can_use_in_any_way(&mut self, act_id: ActiveID) -> bool {
        self.can_use_on_own_chr(act_id) || self.can_use_on_enemy_chr(act_id) || {
            let Some(owner_id) = self.state.try_owner_id(act_id) else {
                return false;
            };

            self.can(
                Event::Use {
                    act_id,
                    use_way: UseWay::OnField,
                }
                .sign(owner_id),
            )
        }
    }

    pub fn can_use_on_own_chr(&mut self, act_id: ActiveID) -> bool {
        let Some(owner_id) = self.state.try_owner_id(act_id) else {
            return false;
        };

        let Some(chr_id) = self.state.chrs_on_field(owner_id).next() else {
            return false;
        };

        self.can(
            Event::Use {
                act_id,
                use_way: UseWay::OnCharacter(chr_id),
            }
            .sign(act_id),
        )
    }

    pub fn can_use_on_enemy_chr(&mut self, act_id: ActiveID) -> bool {
        let Some(player_id) = self.state.try_owner_id(act_id) else {
            return false;
        };

        let Some(subturner) = self.state.turn_info.subturner_by_id(player_id) else {
            return false;
        };

        let Some(chr_id) = self
            .state
            .chrs_on_field(self.state.turn_info.id_by_subturner(subturner.other()))
            .next()
        else {
            return false;
        };

        self.can(
            Event::Use {
                act_id,
                use_way: UseWay::OnCharacter(chr_id),
            }
            .sign(act_id),
        )
    }

    pub fn stat(
        &self,
        chr_id: CharacterID,
        stat_type: StatType,
        signature: impl Into<Signature>,
    ) -> Stat0 {
        let value: Stat0 = self.state.chr(chr_id).stats.stat(stat_type);

        let Check::Stat { value, .. } = self
            .check(
                Check::Stat {
                    chr_id,
                    stat_type,
                    value,
                }
                .sign(signature),
            )
            .expect("expected stat check not to be cancelled")
            .value
        else {
            unreachable!()
        };

        value
    }

    pub fn random(&mut self, min: Stat0, max: Stat0, signature: impl Into<Signature>) -> Stat0 {
        let Event::Random {
            output: Some(output),
            ..
        } = self.force(Event::random(min, max).sign(signature)).value
        else {
            unreachable!()
        };

        output
    }

    pub fn has_chr_placed(&self, player_id: PlayerID) -> bool {
        self.state.chrs_on_field(player_id).next().is_some()
    }

    pub fn heal_on_field_leave(
        &mut self,
        chr_id: CharacterID,
        signature: impl Into<Signature>,
    ) -> Event {
        let heal_value = self.stat(chr_id, StatType::Intellect, signature);

        Event::HealOnFieldLeave { chr_id, heal_value }
    }

    pub fn is_private(
        &self,
        chr_id: CharacterID,
        stat_type: StatType,
        signature: impl Into<Signature>,
    ) -> bool {
        self.check(Check::AssumeNonPrivate { chr_id, stat_type }.sign(signature))
            .is_err()
    }

    pub fn is_const(
        &self,
        chr_id: CharacterID,
        stat_type: StatType,
        signature: impl Into<Signature>,
    ) -> bool {
        self.check(Check::AssumeNonConst { chr_id, stat_type }.sign(signature))
            .is_err()
    }
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
        self.input
            .choose_chr_in_hand(self.state, ChooseCardArgsP::new(args, &|_, _| true))
    }

    pub fn choose_act_in_hand_any(&mut self, args: ChooseCardArgs) -> Option<ActiveID> {
        self.input
            .choose_act_in_hand(self.state, ChooseCardArgsP::new(args, &|_, _| true))
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
        self.input
            .choose_chr_on_field(self.state, ChooseCardArgsP::new(args, &|_, _| true))
    }

    pub fn choose_act_on_field_any(&mut self, args: ChooseCardArgs) -> Option<ActiveID> {
        self.input
            .choose_act_on_field(self.state, ChooseCardArgsP::new(args, &|_, _| true))
    }
}
