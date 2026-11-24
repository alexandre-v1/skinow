//! Spawn the main level.

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    IsLocal,
    asset_tracking::LoadResource,
    gameplay::{
        lane::{self, LANE_Y, LANE_Y_CENTER},
        obstacle::Sequence,
    },
    screens::Screen,
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
    commands.insert_resource(Sequence::default());
    let center: Vec2 = camera.translation().truncate();

    let lane_local = commands.spawn_empty().id();
    let lane_other = commands.spawn_empty().id();

    commands
        .spawn_scene(bsn! {
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
            ]
        })
        .add_children(&[lane_local, lane_other]);

    commands.entity(lane_local).apply_scene(bsn! {
        @lane::scene(true, lane_local)
        IsLocal
        Transform::from_translation(Vec3::default().with_y(-LANE_Y_CENTER))
    });

    commands.entity(lane_other).apply_scene(bsn! {
        @lane::scene(false, lane_other)
        Transform::from_translation(Vec3::default().with_y(LANE_Y - LANE_Y_CENTER))
    });
}
