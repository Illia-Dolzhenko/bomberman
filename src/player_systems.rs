use std::time::Duration;

use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::*;

pub const OFFSET: f32 = CELL_SIZE / 2.;

type PlayerQuery<'a> = (&'a mut Transform, &'a mut TextureAtlasSprite, Entity, Option<&'a Animator<Transform>>);

pub fn move_player_system(
    mut query: Query<
        PlayerQuery,
        (With<Player>, Without<Solid>),
    >,
    walls_query: Query<&Transform, With<Solid>>,
    key: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if let Ok((transform, mut texture, entity, animator)) = query.get_single_mut() {
            if animator.is_none() || (animator.is_some() && animator.unwrap().progress() == 1.0) {
                let mut end = transform.translation;
                let mut closest_walls: Vec<&Transform> = Vec::new();

                for wall in walls_query.iter() {
                    if ((wall.translation.x - CELL_SIZE)..(wall.translation.x + CELL_SIZE * 2.))
                        .contains(&transform.translation.x)
                        && ((wall.translation.y - CELL_SIZE)..(wall.translation.y + CELL_SIZE * 2.))
                            .contains(&transform.translation.y)
                    {
                        closest_walls.push(wall);
                    }
                }

                if key.just_pressed(KeyCode::W) && !closest_walls.iter().any(|wall| {
                        wall.translation.x as i32 == transform.translation.x as i32
                            && wall.translation.y as i32
                                == transform.translation.y as i32 + CELL_SIZE as i32
                    }) {
                    end.y += CELL_SIZE;
                    texture.index = 1;
                }
                if key.just_pressed(KeyCode::S) && !closest_walls.iter().any(|wall| {
                        wall.translation.x as i32 == transform.translation.x as i32
                            && wall.translation.y as i32
                                == transform.translation.y as i32 - CELL_SIZE as i32
                    }) {
                    end.y -= CELL_SIZE;
                    texture.index = 0;
                }
                if key.just_pressed(KeyCode::D) && !closest_walls.iter().any(|wall| {
                        wall.translation.x as i32
                            == transform.translation.x as i32 + CELL_SIZE as i32
                            && wall.translation.y as i32 == transform.translation.y as i32
                    }) {
                    end.x += CELL_SIZE;
                    texture.index = 2;
                }
                if key.just_pressed(KeyCode::A) && !closest_walls.iter().any(|wall| {
                        wall.translation.x as i32
                            == transform.translation.x as i32 - CELL_SIZE as i32
                            && wall.translation.y as i32 == transform.translation.y as i32
                    }) {
                    end.x -= CELL_SIZE;
                    texture.index = 3;
                }

                if end != transform.translation {
                    commands.entity(entity).insert(Animator::new(Tween::new(
                        EaseFunction::QuadraticIn,
                        TweeningType::Once,
                        Duration::from_millis(MOVE_ANIMATION_DURATION),
                        TransformPositionLens {
                            start: transform.translation,
                            end,
                        },
                    )));
                }
            }
    }
}
