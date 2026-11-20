use bevy::prelude::*;

use crate::{gameplay::lane::MIN_Y_POS, AppSystems, IsLocal, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (apply_movement, jump_physics)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

const GRAVITY: f32 = 400.0;
const JUMP_SPEED: f32 = 200.0;
const JUMP_CUT_MULT: f32 = 2.5;
const FALL_MULT: f32 = 1.4;

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct MovementController {
    /// The direction the character wants to move in (on axis x).
    pub intent: f32,

    /// Maximum speed in world units per second.
    /// 1 world unit = 1 pixel when using the default 2D camera and no physics engine.
    pub max_speed: f32,
}

#[derive(Component, Default, Clone)]
pub struct JumpInput {
    pub pressed: bool,
    pub held: bool,
}

#[derive(Component, Default, Clone)]
pub struct JumpState {
    pub velocity_y: f32,
    pub grounded: bool,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: 0.0,
            max_speed: 100.0,
        }
    }
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut Transform), With<IsLocal>>,
) {
    for (controller, mut transform) in &mut movement_query {
        let velocity = controller.max_speed * controller.intent;
        transform.translation += Vec3::default().with_x(velocity) * time.delta_secs();
    }
}

fn jump_physics(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut JumpState, &mut JumpInput)>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut state, mut input) in &mut query {
        if state.grounded && input.pressed {
            state.velocity_y = JUMP_SPEED;
            state.grounded = false;
        }
        input.pressed = false;

        let gravity = if state.velocity_y > 0.0 && !input.held {
            GRAVITY * JUMP_CUT_MULT
        } else if state.velocity_y < 0.0 {
            GRAVITY * FALL_MULT
        } else {
            GRAVITY
        };

        state.velocity_y -= gravity * dt;
        transform.translation.y += state.velocity_y * dt;

        if transform.translation.y <= MIN_Y_POS {
            transform.translation.y = MIN_Y_POS;
            state.velocity_y = 0.0;
            state.grounded = true;
        }
    }
}
