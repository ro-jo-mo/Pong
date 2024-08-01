use bevy::{prelude::*, transform::commands};

const GAME_WIDTH: f32 = 640.0;
const GAME_HEIGHT: f32 = 360.0;

pub struct GamePlugin;
impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);    
    }
}

#[derive(Resource)]
pub struct GameSize{
    pub width: f32,
    pub height: f32
}

fn startup(
    mut commands: Commands
){
    commands.insert_resource(GameSize{width: GAME_WIDTH, height: GAME_HEIGHT});
}