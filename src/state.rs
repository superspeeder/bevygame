use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, States, Default)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    Closing,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, SubStates, Default)]
#[source(GameState = GameState::MainMenu)]
pub enum MenuPage {
    #[default]
    TitleScreen,
    SettingsPage,
    FileSelect,
    Credits,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, SubStates, Default)]
#[source(GameState = GameState::InGame)]
pub enum PlayingState {
    #[default]
    Playing,
    Paused { reason: PauseReason },
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PauseReason {
    PauseMenu,
    Unfocused,
    InCutscene,
    InInventory,
}


pub(crate) fn plugin(app: &mut App) {
    app.init_state::<GameState>().add_sub_state::<MenuPage>().add_sub_state::<PlayingState>();
}