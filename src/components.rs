use bevy::{prelude::*, utils::Instant};

#[derive(Component)]
pub struct Player{
    pub health: i32,
    pub last_hit: Instant,
}

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct BreakableWall;

#[derive(Component)]
pub struct Solid;

#[derive(Component)]
pub struct Destructable;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Bomb{
    pub spawned: Instant,
}

#[derive(Component)]
pub struct Explosion{
    pub spawned: Instant,
}

#[derive(Component)]
pub struct Info;

pub struct Field{
    pub array: [[i32; 30];30],
    pub loaded: bool,
    pub spawned: bool,
    pub current_level: u32,
}

pub struct GameTextures {
    pub wall: Handle<Image>,
    pub wood: Handle<Image>,
    pub bomb: Handle<Image>,
    pub enemy: Handle<Image>,
    pub player: Handle<TextureAtlas>,
    pub explosion: Handle<TextureAtlas>
}

