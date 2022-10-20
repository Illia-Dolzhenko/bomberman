use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use std::time::Duration;

use crate::components::*;
use crate::constants::*;
use crate::enemy_systems::DAMAGE;
use crate::utils::is_equal;
use bevy::prelude::*;
use bevy_tweening::lens::TransformScaleLens;
use bevy_tweening::*;

pub fn spawn_bomb_system(
    key: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<&Bomb>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
    textures: Res<GameTextures>,
) {
    if query.is_empty() && key.just_pressed(KeyCode::Space) {
        if let Ok(transform) = player_query.get_single() {
            commands
                .spawn_bundle(SpriteBundle {
                    texture: textures.bomb.clone(),
                    transform: Transform {
                        translation: Vec3 {
                            x: transform.translation.x,
                            y: transform.translation.y,
                            z: 2.,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Bomb {
                    spawned: time.startup() + time.time_since_startup(),
                })
                .insert(Solid)
                .insert(Animator::new(Tween::new(
                    EaseFunction::BackInOut,
                    TweeningType::Loop,
                    Duration::from_millis(250),
                    TransformScaleLens {
                        start: transform.scale,
                        end: Vec3 {
                            x: transform.scale.x / 2.,
                            y: transform.scale.y / 2.,
                            z: transform.scale.z,
                        },
                    },
                )));
        }
    }
}

pub fn detonate_bomb_system(
    mut commands: Commands,
    query: Query<(&Bomb, Entity, &Transform), With<Bomb>>,
    wall_query: Query<&Transform, With<Wall>>,
    time: Res<Time>,
    textures: Res<GameTextures>,
) {
    for bomb in query.iter() {
        //println!("bomb.0.spawned.elapsed().as_secs() = {}", bomb.0.spawned.elapsed().as_secs());
        if bomb.0.spawned.elapsed().as_secs() > BOMB_TIMER {
            commands.entity(bomb.1).despawn();
            //todo explosion
            //println!("Boom!");

            let right_boundary = find_boundary(0..=EXPLOSION_SIZE, &wall_query, &bomb, -1, (1, 0));

            let left_boundary =
                find_boundary((-EXPLOSION_SIZE..=0).rev(), &wall_query, &bomb, 1, (1, 0));

            let top_boundary = find_boundary(0..=EXPLOSION_SIZE, &wall_query, &bomb, -1, (0, 1));

            let bottom_boundary =
                find_boundary((-EXPLOSION_SIZE..=0).rev(), &wall_query, &bomb, 1, (0, 1));

            for i in left_boundary..=right_boundary {
                let mut index: usize = 1;
                let mut rotation = Quat::from_rotation_z(0.);

                if i == 0 {
                    index = 0;
                }

                if i == left_boundary && i != 0 {
                    index = 2;
                    rotation = Quat::from_rotation_z(PI);
                }

                if i == right_boundary && i != 0 {
                    index = 2;
                }

                spawn_explosion(
                    &mut commands,
                    bomb.2.translation.x + CELL_SIZE * i as f32,
                    bomb.2.translation.y,
                    Explosion {
                        spawned: time.startup() + time.time_since_startup(),
                    },
                    textures.explosion.clone(),
                    index,
                    rotation,
                );
            }

            for i in bottom_boundary..=top_boundary {
                if i != 0 {
                    let mut index: usize = 1;
                    let mut rotation = Quat::from_rotation_z(FRAC_PI_2);

                    if i == bottom_boundary {
                        index = 2;
                        rotation = Quat::from_rotation_z((PI * 3.) / 2.);
                    }

                    if i == top_boundary {
                        index = 2;
                        rotation = Quat::from_rotation_z(FRAC_PI_2);
                    }
                    spawn_explosion(
                        &mut commands,
                        bomb.2.translation.x,
                        bomb.2.translation.y + CELL_SIZE * i as f32,
                        Explosion {
                            spawned: time.startup() + time.time_since_startup(),
                        },
                        textures.explosion.clone(),
                        index,
                        rotation,
                    );
                }
            }
            // println!(
            //     "LB: {}, RB: {}, TB: {}, BB: {}",
            //     left_boundary, right_boundary, top_boundary, bottom_boundary
            // );
        }
    }
}

fn find_boundary<I>(
    range: I,
    wall_query: &Query<&Transform, With<Wall>>,
    bomb: &(&Bomb, Entity, &Transform),
    edge: i32,
    xy: (i32, i32),
) -> i32
where
    I: Iterator<Item = i32>,
{
    let mut boundary = -(EXPLOSION_SIZE * edge);
    for i in range {
        if wall_query.iter().any(|wall| {
            wall.translation.x == bomb.2.translation.x + CELL_SIZE * i as f32 * xy.0 as f32
                && wall.translation.y == bomb.2.translation.y + CELL_SIZE * i as f32 * xy.1 as f32
        }) {
            boundary = i + edge;
            break;
        }
    }
    boundary
}

fn spawn_explosion(
    commands: &mut Commands,
    x: f32,
    y: f32,
    explosion: Explosion,
    texture: Handle<TextureAtlas>,
    index: usize,
    rotation: Quat,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture,
            sprite: TextureAtlasSprite::new(index),
            transform: Transform {
                translation: Vec3 { x, y, z: 2. },
                rotation,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(explosion);
}

pub fn remove_explosion_system(mut commands: Commands, query: Query<(&Explosion, Entity)>) {
    for explosion in query.iter() {
        if explosion.0.spawned.elapsed().as_millis() > 250 {
            commands.entity(explosion.1).despawn();
        }
    }
}

pub fn explosion_destruction_system(
    mut commands: Commands,
    query: Query<(&Transform, Entity), With<Destructable>>,
    mut player_query: Query<(&Transform, &mut Player, &mut TextureAtlasSprite), With<Player>>,
    time: Res<Time>,
    explosion_query: Query<&Transform, With<Explosion>>,
) {
    for explosion in explosion_query.iter() {
        query
            .iter()
            .filter(|destructable| is_equal(destructable.0, explosion))
            .for_each(|destructable| {
                commands.entity(destructable.1).despawn();
            });

        if let Ok(mut player) = player_query.get_single_mut() {
            if is_equal(player.0, explosion) && player.1.last_hit.elapsed().as_millis() > 150 {
                player.1.health -= DAMAGE;
                player.1.last_hit = time.startup() + time.time_since_startup();
                player.2.color = Color::RED;
            }
        }
    }
}
