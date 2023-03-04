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
    WillChange(Stat0),
    Var(Stat0),
    Const(Stat0),
}

#[macro_export]
macro_rules! stat {
    ($value:literal?) => {
        $crate::stats::StatValue::WillChange($value)
    };

    ($value:literal) => {
        $crate::stats::StatValue::Var($value)
    };

    ($value:literal=const) => {
        $crate::stats::StatValue::Const($value)
    };
}

impl StatValue {
    pub fn into_value(self) -> Stat0 {
        match self {
            StatValue::WillChange(x) | StatValue::Var(x) | StatValue::Const(x) => x,
        }
    }
}

impl Display for StatValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::WillChange(x) => write!(f, "{}?", x),
            Self::Var(x) => write!(f, "{}", x),
            Self::Const(x) => write!(f, "{}=const", x),
        }
    }
}

impl SubAssign<Stat0> for StatValue {
    fn sub_assign(&mut self, rhs: Stat0) {
        match self {
            Self::WillChange(val) => {
                let mut res = Self::Var(*val);
                res -= rhs;
                *self = res;
            }

            Self::Var(val) => *val = (*val - rhs).max(0),

            Self::Const(_) => panic!("operations on const"),
        }
    }
}

impl AddAssign<Stat0> for StatValue {
    fn add_assign(&mut self, rhs: Stat0) {
        match self {
            Self::WillChange(val) => {
                let mut res = Self::Var(*val);
                res += rhs;
                *self = res;
            }

            Self::Var(val) => *val = (*val + rhs).max(0),

            Self::Const(_) => panic!("operations on const"),
        }
    }
}

macro_rules! make_stat {
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

make_stat!(Vitality);
make_stat!(Physique);
make_stat!(Defence);
make_stat!(Damage);
make_stat!(Intellect);

#[macro_export]
macro_rules! vit {
    ($($xs:tt)*) => {
        $crate::stats::Vitality($crate::stat!($($xs)*))
    };
}

#[macro_export]
macro_rules! phy {
    ($($xs:tt)*) => {
        $crate::stats::Physique($crate::stat!($($xs)*))
    };
}

#[macro_export]
macro_rules! def {
    ($($xs:tt)*) => {
        $crate::stats::Defence($crate::stat!($($xs)*))
    };
}

#[macro_export]
macro_rules! dmg {
    ($($xs:tt)*) => {
        $crate::stats::Damage($crate::stat!($($xs)*))
    };
}

#[macro_export]
macro_rules! int {
    ($($xs:tt)*) => {
        $crate::stats::Intellect($crate::stat!($($xs)*))
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

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.vit.0 != self.phy.0 {
            write!(f, "{} / ", self.vit)?;
        }
        write!(f, "{}", self.phy)?;
        if self.def != def!(0) {
            write!(f, " + {}", self.def)?;
        }
        writeln!(f)?;

        writeln!(f, "{}", self.dmg)?;

        write!(f, "{}", self.int)
    }
}

#[derive(Clone, Copy)]
pub enum Stat {
    Vitality,
    Physique,
    Defence,
    Damage,
    Intellect,
}

impl Stats {
    pub fn stat(&self, type_: Stat) -> &StatValue {
        match type_ {
            Stat::Vitality => &self.vit.0,
            Stat::Physique => &self.phy.0,
            Stat::Defence => &self.def.0,
            Stat::Damage => &self.dmg.0,
            Stat::Intellect => &self.int.0,
        }
    }

    pub fn stat_mut(&mut self, type_: Stat) -> &mut StatValue {
        match type_ {
            Stat::Vitality => &mut self.vit.0,
            Stat::Physique => &mut self.phy.0,
            Stat::Defence => &mut self.def.0,
            Stat::Damage => &mut self.dmg.0,
            Stat::Intellect => &mut self.int.0,
        }
    }
}
