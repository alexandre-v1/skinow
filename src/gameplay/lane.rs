use bevy::{prelude::*, sprite::Anchor};

use crate::{
    IsLocal,
    gameplay::{
        parallax::{self, LayerInfo},
        player,
    },
};

pub fn plugin(_app: &mut App) {}

pub const LANE_Y: f32 = 108.0;
pub const LANE_Y_CENTER: f32 = LANE_Y / 2.0;

pub const LANE_X: f32 = 240.0;
pub const LANE_X_CENTER: f32 = LANE_X / 2.0;

pub const MIN_X_POS: f32 = 2.0;
pub const MAX_X_POS: f32 = 320.0;

pub const MIN_Y_POS: f32 = 25.0;
pub const _MAX_Y_POS: f32 = 60.0;

#[derive(Component, Clone, PartialEq, Eq, Debug, FromTemplate)]
#[relationship(relationship_target = LaneEntities)]
pub struct InLane(pub Entity);

impl Default for InLane {
    fn default() -> Self {
        InLane(Entity::PLACEHOLDER)
    }
}

#[derive(Component, Clone, PartialEq, Eq, Debug)]
#[relationship_target(relationship = InLane)]
pub struct LaneEntities(Vec<Entity>);

#[derive(Component, Clone, Default)]
pub struct Speed(f32);

pub fn scene(is_local: bool) -> impl Scene {
    bsn! {
        #Lane
        Speed(10.0)
        Visibility
        @local_marker(is_local)
        Children [
            InLane(#Lane)
            @player::scene()
            @local_marker(is_local)
            --
            Sprite { image: "images/background.png" }
            Anchor::BOTTOM_LEFT
            Transform::from_xyz(0.0, 0.0, -100.0)
            --
            @parallax::layer(LayerInfo {
                speed: 85.0,
                z: 0.0,
                y: 0.0,
                image: "images/ground.png",
                image_width: 240.0,
            })
            --
            @parallax::layer(LayerInfo {
                speed: 45.0,
                z: -50.0,
                y: 25.0,
                image: "images/forest_back.png",
                image_width: 240.0,
            })
            --
            @parallax::layer(LayerInfo {
                speed: 55.0,
                z: -40.0,
                y: 25.0,
                image: "images/forest_front.png",
                image_width: 240.0,
            })
            --
            @parallax::layer(LayerInfo {
                speed: 10.0,
                z: -70.0,
                y: 25.0,
                image: "images/mountain.png",
                image_width: 240.0,
            })
            --
            @parallax::layer(LayerInfo {
                speed: 5.0,
                z: -80.0,
                y: 83.0,
                image: "images/moon.png",
                image_width: 300.0,
            })
            --
            @parallax::layer(LayerInfo {
                speed: 12.0,
                z: -60.0,
                y: 83.0,
                image: "images/cloud.png",
                image_width: 300.0,
            })

        ]
    }
}

fn local_marker(is_local: bool) -> Box<dyn Scene> {
    if is_local {
        return Box::new(bsn! { IsLocal });
    }
    Box::new(bsn! {})
}
