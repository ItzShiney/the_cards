use std::{
    fmt::{self, Display, Formatter},
    ops::{AddAssign, SubAssign},
};

use macros::EnumAs;

pub type Stat0 = i32;

// TODO:
// Убрать Unknown, реализовать как `may_init_change: bool` в Stats (?)
// Статы обязаны быть инициализированными, а may_init_change будет сигнализировать о том, что стат скорее всего будет изменён. Отображается как VIT 5?
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumAs)]
pub enum StatValue {
    Unknown,
    Var(Stat0),
    Const(Stat0),
}

impl StatValue {
    pub fn into_value(self) -> Option<Stat0> {
        match self {
            StatValue::Unknown => None,
            StatValue::Var(x) | StatValue::Const(x) => Some(x),
        }
    }
}

impl Display for StatValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Unknown => "?".fmt(f),

            Self::Var(x) => x.fmt(f),

            Self::Const(x) => {
                x.fmt(f)?;
                "=const".fmt(f)
            }
        }
    }
}

impl SubAssign<Stat0> for StatValue {
    fn sub_assign(&mut self, rhs: Stat0) {
        match self {
            Self::Unknown => panic!("operations on ?"),
            Self::Var(val) => *val = (*val - rhs).max(0),
            Self::Const(_) => panic!("operations on const"),
        }
    }
}

impl AddAssign<Stat0> for StatValue {
    fn add_assign(&mut self, rhs: Stat0) {
        match self {
            Self::Unknown => panic!("operations on ?"),
            Self::Var(val) => *val = (*val - rhs).max(0),
            Self::Const(_) => panic!("operations on const"),
        }
    }
}

macro_rules! stat {
    ($Name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $Name(pub StatValue);

        impl Display for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{} {}", $crate::cs![$Name], self.0)
            }
        }
    };
}

stat!(Vitality);
stat!(Physique);
stat!(Defence);
stat!(Damage);
stat!(Intellect);

#[macro_export]
macro_rules! vit {
    (?) => {
        $crate::stats::Vitality(StatValue::Unknown)
    };

    (const $value:expr) => {
        $crate::stats::Vitality(StatValue::Const($value))
    };

    ($value:expr) => {
        $crate::stats::Vitality(StatValue::Var($value))
    };
}

#[macro_export]
macro_rules! phy {
    (?) => {
        $crate::stats::Physique(crate::stats::StatValue::Unknown)
    };

    (const $value:expr) => {
        $crate::stats::Physique(crate::stats::StatValue::Const($value))
    };

    ($value:expr) => {
        $crate::stats::Physique(crate::stats::StatValue::Var($value))
    };
}

#[macro_export]
macro_rules! def {
    (?) => {
        $crate::stats::Defence(crate::stats::StatValue::Unknown)
    };

    (const $value:expr) => {
        $crate::stats::Defence(crate::stats::StatValue::Const($value))
    };

    ($value:expr) => {
        $crate::stats::Defence(crate::stats::StatValue::Var($value))
    };
}

#[macro_export]
macro_rules! dmg {
    (?) => {
        $crate::stats::Damage(crate::stats::StatValue::Unknown)
    };

    (const $value:expr) => {
        $crate::stats::Damage(crate::stats::StatValue::Const($value))
    };

    ($value:expr) => {
        $crate::stats::Damage(crate::stats::StatValue::Var($value))
    };
}

#[macro_export]
macro_rules! int {
    (?) => {
        $crate::stats::Intellect(crate::stats::StatValue::Unknown)
    };

    (const $value:expr) => {
        $crate::stats::Intellect(crate::stats::StatValue::Const($value))
    };

    ($value:expr) => {
        $crate::stats::Intellect(crate::stats::StatValue::Var($value))
    };
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Stats {
    pub vit: Vitality,
    pub phy: Physique,
    pub def: Defence,
    pub dmg: Damage,
    pub int: Intellect,
}

impl Stats {
    pub const UNINIT: Stats =
        Stats { vit: vit!(?), phy: phy!(?), def: def!(?), dmg: dmg!(?), int: int!(?) };

    pub fn new(phy: Physique, dmg: Damage, int: Intellect) -> Self {
        let vit = match phy.0 {
            StatValue::Unknown => Vitality(StatValue::Unknown),
            StatValue::Var(x) | StatValue::Const(x) => Vitality(StatValue::Var(x)),
        };

        let def = def!(0);

        Self { vit, phy, def, dmg, int }
    }

    pub fn new_def(phy: Physique, def: Defence, dmg: Damage, int: Intellect) -> Self {
        let vit = match phy.0 {
            StatValue::Unknown => Vitality(StatValue::Unknown),
            StatValue::Var(x) | StatValue::Const(x) => Vitality(StatValue::Var(x)),
        };

        Self { vit, phy, def, dmg, int }
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.vit)?;
        if self.vit.0 != self.phy.0 {
            write!(f, " / {}", self.phy)?;
        }
        if self.def != def!(0) {
            write!(f, " + {}", self.def)?;
        }
        writeln!(f)?;

        writeln!(f, "{}", self.dmg)?;

        write!(f, "{}", self.int)
    }
}
