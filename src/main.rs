use bevy::{prelude::*, render::camera::CameraPlugin, window::WindowMode};
use collision::CollisionPlugin;

mod game;
mod debug;
mod ball;
mod movement;
mod paddle;
mod collision;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(
                Window{mode: WindowMode::BorderlessFullscreen, ..default()}
            ),
            ..default()
        }))
        .add_plugins(CollisionPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins()
        .run();
}




/*
    Top Down Melee Arena
    Parrying? Dodging? Stamina?
    Environmental Obstacles -> Push enemies into them?
    Different Weapons -> Special Attack?
    Different AI patterns -> Just rush, Wait for allies, Flank
    Blood
    
    Extension: Roguelite elements
*/