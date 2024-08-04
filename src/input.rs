use bevy::prelude::*;

use crate::{paddle::Paddle, schedule::GameSet};

pub struct InputPlugin;
impl Plugin for InputPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,(
            player_input,
            ai_input
        ).in_set(GameSet::UserInput));
    }
}

#[derive(Component)]
pub struct PlayerInput;
#[derive(Component)]
pub struct AiInput;

fn player_input(
    mut query: Query<&mut Paddle>,
    input: Res<ButtonInput<KeyCode>>
){
    for mut paddle in query.iter_mut(){
        paddle.movement = (input.pressed(KeyCode::KeyW) as i32 + -1 * input.pressed(KeyCode::KeyS) as i32) as f32;
        paddle.rotation = (input.pressed(KeyCode::KeyA) as i32 + -1 * input.pressed(KeyCode::KeyD) as i32) as f32;
    }
}

fn ai_input(

){

}