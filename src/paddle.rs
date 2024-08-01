use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use crate::{collision::{CircleCollider, RectangleCollider}, game::GameSize};

const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_WIDTH: f32 = 50.0;

#[derive(Component)]
pub struct Paddle{
    pub movement: f32,
    pub rotation: f32
}
impl Paddle{
    const fn new() -> Paddle {Paddle {movement: 0.0, rotation: 0.0}}
}

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin{
    fn build(&self, app: &mut App) {
        app .add_systems(PostStartup, spawn_paddles)
            .add_systems(Update, move_paddle);
    }
}
#[derive(Bundle)]
pub struct PaddleBundle{
    paddle: Paddle,
    collider: RectangleCollider,
    mesh: MaterialMesh2dBundle<ColorMaterial>
}

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    size: Res<GameSize>
){
    commands.spawn(
        PaddleBundle{
            paddle: Paddle::new(),
            collider: RectangleCollider::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            mesh: MaterialMesh2dBundle{
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(size.width / 2.0, 0.0, 0.0),
                ..default()
            }
        }
    );
    commands.spawn(
        PaddleBundle{
            paddle: Paddle::new(),
            collider: RectangleCollider::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            mesh: MaterialMesh2dBundle{
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(-size.width / 2.0, 0.0, 0.0),
                ..default()
            }
        }
    );
}

fn move_paddle(
    mut query: Query<(&mut Transform,&Paddle)>,
    time: Res<Time>
){
    for (mut trans, paddle) in query.iter_mut(){
        trans.translation += Vec3::Y * paddle.movement * time.delta_seconds();
        trans.rotate_z(paddle.rotation * time.delta_seconds())
    }
}