pub use crate::card_uses::*;

#[allow(unused)]
pub fn handle_event(game: &mut Game, act_id: ActiveID, signed_event: SignedEvent) -> EventResult {
    Ok(signed_event)
}

#[allow(unused)]
pub fn handle_check(game: &Game, act_id: ActiveID, signed_check: SignedCheck) -> CheckResult {
    Ok(signed_check)
}

#[allow(unused)]
pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    Err(Cancelled)
}

#[allow(unused)]
pub fn use_on_field(game: &mut Game, act_id: ActiveID) -> Result<(), Cancelled> {
    Err(Cancelled)
}
