use bevy::prelude::*;

pub struct Direction{
    pub value: f32
}

struct MovementPlugin;

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_paddle);
    }
}

fn move_paddle(){
    
}