use bevy::{prelude::*, sprite::Anchor};

use crate::{gameplay::lane::InLane, PausableSystems};

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
    pub pos: Vec3,
    pub image: &'static str,
    pub lane: Entity,
}

impl LayerInfo {
    fn with_x(mut self, x: f32) -> Self {
        self.pos = self.pos.with_x(x);
        self
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
            @layer(info.with_x(width))
            ParallaxSegment {
                width
            }
            --
            @layer(info.with_x(width * 2.0))
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
        Transform::from_translation(info.pos)
    }
}

fn recycle_layer_segments(segs: Query<(&mut Transform, &ParallaxSegment), With<InLane>>) {
    for (mut transform, segment) in segs {
        if transform.translation.x <= -segment.width {
            transform.translation.x += segment.width * SEGMENT_COUNT;
        }
    }
}
