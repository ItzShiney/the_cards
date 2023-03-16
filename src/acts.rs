use lazy_static::lazy_static;
use crate::card_uses::*;

#[path = "acts/cu_oh2.rs"] mod cu_oh2;
#[path = "acts/godhead.rs"] mod godhead;
#[path = "acts/баян.rs"] mod баян;
#[path = "acts/берн.rs"] mod берн;
#[path = "acts/биология.rs"] mod биология;
#[path = "acts/душа.rs"] mod душа;
#[path = "acts/жёлтая_искра.rs"] mod жёлтая_искра;
#[path = "acts/зеркало.rs"] mod зеркало;
#[path = "acts/козерог.rs"] mod козерог;
#[path = "acts/коммунизм.rs"] mod коммунизм;
#[path = "acts/ластик.rs"] mod ластик;
#[path = "acts/лезвие_ножа.rs"] mod лезвие_ножа;
#[path = "acts/мегаовощной_кейти.rs"] mod мегаовощной_кейти;
#[path = "acts/мой_рот_разворот.rs"] mod мой_рот_разворот;
#[path = "acts/монархия.rs"] mod монархия;
#[path = "acts/неутешительный_приз.rs"] mod неутешительный_приз;
#[path = "acts/обратка.rs"] mod обратка;
#[path = "acts/охаги.rs"] mod охаги;
#[path = "acts/пионер_уже_в_коммунизме.rs"] mod пионер_уже_в_коммунизме;
#[path = "acts/пустая_карта.rs"] mod пустая_карта;
#[path = "acts/разум.rs"] mod разум;
#[path = "acts/ручка_ножа.rs"] mod ручка_ножа;
#[path = "acts/сатокина_бита.rs"] mod сатокина_бита;
#[path = "acts/тело.rs"] mod тело;
#[path = "acts/тетрадь_смерти.rs"] mod тетрадь_смерти;
#[path = "acts/тупость.rs"] mod тупость;
#[path = "acts/утешительный_приз.rs"] mod утешительный_приз;
#[path = "acts/хривна.rs"] mod хривна;
#[path = "acts/чёрт480.rs"] mod чёрт480;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ActiveType {
    CuOh2,
    Godhead,
    Баян,
    Берн,
    Биология,
    Душа,
    ЖёлтаяИскра,
    Зеркало,
    Козерог,
    Коммунизм,
    Ластик,
    ЛезвиеНожа,
    МегаовощнойКейти,
    МойРотРазворот,
    Монархия,
    НеутешительныйПриз,
    Обратка,
    Охаги,
    ПионерУжеВКоммунизме,
    ПустаяКарта,
    Разум,
    РучкаНожа,
    СатокинаБита,
    Тело,
    ТетрадьСмерти,
    Тупость,
    УтешительныйПриз,
    Хривна,
    Чёрт480,
}

impl ActiveType {
    pub fn all() -> Vec<Self> {
        vec![
            Self::CuOh2,
            Self::Godhead,
            Self::Баян,
            Self::Берн,
            Self::Биология,
            Self::Душа,
            Self::ЖёлтаяИскра,
            Self::Зеркало,
            Self::Козерог,
            Self::Коммунизм,
            Self::Ластик,
            Self::ЛезвиеНожа,
            Self::МегаовощнойКейти,
            Self::МойРотРазворот,
            Self::Монархия,
            Self::НеутешительныйПриз,
            Self::Обратка,
            Self::Охаги,
            Self::ПионерУжеВКоммунизме,
            Self::ПустаяКарта,
            Self::Разум,
            Self::РучкаНожа,
            Self::СатокинаБита,
            Self::Тело,
            Self::ТетрадьСмерти,
            Self::Тупость,
            Self::УтешительныйПриз,
            Self::Хривна,
            Self::Чёрт480,
        ]
    }

    #[allow(non_upper_case_globals)]
    pub fn name(self) -> &'static CustomString {
        lazy_static! {
            static ref CuOh2: CustomString = cu_oh2::name();
            static ref Godhead: CustomString = godhead::name();
            static ref Баян: CustomString = баян::name();
            static ref Берн: CustomString = берн::name();
            static ref Биология: CustomString = биология::name();
            static ref Душа: CustomString = душа::name();
            static ref ЖёлтаяИскра: CustomString = жёлтая_искра::name();
            static ref Зеркало: CustomString = зеркало::name();
            static ref Козерог: CustomString = козерог::name();
            static ref Коммунизм: CustomString = коммунизм::name();
            static ref Ластик: CustomString = ластик::name();
            static ref ЛезвиеНожа: CustomString = лезвие_ножа::name();
            static ref МегаовощнойКейти: CustomString = мегаовощной_кейти::name();
            static ref МойРотРазворот: CustomString = мой_рот_разворот::name();
            static ref Монархия: CustomString = монархия::name();
            static ref НеутешительныйПриз: CustomString = неутешительный_приз::name();
            static ref Обратка: CustomString = обратка::name();
            static ref Охаги: CustomString = охаги::name();
            static ref ПионерУжеВКоммунизме: CustomString = пионер_уже_в_коммунизме::name();
            static ref ПустаяКарта: CustomString = пустая_карта::name();
            static ref Разум: CustomString = разум::name();
            static ref РучкаНожа: CustomString = ручка_ножа::name();
            static ref СатокинаБита: CustomString = сатокина_бита::name();
            static ref Тело: CustomString = тело::name();
            static ref ТетрадьСмерти: CustomString = тетрадь_смерти::name();
            static ref Тупость: CustomString = тупость::name();
            static ref УтешительныйПриз: CustomString = утешительный_приз::name();
            static ref Хривна: CustomString = хривна::name();
            static ref Чёрт480: CustomString = чёрт480::name();
        };

        match self {
            Self::CuOh2 => &*CuOh2,
            Self::Godhead => &*Godhead,
            Self::Баян => &*Баян,
            Self::Берн => &*Берн,
            Self::Биология => &*Биология,
            Self::Душа => &*Душа,
            Self::ЖёлтаяИскра => &*ЖёлтаяИскра,
            Self::Зеркало => &*Зеркало,
            Self::Козерог => &*Козерог,
            Self::Коммунизм => &*Коммунизм,
            Self::Ластик => &*Ластик,
            Self::ЛезвиеНожа => &*ЛезвиеНожа,
            Self::МегаовощнойКейти => &*МегаовощнойКейти,
            Self::МойРотРазворот => &*МойРотРазворот,
            Self::Монархия => &*Монархия,
            Self::НеутешительныйПриз => &*НеутешительныйПриз,
            Self::Обратка => &*Обратка,
            Self::Охаги => &*Охаги,
            Self::ПионерУжеВКоммунизме => &*ПионерУжеВКоммунизме,
            Self::ПустаяКарта => &*ПустаяКарта,
            Self::Разум => &*Разум,
            Self::РучкаНожа => &*РучкаНожа,
            Self::СатокинаБита => &*СатокинаБита,
            Self::Тело => &*Тело,
            Self::ТетрадьСмерти => &*ТетрадьСмерти,
            Self::Тупость => &*Тупость,
            Self::УтешительныйПриз => &*УтешительныйПриз,
            Self::Хривна => &*Хривна,
            Self::Чёрт480 => &*Чёрт480,
        }
    }

    #[allow(non_upper_case_globals)]
    pub fn groups(self) -> &'static Groups {
        lazy_static! {
            static ref CuOh2: Groups = cu_oh2::groups();
            static ref Godhead: Groups = godhead::groups();
            static ref Баян: Groups = баян::groups();
            static ref Берн: Groups = берн::groups();
            static ref Биология: Groups = биология::groups();
            static ref Душа: Groups = душа::groups();
            static ref ЖёлтаяИскра: Groups = жёлтая_искра::groups();
            static ref Зеркало: Groups = зеркало::groups();
            static ref Козерог: Groups = козерог::groups();
            static ref Коммунизм: Groups = коммунизм::groups();
            static ref Ластик: Groups = ластик::groups();
            static ref ЛезвиеНожа: Groups = лезвие_ножа::groups();
            static ref МегаовощнойКейти: Groups = мегаовощной_кейти::groups();
            static ref МойРотРазворот: Groups = мой_рот_разворот::groups();
            static ref Монархия: Groups = монархия::groups();
            static ref НеутешительныйПриз: Groups = неутешительный_приз::groups();
            static ref Обратка: Groups = обратка::groups();
            static ref Охаги: Groups = охаги::groups();
            static ref ПионерУжеВКоммунизме: Groups = пионер_уже_в_коммунизме::groups();
            static ref ПустаяКарта: Groups = пустая_карта::groups();
            static ref Разум: Groups = разум::groups();
            static ref РучкаНожа: Groups = ручка_ножа::groups();
            static ref СатокинаБита: Groups = сатокина_бита::groups();
            static ref Тело: Groups = тело::groups();
            static ref ТетрадьСмерти: Groups = тетрадь_смерти::groups();
            static ref Тупость: Groups = тупость::groups();
            static ref УтешительныйПриз: Groups = утешительный_приз::groups();
            static ref Хривна: Groups = хривна::groups();
            static ref Чёрт480: Groups = чёрт480::groups();
        };

        match self {
            Self::CuOh2 => &*CuOh2,
            Self::Godhead => &*Godhead,
            Self::Баян => &*Баян,
            Self::Берн => &*Берн,
            Self::Биология => &*Биология,
            Self::Душа => &*Душа,
            Self::ЖёлтаяИскра => &*ЖёлтаяИскра,
            Self::Зеркало => &*Зеркало,
            Self::Козерог => &*Козерог,
            Self::Коммунизм => &*Коммунизм,
            Self::Ластик => &*Ластик,
            Self::ЛезвиеНожа => &*ЛезвиеНожа,
            Self::МегаовощнойКейти => &*МегаовощнойКейти,
            Self::МойРотРазворот => &*МойРотРазворот,
            Self::Монархия => &*Монархия,
            Self::НеутешительныйПриз => &*НеутешительныйПриз,
            Self::Обратка => &*Обратка,
            Self::Охаги => &*Охаги,
            Self::ПионерУжеВКоммунизме => &*ПионерУжеВКоммунизме,
            Self::ПустаяКарта => &*ПустаяКарта,
            Self::Разум => &*Разум,
            Self::РучкаНожа => &*РучкаНожа,
            Self::СатокинаБита => &*СатокинаБита,
            Self::Тело => &*Тело,
            Self::ТетрадьСмерти => &*ТетрадьСмерти,
            Self::Тупость => &*Тупость,
            Self::УтешительныйПриз => &*УтешительныйПриз,
            Self::Хривна => &*Хривна,
            Self::Чёрт480 => &*Чёрт480,
        }
    }

    #[allow(non_upper_case_globals)]
    pub fn description(self) -> &'static CustomString {
        lazy_static! {
            static ref CuOh2: CustomString = cu_oh2::description();
            static ref Godhead: CustomString = godhead::description();
            static ref Баян: CustomString = баян::description();
            static ref Берн: CustomString = берн::description();
            static ref Биология: CustomString = биология::description();
            static ref Душа: CustomString = душа::description();
            static ref ЖёлтаяИскра: CustomString = жёлтая_искра::description();
            static ref Зеркало: CustomString = зеркало::description();
            static ref Козерог: CustomString = козерог::description();
            static ref Коммунизм: CustomString = коммунизм::description();
            static ref Ластик: CustomString = ластик::description();
            static ref ЛезвиеНожа: CustomString = лезвие_ножа::description();
            static ref МегаовощнойКейти: CustomString = мегаовощной_кейти::description();
            static ref МойРотРазворот: CustomString = мой_рот_разворот::description();
            static ref Монархия: CustomString = монархия::description();
            static ref НеутешительныйПриз: CustomString = неутешительный_приз::description();
            static ref Обратка: CustomString = обратка::description();
            static ref Охаги: CustomString = охаги::description();
            static ref ПионерУжеВКоммунизме: CustomString = пионер_уже_в_коммунизме::description();
            static ref ПустаяКарта: CustomString = пустая_карта::description();
            static ref Разум: CustomString = разум::description();
            static ref РучкаНожа: CustomString = ручка_ножа::description();
            static ref СатокинаБита: CustomString = сатокина_бита::description();
            static ref Тело: CustomString = тело::description();
            static ref ТетрадьСмерти: CustomString = тетрадь_смерти::description();
            static ref Тупость: CustomString = тупость::description();
            static ref УтешительныйПриз: CustomString = утешительный_приз::description();
            static ref Хривна: CustomString = хривна::description();
            static ref Чёрт480: CustomString = чёрт480::description();
        };

        match self {
            Self::CuOh2 => &*CuOh2,
            Self::Godhead => &*Godhead,
            Self::Баян => &*Баян,
            Self::Берн => &*Берн,
            Self::Биология => &*Биология,
            Self::Душа => &*Душа,
            Self::ЖёлтаяИскра => &*ЖёлтаяИскра,
            Self::Зеркало => &*Зеркало,
            Self::Козерог => &*Козерог,
            Self::Коммунизм => &*Коммунизм,
            Self::Ластик => &*Ластик,
            Self::ЛезвиеНожа => &*ЛезвиеНожа,
            Self::МегаовощнойКейти => &*МегаовощнойКейти,
            Self::МойРотРазворот => &*МойРотРазворот,
            Self::Монархия => &*Монархия,
            Self::НеутешительныйПриз => &*НеутешительныйПриз,
            Self::Обратка => &*Обратка,
            Self::Охаги => &*Охаги,
            Self::ПионерУжеВКоммунизме => &*ПионерУжеВКоммунизме,
            Self::ПустаяКарта => &*ПустаяКарта,
            Self::Разум => &*Разум,
            Self::РучкаНожа => &*РучкаНожа,
            Self::СатокинаБита => &*СатокинаБита,
            Self::Тело => &*Тело,
            Self::ТетрадьСмерти => &*ТетрадьСмерти,
            Self::Тупость => &*Тупость,
            Self::УтешительныйПриз => &*УтешительныйПриз,
            Self::Хривна => &*Хривна,
            Self::Чёрт480 => &*Чёрт480,
        }
    }

    #[allow(non_upper_case_globals)]
    pub fn abilities(self) -> &'static GameCallbacks {
        lazy_static! {
            static ref CuOh2: GameCallbacks = cu_oh2::abilities();
            static ref Godhead: GameCallbacks = godhead::abilities();
            static ref Баян: GameCallbacks = баян::abilities();
            static ref Берн: GameCallbacks = берн::abilities();
            static ref Биология: GameCallbacks = биология::abilities();
            static ref Душа: GameCallbacks = душа::abilities();
            static ref ЖёлтаяИскра: GameCallbacks = жёлтая_искра::abilities();
            static ref Зеркало: GameCallbacks = зеркало::abilities();
            static ref Козерог: GameCallbacks = козерог::abilities();
            static ref Коммунизм: GameCallbacks = коммунизм::abilities();
            static ref Ластик: GameCallbacks = ластик::abilities();
            static ref ЛезвиеНожа: GameCallbacks = лезвие_ножа::abilities();
            static ref МегаовощнойКейти: GameCallbacks = мегаовощной_кейти::abilities();
            static ref МойРотРазворот: GameCallbacks = мой_рот_разворот::abilities();
            static ref Монархия: GameCallbacks = монархия::abilities();
            static ref НеутешительныйПриз: GameCallbacks = неутешительный_приз::abilities();
            static ref Обратка: GameCallbacks = обратка::abilities();
            static ref Охаги: GameCallbacks = охаги::abilities();
            static ref ПионерУжеВКоммунизме: GameCallbacks = пионер_уже_в_коммунизме::abilities();
            static ref ПустаяКарта: GameCallbacks = пустая_карта::abilities();
            static ref Разум: GameCallbacks = разум::abilities();
            static ref РучкаНожа: GameCallbacks = ручка_ножа::abilities();
            static ref СатокинаБита: GameCallbacks = сатокина_бита::abilities();
            static ref Тело: GameCallbacks = тело::abilities();
            static ref ТетрадьСмерти: GameCallbacks = тетрадь_смерти::abilities();
            static ref Тупость: GameCallbacks = тупость::abilities();
            static ref УтешительныйПриз: GameCallbacks = утешительный_приз::abilities();
            static ref Хривна: GameCallbacks = хривна::abilities();
            static ref Чёрт480: GameCallbacks = чёрт480::abilities();
        };

        match self {
            Self::CuOh2 => &*CuOh2,
            Self::Godhead => &*Godhead,
            Self::Баян => &*Баян,
            Self::Берн => &*Берн,
            Self::Биология => &*Биология,
            Self::Душа => &*Душа,
            Self::ЖёлтаяИскра => &*ЖёлтаяИскра,
            Self::Зеркало => &*Зеркало,
            Self::Козерог => &*Козерог,
            Self::Коммунизм => &*Коммунизм,
            Self::Ластик => &*Ластик,
            Self::ЛезвиеНожа => &*ЛезвиеНожа,
            Self::МегаовощнойКейти => &*МегаовощнойКейти,
            Self::МойРотРазворот => &*МойРотРазворот,
            Self::Монархия => &*Монархия,
            Self::НеутешительныйПриз => &*НеутешительныйПриз,
            Self::Обратка => &*Обратка,
            Self::Охаги => &*Охаги,
            Self::ПионерУжеВКоммунизме => &*ПионерУжеВКоммунизме,
            Self::ПустаяКарта => &*ПустаяКарта,
            Self::Разум => &*Разум,
            Self::РучкаНожа => &*РучкаНожа,
            Self::СатокинаБита => &*СатокинаБита,
            Self::Тело => &*Тело,
            Self::ТетрадьСмерти => &*ТетрадьСмерти,
            Self::Тупость => &*Тупость,
            Self::УтешительныйПриз => &*УтешительныйПриз,
            Self::Хривна => &*Хривна,
            Self::Чёрт480 => &*Чёрт480,
        }
    }

}
