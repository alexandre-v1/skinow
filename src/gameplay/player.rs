use bevy::{prelude::*, sprite::Anchor};

use crate::{
    AppSystems, IsLocal, PausableSystems,
    asset_tracking::LoadResource,
    gameplay::movement::{Dragable, JumpInput, JumpState, MovementController},
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
}

const START_POS_X: f32 = 20.0;

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
