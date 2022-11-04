use bevy::{ecs::query::QuerySingleError, prelude::*};

use crate::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

#[derive(Inspectable, Component, Clone, Copy, Debug)]
pub enum TowerType {
    Tomato,
    Potato,
    Cabbage,
}

#[derive(Component)]
pub struct TowerUIRoot;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .add_system(tower_shooting)
            .add_system(tower_button_clicked)
            .add_system(create_ui_on_selection);
    }
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &TowerType, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (tower_ent, mut tower, tower_type, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closest_target| closest_target.translation() - bullet_spawn);

            if let Some(direction) = direction {
                let (model, bullet) = tower_type.get_bullet(direction, &bullet_assets);
                commands.entity(tower_ent).with_children(|commands| {
                    commands
                        .spawn_bundle(SceneBundle {
                            scene: model,
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..Default::default()
                        })
                        .insert(Lifetime {
                            timer: Timer::from_seconds(10.0, false),
                        })
                        .insert(bullet)
                        .insert(Name::new("Bullet"));
                });
            }
        }
    }
}

impl TowerType {
    fn get_tower(&self, assets: &GameAssets) -> (Handle<Scene>, Tower) {
        match self {
            TowerType::Tomato => (
                assets.tomato_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.5, true),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                },
            ),
            TowerType::Potato => (
                assets.tomato_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.1, true),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                },
            ),
            TowerType::Cabbage => (
                assets.tomato_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.8, true),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                },
            ),
        }
    }

    fn get_bullet(&self, direction: Vec3, assets: &GameAssets) -> (Handle<Scene>, Bullet) {
        match self {
            TowerType::Tomato => (
                assets.tomato_scene.clone(),
                Bullet {
                    direction,
                    speed: 3.5,
                },
            ),
            TowerType::Potato => (
                assets.tomato_scene.clone(),
                Bullet {
                    direction,
                    speed: 6.5,
                },
            ),
            TowerType::Cabbage => (
                assets.tomato_scene.clone(),
                Bullet {
                    direction,
                    speed: 1.5,
                },
            ),
        }
    }
}

fn spawn_tower(
    commands: &mut Commands,
    assets: &GameAssets,
    position: Vec3,
    tower_type: TowerType,
) -> Entity {
    let (tower_scene, tower) = tower_type.get_tower(assets);
    commands
        .spawn_bundle(SpatialBundle::from_transform(Transform::from_translation(
            position,
        )))
        .insert(Name::new(format!("{:?}_Tower", tower_type)))
        .insert(tower_type)
        .insert(tower)
        .with_children(|commands| {
            commands.spawn_bundle(SceneBundle {
                scene: tower_scene,
                transform: Transform::from_xyz(0.0, -0.8, 0.0),
                ..Default::default()
            });
        })
        .id()
}

fn tower_button_clicked(
    interaction: Query<(&Interaction, &TowerType), Changed<Interaction>>,
    mut commands: Commands,
    selection: Query<(Entity, &Selection, &Transform)>,
    assets: Res<GameAssets>,
) {
    for (interaction, tower_type) in &interaction {
        if matches!(interaction, Interaction::Clicked) {
            for (entity, selection, transform) in &selection {
                if selection.selected() {
                    //Remove the base model/hitbox
                    commands.entity(entity).despawn_recursive();

                    spawn_tower(&mut commands, &assets, transform.translation, *tower_type);
                }
            }
        }
    }
}

fn create_ui(commands: &mut Commands, asset_server: &AssetServer) {
    let button_icons = [
        asset_server.load("tomato_tower.png"),
        asset_server.load("potato_tower.png"),
        asset_server.load("cabbage_tower.png"),
    ];

    let towers = [TowerType::Tomato, TowerType::Potato, TowerType::Cabbage];
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(TowerUIRoot)
        .with_children(|commands| {
            for i in 0..3 {
                commands
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Percent(15.0 * 9.0 / 16.0), Val::Percent(15.0)),
                            align_self: AlignSelf::FlexStart,
                            margin: UiRect::all(Val::Percent(2.0)),
                            ..default()
                        },
                        image: button_icons[i].clone().into(),
                        ..default()
                    })
                    .insert(towers[i]);
            }
        });
}

fn create_ui_on_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //Perf could probably be smarter with change detection
    selections: Query<&Selection>,
    root: Query<Entity, With<TowerUIRoot>>,
) {
    let at_least_one_selected = selections.iter().any(|selection| selection.selected());
    match root.get_single() {
        Ok(root) => {
            if !at_least_one_selected {
                commands.entity(root).despawn_recursive();
            }
        }
        //No root exist
        Err(QuerySingleError::NoEntities(..)) => {
            if at_least_one_selected {
                create_ui(&mut commands, &asset_server);
            }
        }
        _ => unreachable!("Too many ui tower roots!"),
    }
}
