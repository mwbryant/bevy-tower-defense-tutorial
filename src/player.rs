use bevy::prelude::*;

use crate::{GameState, TargetDeathEvent};

// Could be a resource
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    pub money: u32,
    pub health: u32,
}

#[derive(Component)]
pub struct GamePlayUIRoot;

#[derive(Component)]
pub struct HealthUI;

#[derive(Component)]
pub struct MoneyUI;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_system_set(
                SystemSet::on_enter(GameState::Gameplay)
                    .with_system(spawn_player)
                    .with_system(spawn_gameplay_ui),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(give_money_on_kill)
                    .with_system(update_player_ui),
            );
    }
}

fn update_player_ui(
    player: Query<&Player>,
    // The without here prevents queries from potentially matching the same text component (if one text entity had both ui comps)
    mut money_ui: Query<&mut Text, (With<MoneyUI>, Without<HealthUI>)>,
    mut health_ui: Query<&mut Text, With<HealthUI>>,
) {
    //Won't panic: There must be 1 and only 1 of each of these entities
    let player = player.single();
    let mut money = money_ui.single_mut();
    let mut health = health_ui.single_mut();

    *money = Text::from_section(
        format!("Money: {}", player.money),
        money.sections[0].style.clone(),
    );
    *health = Text::from_section(
        format!("Health: {}", player.health),
        health.sections[0].style.clone(),
    );
}

fn spawn_gameplay_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(GamePlayUIRoot)
        .with_children(|commands| {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::FlexStart,
                        align_self: AlignSelf::FlexStart,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|commands| {
                    commands
                        .spawn(TextBundle {
                            style: Style {
                                margin: UiRect::all(Val::Percent(1.2)),
                                ..default()
                            },
                            text: Text::from_section(
                                "Player Money: XX",
                                TextStyle {
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    font_size: 36.0,
                                    color: Color::BLACK,
                                },
                            ),
                            ..default()
                        })
                        .insert(MoneyUI);
                    commands
                        .spawn(TextBundle {
                            style: Style {
                                margin: UiRect::all(Val::Percent(1.2)),
                                ..default()
                            },
                            text: Text::from_section(
                                "Player Health: XX",
                                TextStyle {
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    font_size: 36.0,
                                    color: Color::BLACK,
                                },
                            ),
                            ..default()
                        })
                        .insert(HealthUI);
                });
        });
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player {
            money: 100,
            health: 10,
        },
        Name::new("Player"),
    ));
}

fn give_money_on_kill(
    mut player: Query<&mut Player>,
    mut death_events: EventReader<TargetDeathEvent>,
) {
    let mut player = player.single_mut();
    for _event in death_events.iter() {
        player.money += 10;
    }
}
