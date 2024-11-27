mod close;
mod game;
mod load;
mod marker;
mod menu;
mod state;
mod weird_utils;
mod debug;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(state::plugin)
        .add_plugins(menu::plugin)
        .add_plugins(game::plugin)
        .add_plugins(load::plugin)
        .add_plugins(close::plugin)
        .add_plugins(debug::plugin)
        .run();
}
