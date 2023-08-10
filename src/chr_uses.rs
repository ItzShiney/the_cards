pub use crate::card_uses::*;

#[allow(unused)]
pub fn handle_event(
    game: &mut Game,
    chr_id: CharacterID,
    signed_event: SignedEvent,
) -> EventResult {
    Ok(signed_event)
}

#[allow(unused)]
pub fn handle_check(
    game: &Game,
    chr_id: CharacterID,
    signed_check: SignedCheck,
) -> CheckResult {
    Ok(signed_check)
}
