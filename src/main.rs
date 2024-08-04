use ball::BallPlugin;
use bevy::{prelude::*, window::WindowMode};
use camera::CameraPlugin;
use collision::CollisionPlugin;
use debug::DebugPlugin;
use game::GamePlugin;
use input::InputPlugin;
use paddle::PaddlePlugin;
use schedule::SchedulePlugin;

mod game;
mod debug;
mod schedule;
mod ball;
mod paddle;
mod collision;
mod camera;
mod input;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(
                Window{mode: WindowMode::BorderlessFullscreen, ..default()}
            ),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(InputPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(SchedulePlugin)
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