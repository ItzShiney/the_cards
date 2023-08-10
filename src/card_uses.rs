pub use {
    crate::*,
    act_info::*,
    acts::*,
    chr_info::*,
    chrs::*,
    custom_string::*,
    event::*,
    game::*,
    game_input::*,
    game_state::*,
    group::*,
    itertools::Itertools,
    stats::*,
    std::{
        iter::repeat_with,
        ops::ControlFlow::*,
    },
    ActiveType::*,
    CharacterType::*,
    Group::*,
};

pub struct GroupsBuilder<Genders: IntoIterator<Item = Group>, Tags: IntoIterator<Item = Group>> {
    pub tier: Group,
    pub author: Group,
    pub genders: Genders,
    pub tags: Tags,
}

impl<Genders: IntoIterator<Item = Group>, Tags: IntoIterator<Item = Group>>
    From<GroupsBuilder<Genders, Tags>> for Groups
{
    fn from(
        GroupsBuilder {
            tier,
            author,
            genders,
            tags,
        }: GroupsBuilder<Genders, Tags>,
    ) -> Self {
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
