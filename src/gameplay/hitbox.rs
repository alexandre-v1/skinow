use bevy::prelude::*;

pub(super) fn plugin(_app: &mut App) {}

#[derive(Component, Default, Clone)]
pub struct Hitbox {
    /// offset from the sprite's bottom-left corner
    pub offset: Vec2,
    pub size: Vec2,
}

impl Hitbox {
    pub fn _aabb(&self, gt: &GlobalTransform) -> Aabb2d {
        let min = gt.translation().truncate() + self.offset;
        let max = min + self.size;
        Aabb2d { min, max }
    }
}
