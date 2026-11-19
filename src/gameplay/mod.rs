use bevy::prelude::*;

pub mod lane;
pub mod level;
mod movement;
pub mod parallax;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        lane::plugin,
        level::plugin,
        movement::plugin,
        parallax::plugin,
        player::plugin,
    ));
}
