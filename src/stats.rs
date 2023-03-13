use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};
use std::ops::AddAssign;
use std::ops::SubAssign;

pub type Stat0 = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatValue {
    WillChange(Stat0),
    Var(Stat0),
    Const(Stat0),
}

impl StatValue {
    pub fn into_value(self) -> Stat0 {
        match self {
            StatValue::WillChange(x) | StatValue::Var(x) | StatValue::Const(x) => x,
        }
    }

    pub fn set(&mut self, value: Stat0) {
        match self {
            StatValue::WillChange(_) => *self = StatValue::Var(value),
            StatValue::Var(_) => *self = StatValue::Var(value),
            StatValue::Const(_) => panic!("set const stat"),
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

////////////////////////////////////////////////////////////

macro_rules! make_stat {
    ($Name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $Name(pub StatValue);

        impl Display for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{} {}", $crate::cs![$Name], self.0)
            }
        }

        impl ::std::ops::Deref for $Name {
            type Target = StatValue;

            fn deref(&self) -> &Self::Target {
                &self.0
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

    pub fn max_vit(&mut self) {
        self.vit.0.set(self.phy.0.into_value());
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StatType {
    Vitality,
    Physique,
    Defence,
    Damage,
    Intellect,
}

impl Stats {
    pub fn stat(&self, type_: StatType) -> &StatValue {
        match type_ {
            StatType::Vitality => &self.vit.0,
            StatType::Physique => &self.phy.0,
            StatType::Defence => &self.def.0,
            StatType::Damage => &self.dmg.0,
            StatType::Intellect => &self.int.0,
        }
    }

    pub fn stat_mut(&mut self, type_: StatType) -> &mut StatValue {
        match type_ {
            StatType::Vitality => &mut self.vit.0,
            StatType::Physique => &mut self.phy.0,
            StatType::Defence => &mut self.def.0,
            StatType::Damage => &mut self.dmg.0,
            StatType::Intellect => &mut self.int.0,
        }
    }
}
