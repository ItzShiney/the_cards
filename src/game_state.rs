pub mod ability_description;
pub mod act_id;
pub mod act_info;
pub mod chr_id;
pub mod chr_info;
pub mod condition;
pub mod group;
pub mod id_manager;
pub mod player_id;
pub mod player_id_manager;

use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::mem::take;

use crate::acts::ActiveType;
use crate::chrs::CharacterType;
use itertools::Itertools;
use rand::seq::IteratorRandom;
use rand::thread_rng;

use self::act_id::ActiveID;
use self::act_info::ActiveInfo;
use self::chr_id::CharacterID;
use self::chr_info::CharacterInfo;
use self::condition::Condition;
use self::id_manager::id_trait::IDTrait;
use self::id_manager::IDManager;
use self::player_id::PlayerID;

////////////////////////////////////////////////////////////

pub struct CharacterOnField {
    pub player_id: PlayerID,
    pub chr_id: CharacterID,
    pub used_acts: Vec<ActiveID>,
}

#[derive(Clone, Copy)]
pub enum DamageSource {
    None,
    Character { chr_id: CharacterID },
    Active { act_id: ActiveID },
}

#[derive(Default)]
pub struct CardTyped<C, A> {
    pub chr: C,
    pub act: A,
}

pub struct Player {
    pub nickname: String,
}

pub struct GameOfCardType<ID: IDTrait, CardInfo> {
    id_manager: IDManager<ID>,
    cards: HashMap<ID, CardInfo>,

    gain_pile: Vec<ID>,
    waste_pile: Vec<ID>,

    hands: HashMap<PlayerID, Vec<ID>>,
}

impl<ID: IDTrait, CardInfo> Default for GameOfCardType<ID, CardInfo> {
    fn default() -> Self {
        Self {
            id_manager: Default::default(),
            cards: Default::default(),

            gain_pile: Default::default(),
            waste_pile: Default::default(),

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

    pub fn hand(&self, player_id: PlayerID) -> &Vec<ID> {
        self.hands.get(&player_id).unwrap()
    }

    // FIXME: remove pub
    pub fn hand_mut(&mut self, player_id: PlayerID) -> &mut Vec<ID> {
        self.hands.get_mut(&player_id).unwrap()
    }

    pub fn gain(&mut self, player_id: PlayerID, cards_count: usize) -> usize {
        for cards_gained in 1..=cards_count {
            let Some(card_id) = self.gain_pile.pop() else {
                return cards_gained - 1;
            };

            self.add_to_player(card_id, player_id);
        }
        cards_count
    }

    pub fn add_to_player(&mut self, id: ID, player_id: PlayerID) {
        self.hand_mut(player_id).push(id);
    }

    pub fn remove_from_player(&mut self, id: ID, player_id: PlayerID) {
        self.hand_mut(player_id).retain(|&hand_id| hand_id != id);
    }

    pub fn find_owner(&self, id: ID) -> Option<PlayerID> {
        for (&player_id, hand) in self.hands.iter() {
            if hand.contains(&id) {
                return Some(player_id);
            }
        }
        None
    }

    pub fn remove_from_some_player(&mut self, id: ID) -> PlayerID {
        let player_id =
            self.find_owner(id).expect(format!("expected some player to own {:?}", id).as_str());
        self.remove_from_player(id, player_id);
        player_id
    }

    pub fn add_to_gain_pile(&mut self, id: ID) {
        self.gain_pile.push(id);
    }

    pub fn remove_from_gain_pile(&mut self, id: ID) {
        self.gain_pile.retain(|&pile_id| pile_id != id);
    }

    pub fn add_to_waste_pile(&mut self, id: ID) {
        self.waste_pile.push(id);
    }

    pub fn remove_from_waste_pile(&mut self, id: ID) {
        self.waste_pile.retain(|&pile_id| pile_id != id);
    }
}

pub struct SubturnerOnField {
    pub player_id: PlayerID,
    pub chr_id: Option<CharacterID>,
    pub used_act_ids: Vec<ActiveID>,
}

impl SubturnerOnField {
    fn new(player_id: PlayerID) -> Self {
        Self { player_id, chr_id: None, used_act_ids: Vec::default() }
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

pub struct GameState {
    pub chrs: GameOfCardType<CharacterID, CharacterInfo>,
    pub acts: GameOfCardType<ActiveID, ActiveInfo>,

    players_map: BTreeMap<PlayerID, Player>,

    attacker: SubturnerOnField,
    defender: SubturnerOnField,
    current_subturner: Subturner,
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

        let Some((attacker_id, defender_id)) =
            players_map.keys().copied().next_tuple() else {
                panic!("not enough players");
            };

        let mut game = Self {
            chrs,
            acts,

            players_map,

            attacker: SubturnerOnField::new(attacker_id),
            defender: SubturnerOnField::new(defender_id),
            current_subturner: Subturner::Attacker,
        };
        game.init_cards();
        game
    }

    const INIT_CHARACTERS_PER_HAND: usize = 3;
    const INIT_ACTIVES_PER_HAND: usize = 6;

    const CHARACTERS_GAINED_AFTER_TURN: usize = 2;
    const ACTIVES_GAINED_AFTER_TURN: usize = 4;

    const TOTAL_GAINS_PER_PLAYER: usize = 4;

    fn init_cards(&mut self) {
        self.init_gain_pile();

        for player_id in self.players_map.keys().copied() {
            self.chrs.gain(player_id, Self::INIT_CHARACTERS_PER_HAND);
            self.acts.gain(player_id, Self::INIT_ACTIVES_PER_HAND);
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
            let chr_types = CharacterType::all();
            let chr_type = chr_types.into_iter().choose(&mut thread_rng()).unwrap();
            let chr = chr_type.into();

            let chr_id = self.add_chr(chr);
            self.chrs.add_to_gain_pile(chr_id);
        }

        for _ in 1..=total_acts_count {
            let act_types = ActiveType::all();
            let act_type = act_types.into_iter().choose(&mut thread_rng()).unwrap();
            let act = act_type.into();

            let act_id = self.add_act(act);
            self.acts.add_to_gain_pile(act_id);
        }
    }
}

impl GameState {
    pub fn chr(&self, id: CharacterID) -> &CharacterInfo {
        self.chrs.get(id)
    }

    pub fn act(&self, id: ActiveID) -> &ActiveInfo {
        self.acts.get(id)
    }

    pub fn chr_mut(&mut self, id: CharacterID) -> &mut CharacterInfo {
        self.chrs.get_mut(id)
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

    pub fn attacker(&self) -> &SubturnerOnField {
        &self.attacker
    }

    pub fn attacker_mut(&mut self) -> &mut SubturnerOnField {
        &mut self.attacker
    }

    pub fn defender(&self) -> &SubturnerOnField {
        &self.defender
    }

    pub fn defender_mut(&mut self) -> &mut SubturnerOnField {
        &mut self.defender
    }

    pub fn end_subturn(&mut self) {
        self.current_subturner.switch()
    }

    pub fn end_turn(&mut self) {
        self.remove_from_field(self.current_subturner);
        self.remove_from_field(self.current_subturner.other());

        self.change_turner();
    }

    pub fn current_subturner(&self) -> Subturner {
        self.current_subturner
    }

    pub fn subturner_on_field(&self, subturner: Subturner) -> &SubturnerOnField {
        match subturner {
            Subturner::Attacker => self.attacker(),
            Subturner::Defender => self.defender(),
        }
    }

    pub fn subturner_on_field_mut(&mut self, subturner: Subturner) -> &mut SubturnerOnField {
        match subturner {
            Subturner::Attacker => self.attacker_mut(),
            Subturner::Defender => self.defender_mut(),
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
}

// TODO: переместить в Host
impl GameState {
    fn remove_from_field(&mut self, subturner: Subturner) {
        let subturner_on_field = self.subturner_on_field_mut(subturner);

        let Some(chr_id) = subturner_on_field.chr_id.take() else { panic!("expected chr to be on field") };
        let owner_id = subturner_on_field.player_id;
        let used_act_ids = take(&mut subturner_on_field.used_act_ids);

        if self.is_dead(chr_id) {
            self.chrs.add_to_waste_pile(chr_id);
        } else {
            self.chrs.add_to_player(chr_id, owner_id);
        }

        for act_id in used_act_ids {
            // TODO: Host::waste
            self.acts.add_to_waste_pile(act_id);
        }
    }

    fn is_dead(&self, chr_id: CharacterID) -> bool {
        self.chr(chr_id).stats.vit.0.into_value() == Some(0)
    }

    fn change_turner(&mut self) {
        let new_attacker_id = self.defender.player_id;
        let new_defender_id = self.pick_defender_id(new_attacker_id);

        self.attacker = SubturnerOnField::new(new_attacker_id);
        self.defender = SubturnerOnField::new(new_defender_id);
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
}

impl GameState {
    pub fn check_conditions(&self, conditions: &[Condition]) -> bool {
        conditions.iter().all(|condition| self.is_satisfied_condition(condition))
    }

    pub fn is_satisfied_condition(&self, _condition: &Condition) -> bool {
        todo!()
    }
}
