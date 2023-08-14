use {
    self::{
        event::Event,
        player_id::PlayerOwned,
    },
    crate::act_uses::StatType,
    std::{
        collections::HashSet,
        iter::once,
    },
};

pub mod act_id;
pub mod act_info;
pub mod chr_id;
pub mod chr_info;
pub mod event;
pub mod id_manager;
pub mod player_id;
pub mod player_id_manager;

use {
    self::{
        act_id::ActiveID,
        act_info::ActiveInfo,
        chr_id::CharacterID,
        chr_info::CharacterInfo,
        event::SignedEvent,
        id_manager::{
            id_trait::IDTrait,
            IDManager,
        },
        player_id::PlayerID,
    },
    crate::{
        card_uses::{
            ActiveType,
            CharacterType,
        },
        game::CardID,
        group::Group,
    },
    itertools::Itertools,
    rand::{
        seq::IteratorRandom,
        thread_rng,
    },
    std::{
        collections::{
            BTreeMap,
            HashMap,
        },
        fmt::Debug,
    },
};

pub struct CharacterOnField {
    pub player_id: PlayerID,
    pub chr_id: CharacterID,
    pub used_acts: Vec<ActiveID>,
}

pub struct Player {
    pub nickname: String,
}

pub struct GameOfCardType<ID: IDTrait, CardInfo> {
    id_manager: IDManager<ID>,
    cards: HashMap<ID, CardInfo>,

    drawpile: Vec<ID>,
    wastepile: Vec<ID>,

    hands: HashMap<PlayerID, HashSet<ID>>,
}

impl<ID: IDTrait, CardInfo> Default for GameOfCardType<ID, CardInfo> {
    fn default() -> Self {
        Self {
            id_manager: Default::default(),
            cards: Default::default(),

            drawpile: Default::default(),
            wastepile: Default::default(),

            hands: Default::default(),
        }
    }
}

impl<ID: IDTrait + Debug, CardInfo> GameOfCardType<ID, CardInfo> {
    pub fn try_get(&self, id: ID) -> Option<&CardInfo> {
        self.cards.get(&id)
    }

    pub fn get(&self, id: ID) -> &CardInfo {
        self.try_get(id).unwrap()
    }

    pub fn try_get_mut(&mut self, id: ID) -> Option<&mut CardInfo> {
        self.cards.get_mut(&id)
    }

    pub fn get_mut(&mut self, id: ID) -> &mut CardInfo {
        self.try_get_mut(id).unwrap()
    }

    pub fn add(&mut self, card: CardInfo) -> ID {
        let id = self.id_manager.next_id();
        self.cards.insert(id, card);
        id
    }

    pub fn hand(&self, player_id: PlayerID) -> &HashSet<ID> {
        self.hands.get(&player_id).unwrap()
    }

    pub fn hand_mut(&mut self, player_id: PlayerID) -> &mut HashSet<ID> {
        self.hands.get_mut(&player_id).unwrap()
    }

    pub fn draw(&mut self) -> Option<ID> {
        self.drawpile.pop()
    }

    pub fn draw_n(&mut self, cards_count: usize) -> Vec<ID> {
        let mut res = Vec::default();

        for _ in 1..=cards_count {
            let Some(card_id) = self.draw() else {
                return res;
            };

            res.push(card_id);
        }

        res
    }

    pub fn pick(&mut self, player_id: PlayerID) -> Option<ID> {
        let res = self.draw()?;
        self.add_to_player(res, player_id);
        Some(res)
    }

    pub fn pick_n(&mut self, player_id: PlayerID, cards_count: usize) -> Vec<ID> {
        let res = self.draw_n(cards_count);
        for id in res.iter().copied() {
            self.add_to_player(id, player_id);
        }
        res
    }

    pub fn add_to_player(&mut self, id: ID, player_id: PlayerID) {
        self.hand_mut(player_id).insert(id);
    }

    pub fn remove_from_player(&mut self, id: ID, player_id: PlayerID) {
        self.hand_mut(player_id).retain(|&hand_id| hand_id != id);
    }

    pub fn try_owner_id(&self, id: ID) -> Option<PlayerID> {
        for (&player_id, hand) in self.hands.iter() {
            if hand.contains(&id) {
                return Some(player_id);
            }
        }
        None
    }

    pub fn owner_id(&self, id: ID) -> PlayerID {
        self.try_owner_id(id)
            .expect("expected the card to have an owner")
    }

    pub fn remove_from_some_player(&mut self, id: ID) -> PlayerID {
        let player_id = self
            .try_owner_id(id)
            .unwrap_or_else(|| panic!("expected some player to own {:?}", id));
        self.remove_from_player(id, player_id);
        player_id
    }

    pub fn add_to_drawpile(&mut self, id: ID) {
        self.drawpile.push(id);
    }

    pub fn remove_from_drawpile(&mut self, id: ID) {
        self.drawpile.retain(|&pile_id| pile_id != id);
    }

    pub fn add_to_wastepile(&mut self, id: ID) {
        self.wastepile.push(id);
    }

    pub fn remove_from_wastepile(&mut self, id: ID) {
        self.wastepile.retain(|&pile_id| pile_id != id);
    }
}

#[derive(Clone, Copy)]
pub enum Subturner {
    Attacker,
    Defender,
}

impl Subturner {
    pub fn other(self) -> Self {
        match self {
            Self::Attacker => Self::Defender,
            Self::Defender => Self::Attacker,
        }
    }

    pub fn switch(&mut self) {
        *self = self.other()
    }
}

#[derive(Clone)]
pub struct Nested<T> {
    pub children: Vec<Self>,
    pub value: T,
}

#[derive(Clone, Copy)]
pub struct TurnInfo {
    pub attacker_id: PlayerID,
    pub defender_id: PlayerID,
    pub subturner: Subturner,
}

impl TurnInfo {
    pub fn new(attacker_id: PlayerID, defender_id: PlayerID) -> Self {
        Self {
            attacker_id,
            defender_id,
            subturner: Subturner::Attacker,
        }
    }

    pub fn subturner_id(self) -> PlayerID {
        self.id_by_subturner(self.subturner)
    }

    pub fn other_subturner_id(self) -> PlayerID {
        self.id_by_subturner(self.subturner.other())
    }

    pub fn subturner_by_id(self, player_id: PlayerID) -> Option<Subturner> {
        if player_id == self.attacker_id {
            Some(Subturner::Attacker)
        } else if player_id == self.defender_id {
            Some(Subturner::Defender)
        } else {
            None
        }
    }

    pub fn id_by_subturner(self, subturner: Subturner) -> PlayerID {
        match subturner {
            Subturner::Attacker => self.attacker_id,
            Subturner::Defender => self.defender_id,
        }
    }
}

pub struct GameState {
    pub chrs: GameOfCardType<CharacterID, CharacterInfo>,
    pub acts: GameOfCardType<ActiveID, ActiveInfo>,

    pub players_map: BTreeMap<PlayerID, Player>,

    pub turn_info: TurnInfo,
    pub cards_on_field: Vec<PlayerOwned<CardID>>,

    events: Vec<Nested<SignedEvent>>,
}

impl GameState {
    pub fn new(players: Vec<Player>) -> Self {
        let mut chrs = GameOfCardType::default();
        let mut acts = GameOfCardType::default();

        let mut player_id_manager = IDManager::default();
        let mut players_map = BTreeMap::default();

        for player in players {
            let id = player_id_manager.next_id();

            players_map.insert(id, player);
            chrs.hands.insert(id, Default::default());
            acts.hands.insert(id, Default::default());
        }

        let Some((attacker_id, defender_id)) = players_map.keys().copied().next_tuple() else {
            panic!("not enough players");
        };
        let turn_info = TurnInfo::new(attacker_id, defender_id);

        let mut res = Self {
            chrs,
            acts,

            players_map,

            turn_info,
            cards_on_field: Default::default(),

            events: Default::default(),
        };
        res.init_cards();
        res
    }

    // TODO сделать конструктор принимающим GameInitInfo или GameConfig со всеми этими параметрами
    const INIT_CHARACTERS_PER_HAND: usize = 3;
    const INIT_ACTIVES_PER_HAND: usize = 6;

    const CHARACTERS_GAINED_AFTER_TURN: usize = 2;
    const ACTIVES_GAINED_AFTER_TURN: usize = 4;

    const TOTAL_GAINS_PER_PLAYER: usize = 4;

    fn init_cards(&mut self) {
        self.init_drawpile();

        for player_id in self.players_map.keys().copied() {
            self.chrs.pick_n(player_id, Self::INIT_CHARACTERS_PER_HAND);
            self.acts.pick_n(player_id, Self::INIT_ACTIVES_PER_HAND);
        }
    }

    fn init_drawpile(&mut self) {
        let players_count = self.players_map.len();

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

            let chr_id = self.add_chr(chr);
            self.chrs.add_to_drawpile(chr_id);
        }

        for _ in 1..=total_acts_count {
            let act_type = ActiveType::all()
                .into_iter()
                .filter(|&chr_type| !chr_type.groups().contains(&Group::Нераздаваемая))
                .choose(&mut thread_rng())
                .unwrap();

            let act = ActiveInfo::new(act_type);

            let act_id = self.add_act(act);
            self.acts.add_to_drawpile(act_id);
        }
    }
}

impl GameState {
    pub fn chr(&self, id: CharacterID) -> &CharacterInfo {
        self.chrs.get(id)
    }

    pub fn chr_mut(&mut self, id: CharacterID) -> &mut CharacterInfo {
        self.chrs.get_mut(id)
    }

    pub fn act(&self, id: ActiveID) -> &ActiveInfo {
        self.acts.get(id)
    }

    pub fn act_mut(&mut self, id: ActiveID) -> &mut ActiveInfo {
        self.acts.get_mut(id)
    }

    pub fn add_chr(&mut self, chr: CharacterInfo) -> CharacterID {
        self.chrs.add(chr)
    }

    pub fn add_act(&mut self, act: ActiveInfo) -> ActiveID {
        self.acts.add(act)
    }

    pub fn pick_defender_id(&self, attacker_id: PlayerID) -> PlayerID {
        self.players_map
            .keys()
            .copied()
            .cycle()
            .skip_while(|&key| key != attacker_id)
            .nth(1)
            .unwrap()
    }

    pub fn end_turn(&mut self) {
        let new_attacker_id = self.turn_info.defender_id;
        let new_defender_id = self.pick_defender_id(new_attacker_id);

        self.turn_info = TurnInfo::new(new_attacker_id, new_defender_id);
    }
}

// TODO: вынести в отдельный файл?
pub struct Anchor(usize);

impl GameState {
    pub fn all_chrs_on_field<'s>(&'s self) -> impl Iterator<Item = CharacterID> + 's {
        self.cards_on_field
            .iter()
            .copied()
            .filter_map(|owned_card_id| match owned_card_id.value {
                CardID::Character(chr_id) => Some(chr_id),
                _ => None,
            })
    }

    pub fn all_acts_on_field<'s>(&'s self) -> impl Iterator<Item = ActiveID> + 's {
        self.cards_on_field
            .iter()
            .copied()
            .filter_map(|owned_card_id| match owned_card_id.value {
                CardID::Active(act_id) => Some(act_id),
                _ => None,
            })
    }

    pub fn chrs_on_field<'s>(
        &'s self,
        player_id: PlayerID,
    ) -> impl Iterator<Item = CharacterID> + 's {
        self.all_chrs_on_field()
            .filter(move |&chr_id| self.owner_id(chr_id) == player_id)
    }

    pub fn owner_id(&self, card_id: impl Into<CardID>) -> PlayerID {
        self.try_owner_id(card_id)
            .expect("expected the card to have an owner")
    }

    pub fn try_owner_id(&self, card_id: impl Into<CardID>) -> Option<PlayerID> {
        let card_id = card_id.into();

        for owned_card in self.cards_on_field.iter().copied() {
            if owned_card.value == card_id {
                return Some(owned_card.owner_id);
            }
        }

        match card_id {
            CardID::Character(chr_id) => {
                if let res @ Some(_) = self.chrs.try_owner_id(chr_id) {
                    return res;
                }
            }

            CardID::Active(act_id) => {
                if let res @ Some(_) = self.acts.try_owner_id(act_id) {
                    return res;
                }
            }
        }

        todo!()
    }

    pub fn event_handling_card_ids<'s>(&'s self) -> impl Iterator<Item = CardID> + 's {
        let cards_on_field = self
            .cards_on_field
            .iter()
            .copied()
            .map(|owned_card_id| owned_card_id.value);

        let attacker_hand_chrs = self
            .chrs
            .hand(self.turn_info.attacker_id)
            .iter()
            .copied()
            .map(CardID::Character);

        let attacker_hand_acts = self
            .acts
            .hand(self.turn_info.attacker_id)
            .iter()
            .copied()
            .map(CardID::Active);

        let defender_hand_chrs = self
            .chrs
            .hand(self.turn_info.defender_id)
            .iter()
            .copied()
            .map(CardID::Character);

        let defender_hand_acts = self
            .acts
            .hand(self.turn_info.defender_id)
            .iter()
            .copied()
            .map(CardID::Active);

        cards_on_field
            .chain(attacker_hand_chrs)
            .chain(attacker_hand_acts)
            .chain(defender_hand_chrs)
            .chain(defender_hand_acts)
    }
}

impl GameState {
    pub fn anchor(&self) -> Anchor {
        Anchor(self.events.len())
    }

    pub fn events_flatten<'s>(&'s self) -> impl Iterator<Item = SignedEvent> + 's {
        // динамика необходима, иначе бесконечный рекурсивный тип
        fn helper<'events>(
            events: &'events [Nested<SignedEvent>],
        ) -> Box<dyn Iterator<Item = SignedEvent> + 'events> {
            Box::new(
                events
                    .iter()
                    .map(|event| {
                        helper(&event.children)
                            .into_iter()
                            .chain(once(event.value.clone()))
                    })
                    .flatten(),
            )
        }

        helper(&self.events)
    }

    pub fn push_event(&mut self, signed_event: Nested<SignedEvent>) {
        self.events.push(signed_event);
    }

    pub fn revert_to(&mut self, anchor: Anchor) {
        let signed_events = self.extract_events_from(anchor);
        self.undo_all(signed_events);
    }

    pub fn extract_events_from(&mut self, anchor: Anchor) -> Vec<Nested<SignedEvent>> {
        self.events.split_off(anchor.0)
    }
}

impl GameState {
    fn undo(
        &mut self,
        Nested {
            children,
            value: SignedEvent { value, .. },
        }: Nested<SignedEvent>,
    ) {
        match value {
            Event::Use { act_id, .. } => {
                let PlayerOwned {
                    owner_id,
                    value: _act_id,
                } = self.cards_on_field.pop().unwrap();

                assert_eq!(_act_id, CardID::Active(act_id));

                self.acts.add_to_player(act_id, owner_id);
            }

            Event::StatChange {
                chr_id,
                stat_type,
                old_value,
                old_vit_value,
                ..
            } => {
                self.chr_mut(chr_id)
                    .stats
                    .set(stat_type, old_value.unwrap());

                if let Some(old_vit_value) = old_vit_value {
                    self.chr_mut(chr_id)
                        .stats
                        .set(StatType::Vitality, old_vit_value);
                }
            }

            Event::Attack { .. } => {}

            Event::GetHurt { .. } => {}

            Event::MorphCharacter {
                chr_id, old_info, ..
            } => {
                *self.chr_mut(chr_id) = old_info.unwrap();
            }

            Event::MorphActive {
                act_id, old_info, ..
            } => {
                *self.act_mut(act_id) = old_info.unwrap();
            }

            Event::TakeCharacter { player_id, chr_id } => {
                let chr_id = chr_id.unwrap();

                self.chrs.remove_from_player(chr_id, player_id);
                self.chrs.add_to_drawpile(chr_id);
            }

            Event::TakeActive { player_id, act_id } => {
                let act_id = act_id.unwrap();

                self.acts.remove_from_player(act_id, player_id);
                self.acts.add_to_drawpile(act_id);
            }

            Event::PutCharacterInDrawpile { chr_id } => todo!(),

            Event::PutActiveInDrawpile { act_id } => todo!(),

            Event::Place { chr_id } => {
                let PlayerOwned {
                    owner_id,
                    value: _chr_id,
                } = self.cards_on_field.pop().unwrap();

                assert_eq!(_chr_id, CardID::Character(chr_id));

                self.chrs.add_to_player(chr_id, owner_id);
            }

            Event::Die { chr_id } => todo!(),

            Event::EndSubturn => {
                self.turn_info.subturner.switch();
            }

            Event::EndTurn => todo!(),

            Event::Replace {
                replaced_chr_id,
                replacing_chr_id,
            } => todo!(),

            Event::HealOnFieldLeave { chr_id, heal_value } => todo!(),

            Event::Random { .. } => {}
            Event::RandomBool { .. } => {}
        }

        self.undo_all(children);
    }

    fn undo_all(&mut self, signed_events: Vec<Nested<SignedEvent>>) {
        for signed_event in signed_events.into_iter().rev() {
            self.undo(signed_event);
        }
    }
}
