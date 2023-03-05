use std::collections::BTreeSet;
use std::fmt::Display;

use itertools::Itertools;

use crate::default_formatted::DefaultFormatted;

macro_rules! groups {
    (
        $(
            $Name:ident > $Into:literal $(: [
                $($Super:ident),*
            ])?;
        )*
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Group {
            $($Name,)*
        }

        impl Group {
            pub fn supers(self) -> Vec<Group> {
                match self {
                    $(Self::$Name => vec![$($(Self::$Super),*)?],)*
                }
            }
        }

        impl Display for Group {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "\x1b[1m[{}]\x1b[22m",
                    match self {
                        $(Self::$Name => $Into,)*
                    }
                )
            }
        }
    };
}

groups![
    СделаноЛёней > "\x1b[35m@лёня\x1b[39m";
    СделаноМаксимом > "\x1b[36m@максим\x1b[39m";
    СделаноКостей > "\x1b[33m@костя\x1b[39m";
    СделаноЛёшей > "\x1b[31m@лёша\x1b[39m";

    Скрытая > "\x1b[90mскрытая\x1b[39m";

    Мужчина > "\x1b[36m♂\x1b[39m";
    Женщина > "\x1b[35m♀️\x1b[39m";

    Иллюзия > "иллюзия";
    Моралит > "моралит";
    Дизморалит > "дизморалит";

    Суеверия > "суеверия";
        Таро > "таро": [Суеверия];
        Зодиак > "зодиак": [Суеверия];

    Аниме > "аниме";
        ReZero > "re:zero": [Аниме, Сериалы];
        Monogatari > "monogatari": [Аниме, Сериалы];
        Bocchi > "bocchi": [Аниме, Сериалы];
        NewGame > "new game": [Аниме, Сериалы];
        DeathNote > "death note": [Аниме, Сериалы];

    Программирование > "программирование";
        ЯзыкиПрограммирования > "языки программирования": [Программирование];

    Реальность > "реальность";
        Химия > "химия": [Реальность];

    ОбщественныйСтрой > "общественный строй";

    Музыка > "музыка";
        Pyrokinesis > "pyrokinesis": [Музыка];

    Цитаты > "цитаты";
        ЦитатыКости > "цитаты кости": [Цитаты];
        ЦитатыЧубарова > "цитаты чубарова": [Цитаты];

    Мемы > "мемы";
        Животворит > "животворит": [Мемы]; // убрать надгруппу?
        ПепежноеСущество > "пепежное существо": [Мемы];

    Сериалы > "сериалы";
        SouthPark > "south park": [Сериалы];

    Игры > "игры";
        TBoI > "tboi": [Игры];
        Portal > "portal": [Игры];
        Undertale > "undertale": [Игры];
        Terraria > "terraria": [Игры];
        Minecraft > "minecraft": [Игры];
            DreamSMP > "dream smp": [Minecraft];

        Новеллы > "новеллы": [Аниме, Игры];
            WhenTheyCry > "wtc": [Новеллы];
                Higurashi > "higurashi": [WhenTheyCry];
                Umineko > "umineko": [WhenTheyCry];
            SteinsGate > "steins;gate": [Новеллы];
            SayaNoUta > "saya no uta": [Новеллы];

    Втуберы > "втуберы": [Аниме];
        WePlanet > "weplanet": [Втуберы];
        Hololive > "hololive": [Втуберы];
        Nijisanji > "nijisanji": [Втуберы];

    Нераздаваемая > "\x1b[31mнераздаваемая\x1b[39m";
];

fn fmt_groups(
    groups: impl Iterator<Item = Group> + Clone,
    f: &mut std::fmt::Formatter,
) -> std::fmt::Result {
    fn supers(groups: impl Iterator<Item = Group>) -> BTreeSet<Group> {
        groups.flat_map(|group| group.supers()).collect()
    }

    let res = groups.clone().join(" ");
    write!(f, "{}", res)?;

    let supers = supers(groups);
    if !supers.is_empty() {
        write!(f, "\x1b[90m > {}\x1b[39m", DefaultFormatted(&supers))
    } else {
        Ok(())
    }
}

impl Display for DefaultFormatted<&BTreeSet<Group>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_groups(self.0.iter().copied(), f)
    }
}
