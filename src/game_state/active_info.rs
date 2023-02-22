use crate::acts::ActiveType;

#[derive(Clone)]
pub struct ActiveInfo {
    pub type_: ActiveType,
}

impl From<ActiveType> for ActiveInfo {
    fn from(type_: ActiveType) -> Self {
        Self { type_ }
    }
}
