use bevy::prelude::*;

use crate::{
    schedule::InGameSet,
    health::Health,
    state::*,
};

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, 
            (
                despawn_far_away_entities,
                despawn_dead_entity
            )
                .in_set(InGameSet::DespawnEntities)
        )
        .add_systems(OnEnter(GameState::GameOver), despawn_all_entity);
    }
}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        // Entity is far away from the camera's viewport.
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_dead_entity(
    mut commands: Commands,
    query: Query<(Entity, &Health)>
) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_all_entity(
    mut commands: Commands,
    query: Query<Entity, With<Health>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
