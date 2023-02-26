pub mod active_ability;
pub mod active_trigger;
pub mod character_ability;
pub mod character_trigger;

use std::fmt::Display;

use crate::{
    cs, custom_string::CustomString, default_formatted::DefaultFormatted, gendered::Gendered,
    host::Host, trigger_trait::TriggerTrait,
};

use super::condition::Condition;

pub struct Ability<Trigger: TriggerTrait, CardID> {
    pub name: Option<CustomString>,

    pub trigger: Trigger,
    pub conditions: Vec<Condition>,

    pub description: CustomString,

    pub callback: fn(&mut Host, CardID, Trigger::Went),
}

impl<'condition, Condition: TriggerTrait + 'condition, CardID> Display
    for Gendered<&Ability<Condition, CardID>>
where
    Gendered<&'condition Condition>: Display,
    Self: 'condition,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.value.name {
            writeln!(f, "\x1b[1m{}\x1b[0m", name)?;
        }

        write!(f, "{}", Gendered { ru_gender: self.ru_gender, value: &self.value.trigger })?;
        if !self.value.conditions.is_empty() {
            write!(f, " {} {}", cs![And()], DefaultFormatted(&self.value.conditions))?;
        }
        write!(f, " {}", cs![Implies() "\n" Bullet() " "])?;
        write!(f, "{}", self.value.description)
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
