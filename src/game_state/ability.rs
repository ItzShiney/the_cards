pub mod active_ability;
pub mod character_ability;

use std::fmt::Display;

use crate::{
    custom_string::CustomString, game_state_interactable::GameStateInteractable,
    gendered::Gendered, trigger_trait::TriggerTrait,
};

use super::condition::Condition;

pub struct Ability<Trigger: TriggerTrait, CardID> {
    pub name: Option<CustomString>,

    pub trigger: Trigger,
    pub conditions: Vec<Condition>,

    pub description: CustomString,

    pub callback: fn(&mut GameStateInteractable, CardID, Trigger::Went),
}

impl<'condition, Condition: TriggerTrait + 'condition, CardID> Display
    for Gendered<&Ability<Condition, CardID>>
where
    Gendered<&'condition Condition>: Display,
    Self: 'condition,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Gendered { ru_gender: self.ru_gender, value: &self.value.trigger }.fmt(f)?;
        CustomString::from(" {=>}\n{*} ").fmt(f)?;
        self.value.description.fmt(f)
    }
}

impl<'condition, Condition: TriggerTrait + 'condition, CardID> Display
    for Gendered<&Vec<Ability<Condition, CardID>>>
where
    Gendered<&'condition Condition>: Display,
    Self: 'condition,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value.is_empty() {
            return Ok(());
        }

        let mut iter = self.value.iter();

        Gendered { ru_gender: self.ru_gender, value: iter.next().unwrap() }.fmt(f)?;

        for ability in iter {
            "\n\n".fmt(f)?;
            Gendered { ru_gender: self.ru_gender, value: ability }.fmt(f)?;
        }
        Ok(())
    }
}
