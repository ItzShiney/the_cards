pub mod ability;
pub mod active_id;
pub mod active_info;
pub mod character_id;
pub mod character_info;
pub mod condition;
pub mod group;
pub mod id_manager;
pub mod player_id;
pub mod player_id_manager;

use std::collections::HashMap;
use std::fmt::Debug;

use crate::acts::ActiveType;
use crate::chrs::CharacterType;
use crate::trigger_trait::TriggerTrait;
use itertools::Itertools;

use self::ability::Ability;
use self::active_id::ActiveID;
use self::active_info::ActiveInfo;
use self::character_id::CharacterID;
use self::character_info::CharacterInfo;
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

    fn get_mut(&mut self, id: ID) -> &mut CardInfo {
        self.cards.get_mut(&id).unwrap()
    }

    fn add(&mut self, card: CardInfo) -> ID {
        let id = self.id_manager.next_id();
        self.cards.insert(id, card);
        id
    }

    pub fn hand(&self, player_id: PlayerID) -> &Vec<ID> {
        self.hands.get(&player_id).unwrap()
    }

    fn hand_mut(&mut self, player_id: PlayerID) -> &mut Vec<ID> {
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

    pub fn remove_from_some_player(&mut self, id: ID) {
        let player_id =
            self.find_owner(id).expect(format!("expected some player to own {:?}", id).as_str());
        self.remove_from_player(id, player_id);
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

pub struct PlayerInTurn {
    pub player_id: PlayerID,
    pub chr_id: Option<CharacterID>,
    pub used_act_ids: Vec<ActiveID>,
}

impl From<PlayerID> for PlayerInTurn {
    fn from(player_id: PlayerID) -> Self {
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

    player_id_manager: IDManager<PlayerID>,
    players_map: HashMap<PlayerID, Player>,

    attacker: PlayerInTurn,
    defender: PlayerInTurn,
    subturner: Subturner,
}

impl GameState {
    pub fn new(players: Vec<Player>) -> Self {
        let mut chrs = Default::default();
        let mut acts = Default::default();

        let mut player_id_manager = IDManager::default();
        let mut players_map = Default::default();

        for player in players {
            Self::add_player_manually(
                &mut player_id_manager,
                &mut players_map,
                &mut chrs,
                &mut acts,
                player,
            );
        }

        let Some((attacker_id, defender_id)) =
            players_map.keys().copied().next_tuple() else {
                panic!("not enough players");
            };

        let mut game = Self {
            chrs,
            acts,

            player_id_manager,
            players_map,

            attacker: attacker_id.into(),
            defender: defender_id.into(),
            subturner: Subturner::Attacker,
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
            let chr_id = self.add_chr(CharacterType::TestCharacter.into());
            self.chrs.add_to_gain_pile(chr_id);
        }

        for _ in 1..=total_acts_count {
            let act_id = self.add_act(ActiveType::TestActive.into());
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

    #[allow(unused)]
    pub fn chr_mut(&mut self, id: CharacterID) -> &mut CharacterInfo {
        self.chrs.get_mut(id)
    }

    #[allow(unused)]
    pub fn act_mut(&mut self, id: ActiveID) -> &mut ActiveInfo {
        self.acts.get_mut(id)
    }

    pub fn add_chr(&mut self, chr: CharacterInfo) -> CharacterID {
        self.chrs.add(chr)
    }

    pub fn add_act(&mut self, act: ActiveInfo) -> ActiveID {
        self.acts.add(act)
    }

    fn add_player_manually(
        player_id_manager: &mut IDManager<PlayerID>,
        players_map: &mut HashMap<PlayerID, Player>,
        chrs: &mut GameOfCardType<CharacterID, CharacterInfo>,
        acts: &mut GameOfCardType<ActiveID, ActiveInfo>,
        player: Player,
    ) -> PlayerID {
        let id = player_id_manager.next_id();

        players_map.insert(id, player);
        chrs.hands.insert(id, Default::default());
        acts.hands.insert(id, Default::default());

        id
    }

    pub fn add_player(&mut self, player: Player) -> PlayerID {
        Self::add_player_manually(
            &mut self.player_id_manager,
            &mut self.players_map,
            &mut self.chrs,
            &mut self.acts,
            player,
        )
    }

    pub fn attacker(&self) -> &PlayerInTurn {
        &self.attacker
    }

    pub fn attacker_mut(&mut self) -> &mut PlayerInTurn {
        &mut self.attacker
    }

    pub fn defender(&self) -> &PlayerInTurn {
        &self.defender
    }

    pub fn defender_mut(&mut self) -> &mut PlayerInTurn {
        &mut self.defender
    }

    pub fn end_subturn(&mut self) {
        self.subturner.switch()
    }

    pub fn subturner(&self) -> &PlayerInTurn {
        match self.subturner {
            Subturner::Attacker => self.attacker(),
            Subturner::Defender => self.defender(),
        }
    }

    #[allow(unused)]
    pub fn subturner_mut(&mut self) -> &mut PlayerInTurn {
        match self.subturner {
            Subturner::Attacker => self.attacker_mut(),
            Subturner::Defender => self.defender_mut(),
        }
    }

    pub fn other_subturner(&self) -> &PlayerInTurn {
        match self.subturner {
            Subturner::Attacker => self.defender(),
            Subturner::Defender => self.attacker(),
        }
    }

    #[allow(unused)]
    pub fn other_subturner_mut(&mut self) -> &mut PlayerInTurn {
        match self.subturner {
            Subturner::Attacker => self.defender_mut(),
            Subturner::Defender => self.attacker_mut(),
        }
    }
}

impl GameState {
    pub fn is_matching_ability<Trigger: TriggerTrait + PartialEq, ID>(
        &self,
        trigger: Trigger,
        ability: &Ability<Trigger, ID>,
    ) -> bool {
        ability.trigger == trigger
            && ability.conditions.iter().all(|condition| self.is_satisfied_condition(condition))
    }

    pub fn find_matching_ability_idx<Trigger: TriggerTrait + Copy + PartialEq, ID>(
        &self,
        trigger: Trigger,
        abilities: &[Ability<Trigger, ID>],
    ) -> Option<usize> {
        for (ability_idx, ability) in abilities.iter().enumerate() {
            if self.is_matching_ability(trigger, ability) {
                return Some(ability_idx);
            }
        }
        None
    }

    pub fn find_matching_ability<'abilities, Trigger: TriggerTrait + Copy + PartialEq, ID>(
        &self,
        trigger: Trigger,
        abilities: &'abilities [Ability<Trigger, ID>],
    ) -> Option<&'abilities Ability<Trigger, ID>> {
        Some(&abilities[self.find_matching_ability_idx(trigger, abilities)?])
    }

    pub fn check_conditions(&self, conditions: &[Condition]) -> bool {
        conditions.iter().all(|condition| self.is_satisfied_condition(condition))
    }

    pub fn is_satisfied_condition(&self, _condition: &Condition) -> bool {
        todo!()
    }
}
