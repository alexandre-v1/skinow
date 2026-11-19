use bevy::{prelude::*, sprite::Anchor};

use crate::PausableSystems;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, scroll_and_recycle.in_set(PausableSystems));
}

#[derive(Component, Debug, Clone, Default, Deref, DerefMut)]
pub struct LayerSpeed(f32);

#[derive(Component, Default, Clone)]
pub struct ParallaxSegment {
    width: f32,
}

fn segment(x: f32, image: &'static str, width: f32) -> impl Scene {
    bsn! {
        ParallaxSegment {
            width
        }
        Sprite { image }
        Anchor::BOTTOM_LEFT
        Transform::from_xyz(x, 0.0, 0.0)
    }
}

#[derive(Clone, Default)]
pub struct LayerInfo {
    pub speed: f32,
    pub z: f32,
    pub y: f32,
    pub image: &'static str,
    pub image_width: f32,
}

pub fn layer(info: LayerInfo) -> impl Scene {
    let speed = info.speed;
    let image = info.image;
    let width = info.image_width;

    bsn! {
        LayerSpeed(speed)
        Transform::from_xyz(0.0, info.y, info.z)
        Visibility
        Children [
            @segment(0.0, image, width)
            --
            @segment(width, image, width)
            --
            @segment(width * 2.0, image, width)
        ]
    }
}

fn scroll_and_recycle(
    time: Res<Time>,
    _assets: Res<AssetServer>,
    layers: Query<(&LayerSpeed, &Children)>,
    mut segs: Query<(&mut Transform, &mut Sprite, &ParallaxSegment)>,
) {
    for (layer_speed, children) in &layers {
        let seg_count = children.len();
        for child in children.iter() {
            let Ok((mut transform, mut _sprite, segment)): std::prelude::v1::Result<
                (Mut<'_, Transform>, Mut<'_, Sprite>, &ParallaxSegment),
                bevy::ecs::query::QueryEntityError,
            > = segs.get_mut(child) else {
                continue;
            };
            transform.translation.x -= **layer_speed * time.delta_secs();
            if transform.translation.x <= -segment.width {
                transform.translation.x += segment.width * seg_count as f32;
            }
        }
    }
}
