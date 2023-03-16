use lazy_static::lazy_static;
use crate::card_uses::*;

#[path = "chrs/delirium.rs"] mod delirium;
#[path = "chrs/банка_с_вареньем.rs"] mod банка_с_вареньем;
#[path = "chrs/беатриче.rs"] mod беатриче;
#[path = "chrs/борат.rs"] mod борат;
#[path = "chrs/гв.rs"] mod гв;
#[path = "chrs/глаз_ктулху.rs"] mod глаз_ктулху;
#[path = "chrs/дух_твоей_квартиры.rs"] mod дух_твоей_квартиры;
#[path = "chrs/коса.rs"] mod коса;
#[path = "chrs/магдалина.rs"] mod магдалина;
#[path = "chrs/максимов_баян_животворящий.rs"] mod максимов_баян_животворящий;
#[path = "chrs/марио.rs"] mod марио;
#[path = "chrs/мирослав.rs"] mod мирослав;
#[path = "chrs/ненети.rs"] mod ненети;
#[path = "chrs/нож.rs"] mod нож;
#[path = "chrs/ностальгирующий_критик.rs"] mod ностальгирующий_критик;
#[path = "chrs/питон.rs"] mod питон;
#[path = "chrs/планя.rs"] mod планя;
#[path = "chrs/рей.rs"] mod рей;
#[path = "chrs/рена.rs"] mod рена;
#[path = "chrs/рика.rs"] mod рика;
#[path = "chrs/робеспьер.rs"] mod робеспьер;
#[path = "chrs/сатока.rs"] mod сатока;
#[path = "chrs/тимми.rs"] mod тимми;
#[path = "chrs/чёрный_кубик.rs"] mod чёрный_кубик;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CharacterType {
    Delirium,
    БанкаСВареньем,
    Беатриче,
    Борат,
    Гв,
    ГлазКтулху,
    ДухТвоейКвартиры,
    Коса,
    Магдалина,
    МаксимовБаянЖивотворящий,
    Марио,
    Мирослав,
    Ненети,
    Нож,
    НостальгирующийКритик,
    Питон,
    Планя,
    Рей,
    Рена,
    Рика,
    Робеспьер,
    Сатока,
    Тимми,
    ЧёрныйКубик,
}

impl CharacterType {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Delirium,
            Self::БанкаСВареньем,
            Self::Беатриче,
            Self::Борат,
            Self::Гв,
            Self::ГлазКтулху,
            Self::ДухТвоейКвартиры,
            Self::Коса,
            Self::Магдалина,
            Self::МаксимовБаянЖивотворящий,
            Self::Марио,
            Self::Мирослав,
            Self::Ненети,
            Self::Нож,
            Self::НостальгирующийКритик,
            Self::Питон,
            Self::Планя,
            Self::Рей,
            Self::Рена,
            Self::Рика,
            Self::Робеспьер,
            Self::Сатока,
            Self::Тимми,
            Self::ЧёрныйКубик,
        ]
    }

    #[allow(non_upper_case_globals)]
    pub fn name(self) -> &'static CustomString {
        lazy_static! {
            static ref Delirium: CustomString = delirium::name();
            static ref БанкаСВареньем: CustomString = банка_с_вареньем::name();
            static ref Беатриче: CustomString = беатриче::name();
            static ref Борат: CustomString = борат::name();
            static ref Гв: CustomString = гв::name();
            static ref ГлазКтулху: CustomString = глаз_ктулху::name();
            static ref ДухТвоейКвартиры: CustomString = дух_твоей_квартиры::name();
            static ref Коса: CustomString = коса::name();
            static ref Магдалина: CustomString = магдалина::name();
            static ref МаксимовБаянЖивотворящий: CustomString = максимов_баян_животворящий::name();
            static ref Марио: CustomString = марио::name();
            static ref Мирослав: CustomString = мирослав::name();
            static ref Ненети: CustomString = ненети::name();
            static ref Нож: CustomString = нож::name();
            static ref НостальгирующийКритик: CustomString = ностальгирующий_критик::name();
            static ref Питон: CustomString = питон::name();
            static ref Планя: CustomString = планя::name();
            static ref Рей: CustomString = рей::name();
            static ref Рена: CustomString = рена::name();
            static ref Рика: CustomString = рика::name();
            static ref Робеспьер: CustomString = робеспьер::name();
            static ref Сатока: CustomString = сатока::name();
            static ref Тимми: CustomString = тимми::name();
            static ref ЧёрныйКубик: CustomString = чёрный_кубик::name();
        };

        match self {
            Self::Delirium => &*Delirium,
            Self::БанкаСВареньем => &*БанкаСВареньем,
            Self::Беатриче => &*Беатриче,
            Self::Борат => &*Борат,
            Self::Гв => &*Гв,
            Self::ГлазКтулху => &*ГлазКтулху,
            Self::ДухТвоейКвартиры => &*ДухТвоейКвартиры,
            Self::Коса => &*Коса,
            Self::Магдалина => &*Магдалина,
            Self::МаксимовБаянЖивотворящий => &*МаксимовБаянЖивотворящий,
            Self::Марио => &*Марио,
            Self::Мирослав => &*Мирослав,
            Self::Ненети => &*Ненети,
            Self::Нож => &*Нож,
            Self::НостальгирующийКритик => &*НостальгирующийКритик,
            Self::Питон => &*Питон,
            Self::Планя => &*Планя,
            Self::Рей => &*Рей,
            Self::Рена => &*Рена,
            Self::Рика => &*Рика,
            Self::Робеспьер => &*Робеспьер,
            Self::Сатока => &*Сатока,
            Self::Тимми => &*Тимми,
            Self::ЧёрныйКубик => &*ЧёрныйКубик,
        }
    }

    #[allow(non_upper_case_globals)]
    pub fn groups(self) -> &'static Groups {
        lazy_static! {
            static ref Delirium: Groups = delirium::groups();
            static ref БанкаСВареньем: Groups = банка_с_вареньем::groups();
            static ref Беатриче: Groups = беатриче::groups();
            static ref Борат: Groups = борат::groups();
            static ref Гв: Groups = гв::groups();
            static ref ГлазКтулху: Groups = глаз_ктулху::groups();
            static ref ДухТвоейКвартиры: Groups = дух_твоей_квартиры::groups();
            static ref Коса: Groups = коса::groups();
            static ref Магдалина: Groups = магдалина::groups();
            static ref МаксимовБаянЖивотворящий: Groups = максимов_баян_животворящий::groups();
            static ref Марио: Groups = марио::groups();
            static ref Мирослав: Groups = мирослав::groups();
            static ref Ненети: Groups = ненети::groups();
            static ref Нож: Groups = нож::groups();
            static ref НостальгирующийКритик: Groups = ностальгирующий_критик::groups();
            static ref Питон: Groups = питон::groups();
            static ref Планя: Groups = планя::groups();
            static ref Рей: Groups = рей::groups();
            static ref Рена: Groups = рена::groups();
            static ref Рика: Groups = рика::groups();
            static ref Робеспьер: Groups = робеспьер::groups();
            static ref Сатока: Groups = сатока::groups();
            static ref Тимми: Groups = тимми::groups();
            static ref ЧёрныйКубик: Groups = чёрный_кубик::groups();
        };

        match self {
            Self::Delirium => &*Delirium,
            Self::БанкаСВареньем => &*БанкаСВареньем,
            Self::Беатриче => &*Беатриче,
            Self::Борат => &*Борат,
            Self::Гв => &*Гв,
            Self::ГлазКтулху => &*ГлазКтулху,
            Self::ДухТвоейКвартиры => &*ДухТвоейКвартиры,
            Self::Коса => &*Коса,
            Self::Магдалина => &*Магдалина,
            Self::МаксимовБаянЖивотворящий => &*МаксимовБаянЖивотворящий,
            Self::Марио => &*Марио,
            Self::Мирослав => &*Мирослав,
            Self::Ненети => &*Ненети,
            Self::Нож => &*Нож,
            Self::НостальгирующийКритик => &*НостальгирующийКритик,
            Self::Питон => &*Питон,
            Self::Планя => &*Планя,
            Self::Рей => &*Рей,
            Self::Рена => &*Рена,
            Self::Рика => &*Рика,
            Self::Робеспьер => &*Робеспьер,
            Self::Сатока => &*Сатока,
            Self::Тимми => &*Тимми,
            Self::ЧёрныйКубик => &*ЧёрныйКубик,
        }
    }

    #[allow(non_upper_case_globals)]
    pub fn description(self) -> &'static CustomString {
        lazy_static! {
            static ref Delirium: CustomString = delirium::description();
            static ref БанкаСВареньем: CustomString = банка_с_вареньем::description();
            static ref Беатриче: CustomString = беатриче::description();
            static ref Борат: CustomString = борат::description();
            static ref Гв: CustomString = гв::description();
            static ref ГлазКтулху: CustomString = глаз_ктулху::description();
            static ref ДухТвоейКвартиры: CustomString = дух_твоей_квартиры::description();
            static ref Коса: CustomString = коса::description();
            static ref Магдалина: CustomString = магдалина::description();
            static ref МаксимовБаянЖивотворящий: CustomString = максимов_баян_животворящий::description();
            static ref Марио: CustomString = марио::description();
            static ref Мирослав: CustomString = мирослав::description();
            static ref Ненети: CustomString = ненети::description();
            static ref Нож: CustomString = нож::description();
            static ref НостальгирующийКритик: CustomString = ностальгирующий_критик::description();
            static ref Питон: CustomString = питон::description();
            static ref Планя: CustomString = планя::description();
            static ref Рей: CustomString = рей::description();
            static ref Рена: CustomString = рена::description();
            static ref Рика: CustomString = рика::description();
            static ref Робеспьер: CustomString = робеспьер::description();
            static ref Сатока: CustomString = сатока::description();
            static ref Тимми: CustomString = тимми::description();
            static ref ЧёрныйКубик: CustomString = чёрный_кубик::description();
        };

        match self {
            Self::Delirium => &*Delirium,
            Self::БанкаСВареньем => &*БанкаСВареньем,
            Self::Беатриче => &*Беатриче,
            Self::Борат => &*Борат,
            Self::Гв => &*Гв,
            Self::ГлазКтулху => &*ГлазКтулху,
            Self::ДухТвоейКвартиры => &*ДухТвоейКвартиры,
            Self::Коса => &*Коса,
            Self::Магдалина => &*Магдалина,
            Self::МаксимовБаянЖивотворящий => &*МаксимовБаянЖивотворящий,
            Self::Марио => &*Марио,
            Self::Мирослав => &*Мирослав,
            Self::Ненети => &*Ненети,
            Self::Нож => &*Нож,
            Self::НостальгирующийКритик => &*НостальгирующийКритик,
            Self::Питон => &*Питон,
            Self::Планя => &*Планя,
            Self::Рей => &*Рей,
            Self::Рена => &*Рена,
            Self::Рика => &*Рика,
            Self::Робеспьер => &*Робеспьер,
            Self::Сатока => &*Сатока,
            Self::Тимми => &*Тимми,
            Self::ЧёрныйКубик => &*ЧёрныйКубик,
        }
    }

    #[allow(non_upper_case_globals)]
    pub fn abilities(self) -> &'static GameCallbacks {
        lazy_static! {
            static ref Delirium: GameCallbacks = delirium::abilities();
            static ref БанкаСВареньем: GameCallbacks = банка_с_вареньем::abilities();
            static ref Беатриче: GameCallbacks = беатриче::abilities();
            static ref Борат: GameCallbacks = борат::abilities();
            static ref Гв: GameCallbacks = гв::abilities();
            static ref ГлазКтулху: GameCallbacks = глаз_ктулху::abilities();
            static ref ДухТвоейКвартиры: GameCallbacks = дух_твоей_квартиры::abilities();
            static ref Коса: GameCallbacks = коса::abilities();
            static ref Магдалина: GameCallbacks = магдалина::abilities();
            static ref МаксимовБаянЖивотворящий: GameCallbacks = максимов_баян_животворящий::abilities();
            static ref Марио: GameCallbacks = марио::abilities();
            static ref Мирослав: GameCallbacks = мирослав::abilities();
            static ref Ненети: GameCallbacks = ненети::abilities();
            static ref Нож: GameCallbacks = нож::abilities();
            static ref НостальгирующийКритик: GameCallbacks = ностальгирующий_критик::abilities();
            static ref Питон: GameCallbacks = питон::abilities();
            static ref Планя: GameCallbacks = планя::abilities();
            static ref Рей: GameCallbacks = рей::abilities();
            static ref Рена: GameCallbacks = рена::abilities();
            static ref Рика: GameCallbacks = рика::abilities();
            static ref Робеспьер: GameCallbacks = робеспьер::abilities();
            static ref Сатока: GameCallbacks = сатока::abilities();
            static ref Тимми: GameCallbacks = тимми::abilities();
            static ref ЧёрныйКубик: GameCallbacks = чёрный_кубик::abilities();
        };

        match self {
            Self::Delirium => &*Delirium,
            Self::БанкаСВареньем => &*БанкаСВареньем,
            Self::Беатриче => &*Беатриче,
            Self::Борат => &*Борат,
            Self::Гв => &*Гв,
            Self::ГлазКтулху => &*ГлазКтулху,
            Self::ДухТвоейКвартиры => &*ДухТвоейКвартиры,
            Self::Коса => &*Коса,
            Self::Магдалина => &*Магдалина,
            Self::МаксимовБаянЖивотворящий => &*МаксимовБаянЖивотворящий,
            Self::Марио => &*Марио,
            Self::Мирослав => &*Мирослав,
            Self::Ненети => &*Ненети,
            Self::Нож => &*Нож,
            Self::НостальгирующийКритик => &*НостальгирующийКритик,
            Self::Питон => &*Питон,
            Self::Планя => &*Планя,
            Self::Рей => &*Рей,
            Self::Рена => &*Рена,
            Self::Рика => &*Рика,
            Self::Робеспьер => &*Робеспьер,
            Self::Сатока => &*Сатока,
            Self::Тимми => &*Тимми,
            Self::ЧёрныйКубик => &*ЧёрныйКубик,
        }
    }

    pub fn stats(self) -> Stats {
        match self {
            Self::Delirium => delirium::stats(),
            Self::БанкаСВареньем => банка_с_вареньем::stats(),
            Self::Беатриче => беатриче::stats(),
            Self::Борат => борат::stats(),
            Self::Гв => гв::stats(),
            Self::ГлазКтулху => глаз_ктулху::stats(),
            Self::ДухТвоейКвартиры => дух_твоей_квартиры::stats(),
            Self::Коса => коса::stats(),
            Self::Магдалина => магдалина::stats(),
            Self::МаксимовБаянЖивотворящий => максимов_баян_животворящий::stats(),
            Self::Марио => марио::stats(),
            Self::Мирослав => мирослав::stats(),
            Self::Ненети => ненети::stats(),
            Self::Нож => нож::stats(),
            Self::НостальгирующийКритик => ностальгирующий_критик::stats(),
            Self::Питон => питон::stats(),
            Self::Планя => планя::stats(),
            Self::Рей => рей::stats(),
            Self::Рена => рена::stats(),
            Self::Рика => рика::stats(),
            Self::Робеспьер => робеспьер::stats(),
            Self::Сатока => сатока::stats(),
            Self::Тимми => тимми::stats(),
            Self::ЧёрныйКубик => чёрный_кубик::stats(),
        }
    }

}
