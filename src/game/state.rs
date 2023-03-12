pub mod act_id;
pub mod act_info;
pub mod chr_id;
pub mod chr_info;
pub mod id_manager;
pub mod player_id;
pub mod player_id_manager;

use self::act_id::ActiveID;
use self::act_info::ActiveInfo;
use self::chr_id::CharacterID;
use self::chr_info::CharacterInfo;
use self::id_manager::id_trait::IDTrait;
use self::id_manager::IDManager;
use self::player_id::PlayerID;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::Debug;

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

    drawpile: Vec<ID>,
    wastepile: Vec<ID>,

    hands: HashMap<PlayerID, Vec<ID>>,
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

    pub fn hand(&self, player_id: PlayerID) -> &Vec<ID> {
        self.hands.get(&player_id).unwrap()
    }

    // FIXME: remove pub
    pub fn hand_mut(&mut self, player_id: PlayerID) -> &mut Vec<ID> {
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
        self.hand_mut(player_id).push(id);
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
            .expect(format!("expected some player to own {:?}", id).as_str());
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

    pub players_map: BTreeMap<PlayerID, Player>,

    pub attacker: SubturnerOnField,
    pub defender: SubturnerOnField,
    pub current_subturner: Subturner,
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

        Self {
            chrs,
            acts,

            players_map,

            attacker: SubturnerOnField::new(attacker_id),
            defender: SubturnerOnField::new(defender_id),
            current_subturner: Subturner::Attacker,
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
        self.chr(chr_id).stats.vit.0.into_value() == 0
    }

    // TODO поделить на функции?
    pub fn enemy_chr_id(&self, chr_id: CharacterID) -> Option<CharacterID> {
        let owner_id = self.find_owner_chr(chr_id);
        let subturner = self.subturner_by_id(owner_id);
        let other_subturner = subturner.other();
        self.subturner_on_field(other_subturner).chr_id
    }
}

impl GameState {
    pub fn try_find_owner_on_field_chr(&self, chr_id: CharacterID) -> Option<PlayerID> {
        if self.attacker.chr_id == Some(chr_id) {
            return Some(self.attacker.player_id);
        }

        if self.defender.chr_id == Some(chr_id) {
            return Some(self.defender.player_id);
        }

        None
    }

    pub fn find_owner_on_field_chr(&self, chr_id: CharacterID) -> PlayerID {
        self.try_find_owner_on_field_chr(chr_id).unwrap()
    }

    pub fn try_find_owner_chr(&self, chr_id: CharacterID) -> Option<PlayerID> {
        if let Some(owner_id) = self.try_find_owner_on_field_chr(chr_id) {
            return Some(owner_id);
        }

        self.chrs.try_find_owner_in_decks(chr_id)
    }

    pub fn find_owner_chr(&self, chr_id: CharacterID) -> PlayerID {
        self.try_find_owner_chr(chr_id).unwrap()
    }
}

impl GameState {
    pub fn try_find_owner_on_field_act(&self, act_id: ActiveID) -> Option<PlayerID> {
        if self.attacker.used_act_ids.contains(&act_id) {
            return Some(self.attacker.player_id);
        }

        if self.defender.used_act_ids.contains(&act_id) {
            return Some(self.defender.player_id);
        }

        None
    }

    pub fn find_owner_on_field_act(&self, act_id: ActiveID) -> PlayerID {
        self.try_find_owner_on_field_act(act_id).unwrap()
    }

    pub fn try_find_owner_act(&self, act_id: ActiveID) -> Option<PlayerID> {
        if let Some(owner_id) = self.try_find_owner_on_field_act(act_id) {
            return Some(owner_id);
        }

        self.acts.try_find_owner_in_decks(act_id)
    }

    pub fn find_owner_act(&self, act_id: ActiveID) -> PlayerID {
        self.try_find_owner_act(act_id).unwrap()
    }
}

impl GameState {
    pub fn is_placeable(&self, _chr_id: CharacterID) -> bool {
        true // TODO
    }

    pub fn is_usable_in_any_way(&self, act_id: ActiveID) -> bool {
        self.is_usable_on_chr(act_id) || self.is_usable_on_field(act_id)
    }

    pub fn is_usable_on_field(&self, act_id: ActiveID) -> bool {
        self.act(act_id).type_.abilities().use_on_field.is_some()
    }

    pub fn is_usable_on_chr(&self, act_id: ActiveID) -> bool {
        self.act(act_id).type_.abilities().use_on_chr.is_some()
    }
}
