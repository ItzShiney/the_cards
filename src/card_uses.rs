pub use crate::*;
pub use acts::*;
pub use custom_string::*;
pub use game::*;
pub use game_input::*;
pub use game_state::*;
pub use group::*;
pub use itertools::Itertools;
pub use stats::*;
pub use std::iter::repeat_with;
pub use std::ops::ControlFlow::*;
pub use ActiveType::*;
pub use CharacterType::*;
pub use Group::*;

pub struct GroupsBuilder<Genders: IntoIterator<Item = Group>, Tags: IntoIterator<Item = Group>> {
    pub tier: Group,
    pub author: Group,
    pub genders: Genders,
    pub tags: Tags,
}

impl<Genders: IntoIterator<Item = Group>, Tags: IntoIterator<Item = Group>>
    From<GroupsBuilder<Genders, Tags>> for Groups
{
    fn from(GroupsBuilder { tier, author, genders, tags }: GroupsBuilder<Genders, Tags>) -> Self {
        let mut res = Groups::from([tier, author]);
        res.extend(genders);
        res.extend(tags);
        res
    }
}

#[allow(unused)]
pub fn description() -> CustomString {
    cs![]
}

#[allow(unused)]
pub fn abilities() -> GameCallbacks {
    GameCallbacks::default()
}

pub trait ElseSome<T> {
    fn else_some(self, t: T) -> Option<T>;
}

impl<T> ElseSome<T> for bool {
    fn else_some(self, t: T) -> Option<T> {
        (!self).then_some(t)
    }
}
