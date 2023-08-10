use {
    self::event::Event,
    std::collections::HashSet,
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
    pub fn get(&self, id: ID) -> &CardInfo {
        self.cards.get(&id).unwrap()
    }

    // FIXME: remove pub
    pub fn get_mut(&mut self, id: ID) -> &mut CardInfo {
        self.cards.get_mut(&id).unwrap()
    }

    // FIXME: remove pub
    pub fn add(&mut self, card: CardInfo) -> ID {
        let id = self.id_manager.next_id();
        self.cards.insert(id, card);
        id
    }

    pub fn hand(&self, player_id: PlayerID) -> &HashSet<ID> {
        self.hands.get(&player_id).unwrap()
    }

    // FIXME: remove pub
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

    pub fn try_find_owner_in_decks(&self, id: ID) -> Option<PlayerID> {
        for (&player_id, hand) in self.hands.iter() {
            if hand.contains(&id) {
                return Some(player_id);
            }
        }
        None
    }

    pub fn find_owner_in_decks(&self, id: ID) -> PlayerID {
        self.try_find_owner_in_decks(id).unwrap()
    }

    pub fn remove_from_some_player(&mut self, id: ID) -> PlayerID {
        let player_id = self
            .try_find_owner_in_decks(id)
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

pub struct SubturnerOnField {
    pub player_id: PlayerID,
    pub chr_id: Option<CharacterID>,
    pub used_act_ids: Vec<ActiveID>,
}

impl SubturnerOnField {
    fn new(player_id: PlayerID) -> Self {
        Self {
            player_id,
            chr_id: None,
            used_act_ids: Vec::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Subturner {
    Attacker,
    Defender,
}

impl Subturner {
    pub fn switch(&mut self) {
        *self = self.other()
    }

    pub fn other(self) -> Self {
        match self {
            Self::Attacker => Self::Defender,
            Self::Defender => Self::Attacker,
        }
    }
}

pub struct Nested<T> {
    pub children: Vec<Self>,
    pub value: T,
}

pub struct GameState {
    pub chrs: GameOfCardType<CharacterID, CharacterInfo>,
    pub acts: GameOfCardType<ActiveID, ActiveInfo>,

    pub players_map: BTreeMap<PlayerID, Player>,

    pub attacker: SubturnerOnField,
    pub defender: SubturnerOnField,
    pub current_subturner: Subturner,

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

        let mut res = Self {
            chrs,
            acts,

            players_map,

            attacker: SubturnerOnField::new(attacker_id),
            defender: SubturnerOnField::new(defender_id),
            current_subturner: Subturner::Attacker,

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
        self.init_gain_pile();

        for player_id in self.players_map.keys().copied() {
            self.chrs.pick_n(player_id, Self::INIT_CHARACTERS_PER_HAND);
            self.acts.pick_n(player_id, Self::INIT_ACTIVES_PER_HAND);
        }
    }

    fn init_gain_pile(&mut self) {
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

    pub fn change_turner(&mut self) {
        let new_attacker_id = self.defender.player_id;
        let new_defender_id = self.pick_defender_id(new_attacker_id);

        self.attacker = SubturnerOnField::new(new_attacker_id);
        self.defender = SubturnerOnField::new(new_defender_id);
    }

    pub fn is_dead(&self, chr_id: CharacterID) -> bool {
        self.chr(chr_id).stats.vit.0 == 0
    }
}

impl GameState {
    pub fn subturner_by_id(&self, player_id: PlayerID) -> Subturner {
        self.try_subturner_by_id(player_id).unwrap()
    }

    pub fn try_subturner_by_id(&self, player_id: PlayerID) -> Option<Subturner> {
        if player_id == self.current_subturner_on_field().player_id {
            Some(self.current_subturner)
        } else if player_id == self.other_subturner_on_field().player_id {
            Some(self.current_subturner.other())
        } else {
            None
        }
    }

    pub fn try_subturner_on_field_by_id(&self, player_id: PlayerID) -> Option<&SubturnerOnField> {
        Some(self.subturner_on_field(self.try_subturner_by_id(player_id)?))
    }

    pub fn try_other_subturner_on_field_by_id(
        &self,
        player_id: PlayerID,
    ) -> Option<&SubturnerOnField> {
        Some(self.subturner_on_field(self.try_subturner_by_id(player_id)?.other()))
    }

    pub fn subturner_on_field(&self, subturner: Subturner) -> &SubturnerOnField {
        match subturner {
            Subturner::Attacker => &self.attacker,
            Subturner::Defender => &self.defender,
        }
    }

    pub fn subturner_on_field_mut(&mut self, subturner: Subturner) -> &mut SubturnerOnField {
        match subturner {
            Subturner::Attacker => &mut self.attacker,
            Subturner::Defender => &mut self.defender,
        }
    }

    pub fn current_subturner_on_field(&self) -> &SubturnerOnField {
        self.subturner_on_field(self.current_subturner)
    }

    pub fn current_subturner_on_field_mut(&mut self) -> &mut SubturnerOnField {
        self.subturner_on_field_mut(self.current_subturner)
    }

    pub fn other_subturner_on_field(&self) -> &SubturnerOnField {
        self.subturner_on_field(self.current_subturner.other())
    }

    pub fn other_subturner_mut(&mut self) -> &mut SubturnerOnField {
        self.subturner_on_field_mut(self.current_subturner.other())
    }

    pub fn try_own_chr_id(&self, player_id: PlayerID) -> Option<CharacterID> {
        self.try_subturner_on_field_by_id(player_id)?.chr_id
    }

    pub fn own_chr_id(&self, player_id: PlayerID) -> CharacterID {
        self.try_own_chr_id(player_id).unwrap()
    }

    pub fn try_enemy_chr_id(&self, player_id: PlayerID) -> Option<CharacterID> {
        self.try_other_subturner_on_field_by_id(player_id)?.chr_id
    }

    pub fn enemy_chr_id(&self, player_id: PlayerID) -> CharacterID {
        self.try_enemy_chr_id(player_id).unwrap()
    }

    pub fn try_enemy_id(&self, chr_id: CharacterID) -> Option<CharacterID> {
        self.try_enemy_chr_id(self.try_find_owner_of_chr(chr_id)?)
    }

    pub fn enemy_id(&self, chr_id: CharacterID) -> CharacterID {
        self.try_enemy_id(chr_id).unwrap()
    }
}

// TODO: вынести в отдельный файл?
pub struct Anchor(usize);

impl GameState {
    pub fn event_handling_card_ids<'s>(&'s self) -> impl Iterator<Item = CardID> + 's {
        let attacker = self.attacker.chr_id.into_iter().map(CardID::Character);
        let defender = self.defender.chr_id.into_iter().map(CardID::Character);

        // TODO: used actives

        let attacker_hand_acts = self
            .acts
            .hand(self.attacker.player_id)
            .iter()
            .copied()
            .map(CardID::Active);

        let defender_hand_acts = self
            .acts
            .hand(self.attacker.player_id)
            .iter()
            .copied()
            .map(CardID::Active);

        attacker
            .chain(defender)
            .chain(attacker_hand_acts)
            .chain(defender_hand_acts)
    }

    pub fn anchor(&self) -> Anchor {
        Anchor(self.events.len())
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

    fn undo(
        &mut self,
        Nested {
            children,
            value: SignedEvent { signature, value },
        }: Nested<SignedEvent>,
    ) {
        match value {
            Event::Use { act_id, .. } => {
                let owner_id = self.find_owner_of_act(act_id);

                self.acts.remove_from_wastepile(act_id);
                self.acts.add_to_player(act_id, owner_id)
            }

            Event::StatChange {
                chr_id,
                stat_type,
                old_value,
                ..
            } => {
                self.chr_mut(chr_id)
                    .stats
                    .set(stat_type, old_value.unwrap());
            }

            Event::Attack { .. } => {}

            Event::GetHurt { .. } => {}

            Event::TakeCharacter { player_id, chr_id } => todo!(),

            Event::TakeActive { player_id, act_id } => todo!(),

            Event::Place { chr_id } => {
                let owner_id = self.find_owner_on_field_of_chr(chr_id);
                let subturner = self.subturner_by_id(owner_id);

                self.subturner_on_field_mut(subturner).chr_id = None;
                self.chrs.add_to_player(chr_id, owner_id);
            }

            Event::Die { chr_id } => todo!(),

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

impl GameState {
    pub fn try_find_owner_on_field_of_chr(&self, chr_id: CharacterID) -> Option<PlayerID> {
        if self.attacker.chr_id == Some(chr_id) {
            return Some(self.attacker.player_id);
        }

        if self.defender.chr_id == Some(chr_id) {
            return Some(self.defender.player_id);
        }

        None
    }

    pub fn find_owner_on_field_of_chr(&self, chr_id: CharacterID) -> PlayerID {
        self.try_find_owner_on_field_of_chr(chr_id).unwrap()
    }

    pub fn try_find_owner_of_chr(&self, chr_id: CharacterID) -> Option<PlayerID> {
        if let Some(owner_id) = self.try_find_owner_on_field_of_chr(chr_id) {
            return Some(owner_id);
        }

        self.chrs.try_find_owner_in_decks(chr_id)
    }

    pub fn find_owner_of_chr(&self, chr_id: CharacterID) -> PlayerID {
        self.try_find_owner_of_chr(chr_id).unwrap()
    }
}

impl GameState {
    pub fn try_find_owner_on_field_of_act(&self, act_id: ActiveID) -> Option<PlayerID> {
        if self.attacker.used_act_ids.contains(&act_id) {
            return Some(self.attacker.player_id);
        }

        if self.defender.used_act_ids.contains(&act_id) {
            return Some(self.defender.player_id);
        }

        None
    }

    pub fn find_owner_on_field_of_act(&self, act_id: ActiveID) -> PlayerID {
        self.try_find_owner_on_field_of_act(act_id).unwrap()
    }

    pub fn try_find_owner_of_act(&self, act_id: ActiveID) -> Option<PlayerID> {
        if let Some(owner_id) = self.try_find_owner_on_field_of_act(act_id) {
            return Some(owner_id);
        }

        self.acts.try_find_owner_in_decks(act_id)
    }

    pub fn find_owner_of_act(&self, act_id: ActiveID) -> PlayerID {
        self.try_find_owner_of_act(act_id).unwrap()
    }
}

impl GameState {
    fn remove_from_field(&mut self, subturner: Subturner) {
        todo!()
    }
}
