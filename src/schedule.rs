use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameSet {
    UserInput,
    EntityUpdates,
    CollisionDetection,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameSet::UserInput,
                GameSet::EntityUpdates,
            )
                .chain())
            .configure_sets(PostUpdate, GameSet::CollisionDetection.after(TransformSystem::TransformPropagate));
    }
}
