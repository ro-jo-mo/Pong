use std::f32::consts::PI;

use bevy::{math::VectorSpace, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, transform::commands};
use rand::{thread_rng, Rng};
use crate::{collision::{CircleCollider, CollisionEvent, CollisionPlugin}, schedule::GameSet};

const BALL_SPEED: f32 = 400.0;
const BALL_SIZE: f32 = 35.0;

pub struct BallPlugin;
impl Plugin for BallPlugin{
    fn build(&self, app: &mut App) {
        app .add_systems(Startup, spawn_ball)
            .add_systems(Update, (
                bounce_on_collision,
                move_ball
            ).chain().in_set(GameSet::EntityUpdates));
    }
}
#[derive(Component)]
struct Velocity{
    pub value: Vec3
}

#[derive(Bundle)]
struct BallBundle{
    collider: CircleCollider,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    velocity: Velocity
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let mut rng = thread_rng();
    
    let velocity = Velocity {
        value: /*Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rng.gen_range(-PI / 4.0..PI / 4.0)) * */Vec3::new(-1.0,0.0,0.0) * BALL_SPEED
    };

    commands.spawn(BallBundle{
        collider: CircleCollider {radius: BALL_SIZE},
        mesh: MaterialMesh2dBundle{
            mesh: Mesh2dHandle(meshes.add(Circle {radius: BALL_SIZE})),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        velocity
    });
}

fn move_ball(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>
){
    for (mut trans, vel) in query.iter_mut(){
        trans.translation += vel.value * time.delta_seconds();
    }
}

fn bounce_on_collision(
    mut reader: EventReader<CollisionEvent>,
    mut query: Query<&mut Velocity>
){
    for collision in reader.read(){
        let Ok(mut velocity) = query.get_mut(collision.b) else {continue;};
        velocity.value = collision.normal * BALL_SPEED;
    }
}