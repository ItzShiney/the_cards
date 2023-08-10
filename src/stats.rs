use {
    crate::{
        card_uses::CharacterID,
        chr_uses::Signature,
        game_formatted::GameFormatted,
    },
    std::{
        fmt,
        fmt::Display,
    },
};

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
                (
                    $Name,
                    $crate::game_state::chr_id::CharacterID,
                    $crate::game_state::event::Signature,
                ),
            >
        {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let (value, id, signature) = self.value;

                if self.game.is_const(id, StatType::$Name, signature) {
                    write!(
                        f,
                        "{} {}",
                        $crate::cs![$Name],
                        $crate::cs![Const($crate::cs![self.value.0.to_string()])]
                    )
                } else if self.game.is_private(id, StatType::$Name, signature) {
                    write!(
                        f,
                        "{} {}",
                        $crate::cs![$Name],
                        $crate::cs![Private($crate::cs![self.value.0.to_string()])]
                    )
                } else {
                    write!(f, "{} {}", $crate::cs![$Name], value.0)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

        Self {
            vit,
            phy,
            def,
            dmg,
            int,
        }
    }

    pub fn new_def(phy: Physique, def: Defence, dmg: Damage, int: Intellect) -> Self {
        let vit = Vitality(phy.0);

        Self {
            vit,
            phy,
            def,
            dmg,
            int,
        }
    }
}

impl Display for GameFormatted<'_, '_, '_, (Stats, CharacterID, Signature)> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (value, id, signature) = self.value;

        if value.vit.0 != value.phy.0 {
            write!(f, "{} / ", self.with_value((value.vit, id, signature)))?;
        }
        write!(f, "{}", self.with_value((value.phy, id, signature)))?;
        if value.def != def!(0) {
            write!(f, " + {}", self.with_value((value.def, id, signature)))?;
        }
        writeln!(f)?;

        writeln!(f, "{}", self.with_value((value.dmg, id, signature)))?;

        write!(f, "{}", self.with_value((value.int, id, signature)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatType {
    Vitality,
    Physique,
    Defence,
    Damage,
    Intellect,
}

impl Stats {
    pub fn stat(&self, stat_type: StatType) -> Stat0 {
        match stat_type {
            StatType::Vitality => self.vit.0,
            StatType::Physique => self.phy.0,
            StatType::Defence => self.def.0,
            StatType::Damage => self.dmg.0,
            StatType::Intellect => self.int.0,
        }
    }

    pub fn add(&mut self, stat_type: StatType, value: Stat0) {
        self.set(stat_type, self.stat(stat_type) + value)
    }

    pub fn set(&mut self, stat_type: StatType, mut value: Stat0) {
        if value < 0 {
            value = 0;
        }

        match stat_type {
            StatType::Vitality => self.vit.0 = value.min(self.phy.0),
            StatType::Physique => {
                self.phy.0 = value;
                self.vit.0 = self.vit.0.min(self.phy.0);
            }
            StatType::Defence => self.def.0 = value,
            StatType::Damage => self.dmg.0 = value,
            StatType::Intellect => self.int.0 = value,
        }
    }
}
