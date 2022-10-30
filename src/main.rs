use bevy::{prelude::*, time::FixedTimestep};
use bevy_tweening::*;
use bomb_systems::*;
use components::*;
use constants::*;
use enemy_systems::*;
use field_systems::*;
use player_systems::*;

pub mod bomb_systems;
pub mod components;
pub mod constants;
pub mod enemy_systems;
pub mod field_systems;
pub mod player_systems;
pub mod utils;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Bon'berman".to_string(),
            width: FIELD_SIZE,
            height: FIELD_SIZE,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TweeningPlugin)
        .add_startup_system(startup_system)
        .add_system(load_field_system)
        .add_system(spawn_field_system)
        .add_system(move_player_system)
        .add_system(spawn_bomb_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(detonate_bomb_system),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.25))
                .with_system(move_enemy_system),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.25))
                .with_system(update_info_system),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(update_info_system),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(complete_level_system),
        )
        .add_system(remove_explosion_system)
        .add_system(explosion_destruction_system)
        .add_system(enemy_kill_player_system)
        .add_system(debug_kill_enemy)
        .add_system(player_health_system)
        .run();
}

fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.insert_resource(Field {
        array: [[0; 30]; 30],
        loaded: false,
        spawned: false,
        current_level: 1,
    });

    let player_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(player_handle, Vec2 { x: 32.0, y: 32.0 }, 4, 1);

    let explosion_handle = asset_server.load("explosion.png");
    let explosion_atlas = TextureAtlas::from_grid(explosion_handle, Vec2 { x: 32., y: 32. }, 3, 1);

    commands.insert_resource(GameTextures{
        wall: asset_server.load("wall.png"),
        wood: asset_server.load("wood.png"),
        bomb: asset_server.load("bomb.png"),
        enemy: asset_server.load("enemy.png"),
        player: texture_atlases.add(texture_atlas),
        explosion: texture_atlases.add(explosion_atlas),    
    });
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "value",
                TextStyle {
                    font: asset_server.load("FiraSans-Regular.ttf"),
                    font_size: 16.,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(CELL_OFFSET),
                    //right: Val::Px(15.),
                    left: Val::Px(CELL_OFFSET),
                    ..default()
                },
                ..Default::default()
            }),
        )
        .insert(Info);
}

fn update_info_system(
    mut query: Query<&mut Text, With<Info>>,
    player_query: Query<&Player>,
    level: Res<Field>,
) {
    let player_info = match player_query.get_single() {
        Ok(player) => format!(
            "Health: {}, Last hit: {}",
            player.health,
            player.last_hit.elapsed().as_millis()
        ),
        Err(_) => "Can't get player info.".to_string(),
    };

    let level_info = format!(
        "Level: {}",
        level.current_level
    );

    match query.get_single_mut() {
        Ok(mut text) => text.sections[0].value = format!("{}\n{}", player_info, level_info),
        Err(error) => error!("Error while updating debug info: {}", error.to_string()),
    }
}
