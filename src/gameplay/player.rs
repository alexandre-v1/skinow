use bevy::{prelude::*, sprite::Anchor};

use crate::{
    asset_tracking::LoadResource,
    gameplay::{
        lane::{InLane, MAX_X_POS, MIN_X_POS},
        movement::{Dragable, JumpInput, JumpState, MovementController},
        speed::{self, RiskBoost},
    },
    AppSystems, IsLocal, PausableSystems,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        (record_player_directional_input, record_jump_input)
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );

    app.add_systems(FixedUpdate, update_risk_boost.in_set(PausableSystems));
}

const START_POS_X: f32 = 30.0;

/// The player character.
pub fn scene() -> impl Scene {
    bsn! {
        #Player
        Player
        Sprite {
            image:  DINO,
        }
        Anchor::BOTTOM_LEFT
        Transform::from_xyz(START_POS_X, 0.0, 0.0)
        MovementController {
            max_speed: 100.0,
        }
        JumpState
        JumpInput
        Dragable
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    // Collect directional input.
    let mut intent = 0.0;
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent += 1.0;
    }

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}

fn record_jump_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut JumpInput, With<IsLocal>>,
) {
    let keys = [KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::Space];
    for mut jump in &mut query {
        if input.any_just_pressed(keys) {
            jump.pressed = true;
        }
        jump.held = input.any_pressed(keys);
    }
}

const DINO: &str = "images/dino.png";

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    _dino: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            _dino: assets.load(DINO),
        }
    }
}

const MAX_RISK_BOOST: f32 = 10.0;

fn update_risk_boost(
    players: Query<(&Transform, &InLane), With<Player>>,
    mut lanes: Query<&mut RiskBoost>,
) {
    for (transform, in_lane) in players {
        let Ok(mut risk) = lanes.get_mut(in_lane.lane) else {
            continue;
        };

        let x = transform.translation.x;
        let t = ((x - MIN_X_POS) / (MAX_X_POS - MIN_X_POS)).clamp(0.0, 1.0);
        let boost = MAX_RISK_BOOST * t;

        **risk = speed::Mod::Add(boost);
    }
}
