use bevy::prelude::*;

pub mod hitbox;
pub mod lane;
pub mod level;
mod movement;
pub mod obstacle;
pub mod parallax;
pub mod player;
pub mod speed;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        hitbox::plugin,
        lane::plugin,
        level::plugin,
        movement::plugin,
        obstacle::plugin,
        parallax::plugin,
        player::plugin,
        speed::plugin,
    ));
}
