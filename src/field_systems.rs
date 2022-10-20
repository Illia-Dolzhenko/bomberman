use crate::components::*;
use crate::constants::*;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::utils::Instant;
use std::fs;

pub fn load_field_system(mut field: ResMut<Field>, mut exit: EventWriter<AppExit>) {
    if !field.loaded {
        println!("Loading level...");
        field.array = load_level(field.current_level).unwrap_or_else(|| {
            println!("Can't load the level.");
            exit.send(AppExit);
            [[0; SIZE_IN_CELLS]; SIZE_IN_CELLS]
        });
        field.loaded = true;
        println!("Level loaded.");
    }
}

pub fn spawn_field_system(
    mut commands: Commands,
    mut field: ResMut<Field>,
    textures: Res<GameTextures>,
) {
    if !field.spawned && field.loaded {
        for i in 0..field.array.len() {
            for j in 0..field.array[0].len() {
                match field.array[i][j] {
                    1 => {
                        commands
                            .spawn_bundle(SpriteBundle {
                                texture: textures.wall.clone(),

                                transform: Transform {
                                    translation: Vec3 {
                                        x: i as f32 * CELL_SIZE - FIELD_OFFSET + CELL_OFFSET,
                                        y: j as f32 * CELL_SIZE - FIELD_OFFSET + CELL_OFFSET,
                                        z: 2.,
                                    },
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(Wall)
                            .insert(Solid);
                    }
                    2 => {
                        commands
                            .spawn_bundle(SpriteBundle {
                                texture: textures.wood.clone(),
                                transform: Transform {
                                    translation: Vec3 {
                                        x: i as f32 * CELL_SIZE - FIELD_OFFSET + CELL_OFFSET,
                                        y: j as f32 * CELL_SIZE - FIELD_OFFSET + CELL_OFFSET,
                                        z: 2.,
                                    },
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(BreakableWall)
                            .insert(Solid)
                            .insert(Destructable);
                    }
                    3 => {
                        commands
                            .spawn_bundle(SpriteBundle {
                                texture: textures.enemy.clone(),
                                transform: Transform {
                                    translation: Vec3 {
                                        x: i as f32 * CELL_SIZE - FIELD_OFFSET + CELL_OFFSET,
                                        y: j as f32 * CELL_SIZE - FIELD_OFFSET + CELL_OFFSET,
                                        z: 2.,
                                    },
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(Enemy)
                            .insert(Destructable)
                            .insert(Solid);
                    }
                    4 => {
                        commands
                            .spawn_bundle(SpriteSheetBundle {
                                texture_atlas: textures.player.clone(),
                                transform: Transform {
                                    translation: Vec3 {
                                        x: i as f32 * CELL_SIZE - FIELD_OFFSET + CELL_OFFSET,
                                        y: j as f32 * CELL_SIZE - FIELD_OFFSET + CELL_OFFSET,
                                        z: 1.,
                                    },
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(Player {
                                health: 10,
                                last_hit: Instant::now(),
                            });
                    }
                    _ => {}
                }
            }
        }
        field.spawned = true;
        println!("Field spawned");
    }
}

pub fn complete_level_system(
    mut field: ResMut<Field>,
    query: Query<&Enemy>,
    mut commands: Commands,
    entities: Query<Entity, With<Solid>>,
    player: Query<Entity, With<Player>>,
) {
    if query.is_empty() {
        field.current_level += 1;
        field.loaded = false;
        field.spawned = false;

        if let Ok(player_entity) = player.get_single() {
            commands.entity(player_entity).despawn();
        }

        entities.for_each(|entity| {
            commands.entity(entity).despawn();
        })
    }
}

fn load_level(index: u32) -> Option<[[i32; SIZE_IN_CELLS]; SIZE_IN_CELLS]> {
    let level = fs::read_to_string(format!("assets/{}.level", index));
    match level {
        Ok(text) => {
            println!("File is loaded. ({})", text);
            Some(create_level_from_string(text))
        }
        Err(_) => {
            println!("Can't read the file.");
            None
        }
    }
}

fn create_level_from_string(level_data: String) -> [[i32; SIZE_IN_CELLS]; SIZE_IN_CELLS] {
    let mut array = [[0; SIZE_IN_CELLS]; SIZE_IN_CELLS];

    let level_vec: Vec<Vec<char>> = level_data
        .split('\n')
        .map(|line| {
            return line.chars().collect::<Vec<char>>();
        })
        .collect();

    if level_vec
        .iter()
        .all(|char_vec| { char_vec.len() == SIZE_IN_CELLS } && level_vec.len() == SIZE_IN_CELLS)
    {
        for (i,char_vec) in level_vec.iter().enumerate().take(SIZE_IN_CELLS) {
            for j in 0..SIZE_IN_CELLS {
                //if let Some(char_vec) = level_vec.get(i) {
                    if let Some(char) = char_vec.get(j) {
                        match char {
                            'W' => {
                                array[i][j] = 1; // Wall
                            }
                            'B' => {
                                array[i][j] = 2; // Breakable wall
                            }
                            'E' => {
                                array[i][j] = 3; // Enemy
                            }
                            'S' => {
                                array[i][j] = 4; // Player spawn
                            }
                            _ => (),
                        }
                    }
                //}
            }
        }
    }

    array
}
