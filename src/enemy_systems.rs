use std::time::Duration;

use crate::components::*;
use crate::constants::*;
use crate::utils::is_equal_approximate;
use bevy::prelude::*;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::*;
use rand::prelude::*;

pub const DAMAGE: i32 = 1;

type EnemyQuery<'a> = (&'a Transform, Entity, Option<&'a Animator<Transform>>);

pub fn move_enemy_system(
    mut query: Query<EnemyQuery, With<Enemy>>,
    wall_query: Query<&Transform, (With<Solid>, Without<Enemy>)>,
    mut commands: Commands,
) {
    let mut random = rand::thread_rng();

    for (enemy, entity, animator_option) in query.iter_mut() {
        if animator_option.is_none()
            || (animator_option.is_some() && animator_option.unwrap().progress() == 1.0)
        {
            let mut moved = false;
            while !moved {
                let mut end = enemy.translation;
                match random.gen_range(0..5) {
                    0 => {
                        if !wall_query.iter().any(|wall| {
                            wall.translation.x == enemy.translation.x + CELL_SIZE
                                && wall.translation.y == enemy.translation.y
                        }) {
                            end.x += CELL_SIZE;
                            moved = true;
                        }
                    }
                    1 => {
                        if !wall_query.iter().any(|wall| {
                            wall.translation.x == enemy.translation.x
                                && wall.translation.y == enemy.translation.y + CELL_SIZE
                        }) {
                            end.y += CELL_SIZE;
                            moved = true;
                        }
                    }
                    2 => {
                        if !wall_query.iter().any(|wall| {
                            wall.translation.x == enemy.translation.x - CELL_SIZE
                                && wall.translation.y == enemy.translation.y
                        }) {
                            end.x -= CELL_SIZE;
                            moved = true;
                        }
                    }
                    3 => {
                        if !wall_query.iter().any(|wall| {
                            wall.translation.x == enemy.translation.x
                                && wall.translation.y == enemy.translation.y - CELL_SIZE
                        }) {
                            end.y -= CELL_SIZE;
                            moved = true;
                        }
                    }
                    _ => moved = true,
                }
                if end != enemy.translation {
                    commands.entity(entity).insert(Animator::new(Tween::new(
                        EaseFunction::QuadraticIn,
                        TweeningType::Once,
                        Duration::from_millis(MOVE_ANIMATION_DURATION),
                        TransformPositionLens {
                            start: enemy.translation,
                            end,
                        },
                    )));
                }
            }
        }
    }
}

pub fn enemy_kill_player_system(
    mut player_query: Query<(&Transform, &mut Player, &mut TextureAtlasSprite), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    time: Res<Time>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let attacks = enemy_query
            .iter()
            .filter(|enemy| is_equal_approximate(player.0, enemy))
            .count();

        if attacks > 0 && player.1.last_hit.elapsed().as_millis() > 150 {
            player.1.health -= attacks as i32 * DAMAGE;
            player.1.last_hit = time.startup() + time.time_since_startup();
            player.2.color = Color::RED;
        }

        if player.1.last_hit.elapsed().as_millis() > 200 {
            player.2.color = Color::WHITE;
        }
    }
}

pub fn debug_kill_enemy(
    mut commands: Commands,
    query: Query<Entity, With<Enemy>>,
    key: Res<Input<KeyCode>>,
) {
    if key.just_pressed(KeyCode::K) {
        if let Some(enemy) = query.iter().next() {
            commands.entity(enemy).despawn();
        }
    }
}
