use crate::card_uses::CharacterID;
use crate::game_formatted::GameFormatted;
use std::fmt;
use std::fmt::Display;

pub type Stat0 = i32;

macro_rules! make_stat {
    ($Name:ident, $macro:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $Name(pub Stat0);

        impl Display
            for $crate::game_formatted::GameFormatted<
                '_,
                '_,
                '_,
                $Name,
                $crate::game_state::chr_id::CharacterID,
            >
        {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if self.game.is_const(self.id, StatType::$Name) {
                    write!(f, "{} {}", $crate::cs![$Name], $crate::cs![Const(self.value.0)])
                } else if self.game.is_private(self.id, StatType::$Name) {
                    write!(f, "{} {}", $crate::cs![$Name], $crate::cs![Private(self.value.0)])
                } else {
                    write!(f, "{} {}", $crate::cs![$Name], self.value.0)
                }
            }
        }

        impl std::ops::Deref for $Name {
            type Target = Stat0;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[macro_export]
        macro_rules! $macro {
            ($value:literal) => {
                $crate::stats::$Name($value)
            };

            // TODO remove
            ($value:literal = const) => {
                $crate::stats::$Name($value)
            };

            // TODO remove
            ($value:literal ?) => {
                $crate::stats::$Name($value)
            };
        }
    };
}

make_stat!(Vitality, vit);
make_stat!(Physique, phy);
make_stat!(Defence, def);
make_stat!(Damage, dmg);
make_stat!(Intellect, int);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Stats {
    pub vit: Vitality,
    pub phy: Physique,
    pub def: Defence,
    pub dmg: Damage,
    pub int: Intellect,
}

impl Stats {
    pub fn new(phy: Physique, dmg: Damage, int: Intellect) -> Self {
        let vit = Vitality(phy.0);
        let def = def!(0);

        Self { vit, phy, def, dmg, int }
    }

    pub fn new_def(phy: Physique, def: Defence, dmg: Damage, int: Intellect) -> Self {
        let vit = Vitality(phy.0);

        Self { vit, phy, def, dmg, int }
    }
}

impl Display for GameFormatted<'_, '_, '_, &Stats, CharacterID> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value.vit.0 != self.value.phy.0 {
            write!(f, "{} / ", self.with_value(self.value.vit))?;
        }
        write!(f, "{}", self.with_value(self.value.phy))?;
        if self.value.def != def!(0) {
            write!(f, " + {}", self.with_value(self.value.def))?;
        }
        writeln!(f)?;

        writeln!(f, "{}", self.with_value(self.value.dmg))?;

        write!(f, "{}", self.with_value(self.value.int))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StatType {
    Vitality,
    Physique,
    Defence,
    Damage,
    Intellect,
}

impl Stats {
    pub fn stat(&self, type_: StatType) -> Stat0 {
        match type_ {
            StatType::Vitality => self.vit.0,
            StatType::Physique => self.phy.0,
            StatType::Defence => self.def.0,
            StatType::Damage => self.dmg.0,
            StatType::Intellect => self.int.0,
        }
    }

    pub fn stat_mut(&mut self, type_: StatType) -> &mut Stat0 {
        match type_ {
            StatType::Vitality => &mut self.vit.0,
            StatType::Physique => &mut self.phy.0,
            StatType::Defence => &mut self.def.0,
            StatType::Damage => &mut self.dmg.0,
            StatType::Intellect => &mut self.int.0,
        }
    }
}
