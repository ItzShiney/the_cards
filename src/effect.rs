use crate::host::GameCallbacks;

pub enum EffectDuration {
    Battle,
    Turn,
    Game,
}

pub struct Effect {
    pub duration: EffectDuration,
    pub callbacks: GameCallbacks,
}
