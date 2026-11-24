use bevy::prelude::*;

use crate::PausableSystems;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, update_speed.in_set(PausableSystems));
}

#[derive(Component, Clone, Default, Deref, DerefMut)]
#[require(ComputedSpeed, Acceleration)]
pub struct Speed(pub f32);

#[derive(Component, Clone, Default, Deref, DerefMut)]
pub struct ComputedSpeed(pub f32);

#[derive(Component, Clone, Default)]
pub struct Acceleration {
    pub up: f32,
    pub down: f32,
}

#[derive(Clone)]
pub enum Mod {
    Add(f32),
    Mul(f32),
}

impl Default for Mod {
    fn default() -> Self {
        Self::Add(0.0)
    }
}

impl Mod {
    fn apply(&self, value: f32) -> f32 {
        match self {
            Mod::Add(v) => value + v,
            Mod::Mul(v) => value * v,
        }
    }
}

/// Speed boost based on distance from the right of the screen
#[derive(Component, Clone, Default, Deref, DerefMut)]
pub struct RiskBoost(Mod);

fn update_speed(
    time: Res<Time>,
    mut query: Query<(
        &Speed,
        &Acceleration,
        &mut ComputedSpeed,
        Option<&mut RiskBoost>,
    )>,
) {
    let dt = time.delta_secs();
    for (base, accel, mut computed, risk) in &mut query {
        let mut target = **base;
        if let Some(risk) = risk {
            target = risk.0.apply(target);
        }
        let target = target.max(0.0);

        let rate = if target > **computed {
            accel.up
        } else {
            accel.down
        };
        let step = rate * dt;
        **computed = if (target - **computed).abs() <= step {
            target
        } else {
            **computed + step * (target - **computed).signum()
        };
    }
}
