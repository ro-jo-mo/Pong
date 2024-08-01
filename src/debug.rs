use std::default;

use bevy::prelude::*;


#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
enum DebugState{
    #[default]
    Disabled,
    Enabled
}
pub struct DebugPlugin;
impl Plugin for DebugPlugin{
    fn build(&self, app: &mut App) {
        app.init_state::<DebugState>();
    }
}

