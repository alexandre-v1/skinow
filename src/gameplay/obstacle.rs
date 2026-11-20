use bevy::{prelude::*, sprite::Anchor};
use rand::{
    RngExt, SeedableRng,
    distr::{Distribution, weighted::WeightedIndex},
    rngs::StdRng,
};

use crate::{
    PausableSystems,
    gameplay::{
        hitbox::Hitbox,
        lane::{Distance, InLane, MIN_Y_POS, SPAWN_AHEAD},
    },
};

pub fn plugin(app: &mut App) {
    app.init_resource::<ObstacleTable>();
    app.init_resource::<Sequence>();
    app.add_systems(Update, spawn_obstacles.in_set(PausableSystems));
}

const START_X: f32 = 300.0;

const RAMP_DISTANCE: f32 = 6000.0; // max difficulty distance
const FADE_IN: f32 = 400.0;
const GAP_EASY: (f32, f32) = (220.0, 380.0);
const GAP_HARD: (f32, f32) = (90.0, 180.0);

#[derive(Component, Clone, Default)]
pub struct Obstacle;

pub struct ObstacleDef {
    pub unlock_distance: f32,
    pub base_weight: f32,
    pub width: f32,
    pub scene: fn() -> Box<dyn Scene>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct ObstacleTable(pub Vec<ObstacleDef>);

impl Default for ObstacleTable {
    fn default() -> Self {
        Self(vec![ObstacleDef {
            unlock_distance: 0.0,
            base_weight: 10.0,
            width: 40.0,
            scene: single_tree,
        }])
    }
}

impl ObstacleTable {
    fn difficulty(x: f32) -> f32 {
        (x / RAMP_DISTANCE).clamp(0.0, 1.0)
    }

    fn weight(def: &ObstacleDef, x: f32) -> f32 {
        if x < def.unlock_distance {
            return 0.0;
        }
        let fade = ((x - def.unlock_distance) / FADE_IN).clamp(0.2, 1.0);
        def.base_weight * fade
    }

    pub fn pick(&self, x: f32, rng: &mut StdRng) -> usize {
        let weights: Vec<f32> = self.iter().map(|d| Self::weight(d, x)).collect();
        match WeightedIndex::new(&weights) {
            Ok(dist) => dist.sample(rng),
            Err(_) => 0,
        }
    }

    pub fn gap(&self, x: f32, rng: &mut StdRng) -> f32 {
        let t = Self::difficulty(x);
        let before = GAP_EASY.0.lerp(GAP_HARD.0, t);
        let after = GAP_EASY.1.lerp(GAP_HARD.1, t);
        rng.random_range(before..=after)
    }
}

#[derive(Resource)]
pub struct Sequence {
    rng: StdRng,
    next_x: f32,
    placed: Vec<Placed>,
}

impl Default for Sequence {
    fn default() -> Self {
        let seed: u64 = rand::random::<u64>();
        Self::new(seed)
    }
}

impl Sequence {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            next_x: START_X,
            placed: Vec::new(),
        }
    }

    pub fn ensure(&mut self, length: f32, table: &ObstacleTable) {
        while self.next_x <= length {
            let pos_x = self.next_x;
            let def = table.pick(pos_x, &mut self.rng);
            let gap = table.gap(pos_x, &mut self.rng);

            self.next_x = pos_x + table[def].width + gap;
            self.placed.push(Placed { def, x: pos_x })
        }
    }
}

#[derive(Clone, Copy)]
struct Placed {
    def: usize,
    x: f32,
}

#[derive(Component, Default, Clone, Copy, Deref, DerefMut)]
pub struct SequenceIndex(pub usize);

fn single_tree() -> Box<dyn Scene> {
    Box::new(bsn! {
        Sprite {
            image: "images/tree.png"
        }
        Anchor::BOTTOM_LEFT
        Hitbox {
            offset: Vec2::new(3.0, 0.0),
            size: Vec2::new(16.0, 20.0),
        }
    })
}

fn spawn_obstacles(
    mut commands: Commands,
    mut sequence: ResMut<Sequence>,
    table: Res<ObstacleTable>,
    mut lanes: Query<(Entity, &mut SequenceIndex, &Distance)>,
) {
    for (lane, mut index, distance) in &mut lanes {
        sequence.ensure(**distance + SPAWN_AHEAD, &table);

        while let Some(placed) = sequence.placed.get(**index).copied() {
            if placed.x > **distance + SPAWN_AHEAD {
                break;
            }
            let def = &table[placed.def];
            let scene = (def.scene)();
            let x = placed.x - **distance;

            info!("Spawn obstacle at {}", x);
            commands.spawn_scene(bsn! {
                @scene
                Transform::from_xyz(x, MIN_Y_POS, 10.0)
                ChildOf(lane)
                Obstacle
                InLane::new(lane).with_terrain_ratio()
            });
            **index += 1;
        }
    }
}
