use super::player_id::PlayerID;

pub struct PlayerIDManager {
    next_id: PlayerID,
}

impl Default for PlayerIDManager {
    fn default() -> Self {
        Self { next_id: PlayerID(1) }
    }
}

impl PlayerIDManager {
    pub fn next_id(&mut self) -> PlayerID {
        let res = self.next_id;
        self.next_id.0 += 1;
        res
    }
}
