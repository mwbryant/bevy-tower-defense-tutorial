use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{GameState, Player};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
    pub path_index: usize,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

#[derive(Resource)]
pub struct TargetPath {
    waypoints: Vec<Vec2>,
}

//Can have any data attached (i.e what kind of target or it's value)
pub struct TargetDeathEvent;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .add_event::<TargetDeathEvent>()
            //Could be loaded from a config or level file
            .insert_resource(TargetPath {
                waypoints: vec![
                    Vec2::new(6.0, 2.0),
                    Vec2::new(6.0, 6.0),
                    Vec2::new(9.0, 9.0),
                ],
            })
            .add_systems(
                (move_targets, hurt_player.after(move_targets), target_death)
                    .in_set(OnUpdate(GameState::Gameplay)),
            );
    }
}

fn target_death(
    mut commands: Commands,
    targets: Query<(Entity, &Health)>,
    mut death_event_writer: EventWriter<TargetDeathEvent>,
) {
    for (ent, health) in &targets {
        if health.value <= 0 {
            death_event_writer.send(TargetDeathEvent);
            commands.entity(ent).despawn_recursive();
        }
    }
}

fn hurt_player(
    mut commands: Commands,
    targets: Query<(Entity, &Target)>,
    path: Res<TargetPath>,
    mut player: Query<&mut Player>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for (entity, target) in &targets {
        if target.path_index >= path.waypoints.len() {
            commands.entity(entity).despawn_recursive();

            //Enemies reaching the end of their path could write an event to cause the player to take damage or play audio
            audio.play(asset_server.load("damage.wav"));

            let mut player = player.single_mut();
            if player.health > 0 {
                player.health -= 1;
            }

            if player.health == 0 {
                //TODO this could write an event or change the game state
                info!("GAME OVER");
            }
        }
    }
}

fn move_targets(
    mut targets: Query<(&mut Target, &mut Transform)>,
    path: Res<TargetPath>,
    time: Res<Time>,
) {
    for (mut target, mut transform) in &mut targets {
        let delta = target.speed * time.delta_seconds();
        let delta_target = path.waypoints[target.path_index] - transform.translation.xz();

        // This step will get us closer to the goal
        if delta_target.length() > delta {
            let movement = delta_target.normalize() * delta;
            transform.translation += movement.extend(0.0).xzy();
            //Copy for ownership reasons
            let y = transform.translation.y;
            transform.look_at(path.waypoints[target.path_index].extend(y).xzy(), Vec3::Y);
        } else {
            // At current step
            target.path_index += 1;
        }
    }
}
