//! Spawn the main level.

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    asset_tracking::LoadResource,
    gameplay::lane::{self, LANE_Y, LANE_Y_CENTER},
    screens::Screen,
    IsLocal,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let _assets = world.resource::<AssetServer>();
        Self {}
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    _level_assets: Res<LevelAssets>,
    camera: Single<&GlobalTransform, With<Camera2d>>,
) {
    let center: Vec2 = camera.translation().truncate();
    commands.spawn_scene(bsn! {
        #Level
        Transform
        Visibility
        DespawnOnExit<Screen>(Screen::Gameplay)
        Children[
            #Gameboy
            Sprite {
                image: "images/gameboy.png"
            }
            Transform::from_translation(center.extend(100.0)).with_scale(Vec3::splat(8.0))
            Anchor::CENTER
            --
            @lane::scene()
            IsLocal
            Transform::from_translation(Vec3::default().with_y(-LANE_Y_CENTER))
            --
            @lane::scene()
            Transform::from_translation(Vec3::default().with_y(LANE_Y - LANE_Y_CENTER))
        ]
    });
}
