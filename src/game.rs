use bevy::{prelude::*, transform::commands};

const GAME_WIDTH: f32 = 1760.0;
const GAME_HEIGHT: f32 = 1000.0;

pub struct GamePlugin;
impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app .add_systems(Startup, startup)
            .add_systems(Update, quit);    

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

fn quit(
    input: Res<ButtonInput<KeyCode>>,
    mut writer: EventWriter<AppExit>
){
    if input.pressed(KeyCode::Escape){writer.send(AppExit::Success);}
}