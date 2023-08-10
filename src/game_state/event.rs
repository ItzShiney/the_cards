use {
    super::{
        act_id::ActiveID,
        chr_id::CharacterID,
        player_id::PlayerID,
    },
    crate::stats::{
        Stat0,
        StatType,
    },
};

#[derive(Debug, Clone, Copy)]
pub enum StatChange {
    Add(Stat0),
    Mul(Stat0),
    Div(Stat0),
    Set(Stat0),
}

#[derive(Debug, Clone, Copy)]
pub enum UseWay {
    OnField,
    OnCharacter(CharacterID),
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Use {
        act_id: ActiveID,
        use_way: UseWay,
    },
    StatChange {
        chr_id: CharacterID,
        stat_type: StatType,
        stat_change: StatChange,

        old_value: Option<Stat0>,
        old_vit_value: Option<Stat0>,
    },
    Attack {
        attacker_id: CharacterID,
        target_id: CharacterID,
        dmg: Stat0,
    },
    GetHurt {
        chr_id: CharacterID,
        dmg: Stat0,
    },
    TakeCharacter {
        player_id: PlayerID,

        chr_id: Option<CharacterID>,
    },
    TakeActive {
        player_id: PlayerID,

        act_id: Option<ActiveID>,
    },
    PutCharacterInDrawpile {
        chr_id: CharacterID,
    },
    PutActiveInDrawpile {
        act_id: ActiveID,
    },
    Place {
        chr_id: CharacterID,
    },
    Die {
        chr_id: CharacterID,
    },
    EndTurn,
    Replace {
        replaced_chr_id: CharacterID,
        replacing_chr_id: CharacterID,
    },
    HealOnFieldLeave {
        chr_id: CharacterID,
        heal_value: Stat0,
    },
    Random {
        min: Stat0,
        max: Stat0,

        output: Option<Stat0>,
    },
    RandomBool {
        true_p: f64,

        output: Option<bool>,
    },
}

impl Event {
    pub fn stat_change(chr_id: CharacterID, stat_type: StatType, stat_change: StatChange) -> Self {
        Self::StatChange {
            chr_id,
            stat_type,
            stat_change,

            old_value: None,
            old_vit_value: None,
        }
    }

    pub fn take_chr(player_id: PlayerID) -> Self {
        Self::TakeCharacter {
            player_id,

            chr_id: None,
        }
    }

    pub fn take_act(player_id: PlayerID) -> Self {
        Self::TakeActive {
            player_id,

            act_id: None,
        }
    }

    pub fn random(min: Stat0, max: Stat0) -> Self {
        Self::Random {
            min,
            max,

            output: None,
        }
    }

    pub fn random_bool(true_p: f64) -> Self {
        Self::RandomBool {
            true_p,

            output: None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Check {
    Stat {
        chr_id: CharacterID,
        stat_type: StatType,
        value: Stat0,
    },
    AssumeNonPrivate {
        chr_id: CharacterID,
        stat_type: StatType,
    },
    AssumeNonConst {
        chr_id: CharacterID,
        stat_type: StatType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signature {
    Character(CharacterID),
    Active(ActiveID),
    Player(PlayerID),
}

impl From<CharacterID> for Signature {
    fn from(id: CharacterID) -> Self {
        Self::Character(id)
    }
}

impl From<ActiveID> for Signature {
    fn from(id: ActiveID) -> Self {
        Self::Active(id)
    }
}

impl From<PlayerID> for Signature {
    fn from(id: PlayerID) -> Self {
        Self::Player(id)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Signed<T> {
    pub signature: Signature,
    pub value: T,
}

pub type SignedEvent = Signed<Event>;
pub type SignedCheck = Signed<Check>;

pub trait Sign: Sized {
    fn sign(self, signature: impl Into<Signature>) -> Signed<Self>;
}

impl<T> Sign for T {
    fn sign(self, signature: impl Into<Signature>) -> Signed<Self> {
        Signed {
            signature: signature.into(),
            value: self,
        }
    }
}
