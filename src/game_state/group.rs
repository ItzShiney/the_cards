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
                match self {
                    $(Self::$Name => $Into,)*
                }
                .fmt(f)
            }
        }
    };
}

groups![
    Shiney > "\x1b[34m@лёня\x1b[0m";
    Maxvog > "\x1b[34m@максим\x1b[0m";
    Constantine > "\x1b[34m@костя\x1b[0m";
    ZoinX > "\x1b[34m@лёша\x1b[0m";

    Male > "\x1b[36m♂\x1b[0m";
    Female > "\x1b[35m♀️\x1b[0m";

    Illusion > "иллюзия";
    Anime > "аниме";

    Reality > "реальность";

    Music > "музыка";
        Pyrokinesis > "pyrokinesis": [Music];

    Memes > "мемы";
        Quotes > "цитаты": [Memes];
            ConstantineQuotes > "цитаты кости": [Quotes];
            ChubarovQuotes > "цитаты чубарова": [Quotes];
        Lifemaking > "животворящё": [Memes];

    Shows > "шоу";
        SouthPark > "south park": [Shows];

        ReZero > "re:zero": [Anime, Shows];
        Monogatari > "monogatari": [Anime, Shows];
        BocchiTheRock > "bocchi": [Anime, Shows];
        NewGame > "new game": [Anime, Shows];

    Games > "игры";
        TBoI > "tboi": [Games];
        Portal > "portal": [Games];
        Minecraft > "minecraft": [Games];
            DreamSMP > "dream smp": [Minecraft];

    VisualNovels > "новеллы": [Games, Anime];
        WhenTheyCry > "wtc": [VisualNovels];
            Higurashi > "higurashi": [WhenTheyCry];
            Umineko > "umineko": [WhenTheyCry];
        SteinsGate > "steins;gate": [VisualNovels];
        SayaNoUta > "saya no uta": [VisualNovels];

    VTubers > "втуберы": [Anime];
        WePlanet > "weplanet": [VTubers];
        Hololive > "hololive": [VTubers];
        Nijisanji > "nijisanji": [VTubers];
];

fn fmt_groups(
    groups: impl Iterator<Item = Group> + Clone,
    f: &mut std::fmt::Formatter,
) -> std::fmt::Result {
    fn supers(groups: impl Iterator<Item = Group>) -> BTreeSet<Group> {
        groups.flat_map(|group| group.supers()).collect()
    }

    let res = groups.clone().map(|x| format!("[{x}]")).join(" ");
    write!(f, "{}", res)?;

    let supers = supers(groups);
    if !supers.is_empty() {
        write!(f, "\x1b[90m > {}\x1b[0m", DefaultFormatted(&supers))
    } else {
        Ok(())
    }
}

impl Display for DefaultFormatted<&BTreeSet<Group>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_groups(self.0.iter().copied(), f)
    }
}
