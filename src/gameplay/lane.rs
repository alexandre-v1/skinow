use bevy::{ecs::template::EntityTemplate, prelude::*, sprite::Anchor};

use crate::{
    IsLocal, PausableSystems,
    gameplay::{
        obstacle::{Obstacle, SequenceIndex},
        parallax::{self, LayerInfo},
        player,
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, update_distance.in_set(PausableSystems));
    app.add_systems(
        Update,
        (move_in_lanes, despawn_obstacles).in_set(PausableSystems),
    );
}

pub const LANE_Y: f32 = 108.0;
pub const LANE_Y_CENTER: f32 = LANE_Y / 2.0;

pub const LANE_X: f32 = 240.0;
pub const LANE_X_CENTER: f32 = LANE_X / 2.0;

pub const MIN_X_POS: f32 = 2.0;
pub const MAX_X_POS: f32 = 200.0;

pub const MIN_Y_POS: f32 = 25.0;
pub const _MAX_Y_POS: f32 = 60.0;

pub const SPAWN_AHEAD: f32 = 500.0;

#[derive(Component, Clone, Debug, FromTemplate)]
#[relationship(relationship_target = LaneEntities)]
pub struct InLane {
    #[relationship]
    pub lane: Entity,
    pub move_ratio: Option<f32>,
}

impl Default for InLane {
    fn default() -> Self {
        Self::new(Entity::PLACEHOLDER)
    }
}

impl InLane {
    pub fn new(lane: Entity) -> Self {
        Self {
            lane,
            move_ratio: None,
        }
    }
}

impl InLaneTemplate {
    pub fn new(lane: impl Into<EntityTemplate>) -> Self {
        Self {
            lane: lane.into(),
            move_ratio: None,
            ..default()
        }
    }

    pub fn with_ratio(mut self, ratio: f32) -> Self {
        self.move_ratio = Some(ratio);
        self
    }

    pub fn with_terrain_ratio(self) -> Self {
        self.with_ratio(1.0)
    }
}

#[derive(Component, Clone, PartialEq, Eq, Debug)]
#[relationship_target(relationship = InLane)]
pub struct LaneEntities(Vec<Entity>);

#[derive(Component, Clone, Default, Deref, DerefMut)]
pub struct Speed(f32);

#[derive(Component, Clone, Default, Deref, DerefMut)]
pub struct Distance(f32);

pub fn scene(is_local: bool) -> impl Scene {
    bsn! {
        #Lane
        Speed(85.0)
        Distance
        Visibility
        SequenceIndex
        @local_marker(is_local)
        Children [
            InLane {
                lane: #Lane
            }
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

fn update_distance(mut query: Query<(&mut Distance, &Speed)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (mut distance, speed) in &mut query {
        distance.0 += **speed * dt;
    }
}

fn move_in_lanes(
    in_lanes: Query<(&mut Transform, &InLane)>,
    lanes: Query<&Speed, With<LaneEntities>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (mut transform, in_lane) in in_lanes {
        let Some(move_ratio) = in_lane.move_ratio else {
            continue;
        };

        let Ok(speed) = lanes.get(in_lane.lane) else {
            continue;
        };

        transform.translation.x -= (**speed * move_ratio) * dt;
    }
}

const DESPAWN_POSITION_X: f32 = -50.0;
fn despawn_obstacles(
    obstacles: Query<(&Transform, Entity), With<Obstacle>>,
    mut commands: Commands,
) {
    for (transform, entity) in obstacles {
        if transform.translation.x > DESPAWN_POSITION_X {
            continue;
        }
        commands.entity(entity).despawn();
    }
}
