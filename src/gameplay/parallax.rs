use bevy::{prelude::*, sprite::Anchor};

use crate::{
    PausableSystems,
    gameplay::lane::{Distance, InLane},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, recycle_layer_segments.in_set(PausableSystems));
}

const SEGMENT_COUNT: f32 = 3.0;

#[derive(Component, Default, Clone)]
pub struct ParallaxSegment {
    width: f32,
}

#[derive(Clone, Copy)]
pub struct LayerInfo {
    pub move_ratio: Option<f32>,
    pub distance: Distance,
    pub y: f32,
    pub z: f32,
    pub image: &'static str,
    pub lane: Entity,
}

impl LayerInfo {
    fn add_distance(mut self, distance: f32) -> Self {
        self.distance.0 += distance;
        self
    }
}

impl Default for LayerInfo {
    fn default() -> Self {
        Self {
            move_ratio: Some(1.0),
            distance: Distance::default(),
            y: 0.0,
            z: 0.0,
            image: "",
            lane: Entity::PLACEHOLDER,
        }
    }
}

pub fn infinite_layer(info: LayerInfo, width: f32) -> impl Scene {
    bsn! {
        Transform
        Visibility
        Children [
            @layer(info)
            ParallaxSegment {
                width
            }
            --
            @layer(info.add_distance(width))
            ParallaxSegment {
                width
            }
            --
            @layer(info.add_distance(width * 2.0))
            ParallaxSegment {
                width
            }
        ]
    }
}

pub fn layer(info: LayerInfo) -> impl Scene {
    let move_ratio = info.move_ratio;
    let lane = info.lane;
    let image = info.image;

    bsn! {
        InLane {
            move_ratio,
            lane,
        }
        Sprite { image }
        Anchor::BOTTOM_LEFT
        Transform::from_xyz(0.0, info.y, info.z)
        info.distance.to_spawn()
    }
}

fn recycle_layer_segments(segs: Query<(&mut Transform, &ParallaxSegment), With<InLane>>) {
    for (mut transform, segment) in segs {
        if transform.translation.x <= -segment.width {
            transform.translation.x += segment.width * SEGMENT_COUNT;
        }
    }
}
