use bevy::prelude::*;
use crate::configure_stated_system_set;
use crate::state::GameState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameSystemSet;

pub(crate) fn plugin(app: &mut App) {
    configure_stated_system_set!(app, GameSystemSet, GameState::InGame);
}