use bevy::prelude::*;

use crate::{GameState, TargetDeathEvent};

// Could be a resource
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    money: u32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay).with_system(give_money_on_kill),
            );
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(Player { money: 100 });
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
