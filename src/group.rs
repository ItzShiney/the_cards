mod _macro;

use crate::default_formatted::DefaultFormatted;
use crate::group;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::fmt::Display;

group![
    S > "\x1b[34mS\x1b[39m";
    A > "\x1b[31mA\x1b[39m";
    B > "\x1b[33mB\x1b[39m";
    C > "\x1b[32mС\x1b[39m";
    D > "\x1b[36mD\x1b[39m";

    ByЛёня > "\x1b[35m@лёня\x1b[39m";
    ByМаксим > "\x1b[36m@максим\x1b[39m";
    ByКостя > "\x1b[33m@костя\x1b[39m";
    ByЛёша > "\x1b[31m@лёша\x1b[39m";

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

pub type Groups = BTreeSet<Group>;

fn fmt_groups(
    groups: impl Iterator<Item = Group> + Clone,
    f: &mut std::fmt::Formatter,
) -> std::fmt::Result {
    let res = groups.clone().join(" ");
    write!(f, "{}", res)
}

/* fn fmt_groups(
    groups: impl Iterator<Item = Group> + Clone,
    f: &mut std::fmt::Formatter,
) -> std::fmt::Result {
    fn supers(groups: impl Iterator<Item = Group>) -> Groups {
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
} */

impl Display for DefaultFormatted<&BTreeSet<Group>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_groups(self.0.iter().copied(), f)
    }
}
