use bevy::prelude::*;

use crate::schedule::GameSet;

#[derive(Component)]
pub struct RectangleCollider{
    //height of the rectangle, along the y-axis
    pub height: f32,
    //width of the rectangle, along the x-axis
    pub width: f32
}
impl RectangleCollider{
    pub const fn new(width: f32, height: f32) -> RectangleCollider {RectangleCollider {width, height}}
}

#[derive(Component)]
pub struct CircleCollider{
    pub radius: f32
}
impl CircleCollider{
    pub const fn new(radius: f32) -> CircleCollider {CircleCollider {radius}}
}
#[derive(Event)]
pub struct CollisionEvent{
    pub normal: Vec3,
    pub a: Entity,
    pub b: Entity
}
impl CollisionEvent{
    pub fn new (normal: Vec3,a: Entity,b: Entity) -> Self {CollisionEvent{normal,a,b}}
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin{
    fn build(&self, app: &mut App) {
        app .add_event::<CollisionEvent>()
            .add_systems(PostUpdate, circle_rectangle_collision.in_set(GameSet::CollisionDetection));
    }
}



fn circle_rectangle_collision(
    circle_query:        Query<(Entity,&GlobalTransform,&CircleCollider)>,
    rectangle_query:     Query<(Entity,&GlobalTransform,&RectangleCollider)>,
    mut event_writer:   EventWriter<CollisionEvent>
){
    for (rect, rect_trans, rect_coll) in rectangle_query.iter(){
        for (circle, circle_trans, circle_coll) in circle_query.iter(){            
            let (_,rotation,_) = rect_trans.to_scale_rotation_translation();
            
            let rel_circle_pos = rotation.inverse() * circle_trans.translation();
            let rel_rect_pos = rotation.inverse() * rect_trans.translation();
            
            let to_right = rel_rect_pos.x < rel_circle_pos.x;
            let above = rel_rect_pos.y < rel_circle_pos.y;

            let in_x_bounds = rel_circle_pos.x <= rel_rect_pos.x + rect_coll.width  / 2.0
                                 && rel_circle_pos.x >= rel_rect_pos.x - rect_coll.width  / 2.0;
            let in_y_bounds = rel_circle_pos.y <= rel_rect_pos.x + rect_coll.height / 2.0
                                 && rel_circle_pos.y >= rel_rect_pos.x - rect_coll.height / 2.0;

            let normal = 
                match (in_x_bounds,in_y_bounds){
                    (true,true) => Some((rel_circle_pos - rel_rect_pos).normalize()),//collision has occured
                    (true, false) => {
                        let normal = if above {Vec3::Y} else {Vec3::NEG_Y};
                        line_collision(normal,rel_circle_pos,rel_rect_pos,circle_coll,rect_coll)
                    },
                    (false, true) => {
                        let normal = if to_right {Vec3::X} else {Vec3::NEG_X};
                        line_collision(normal,rel_circle_pos,rel_rect_pos,circle_coll,rect_coll)
                    },
                    (false, false) => corner_collision(above, to_right, rel_circle_pos, rel_rect_pos, circle_coll, rect_coll),
                };
            
            if let Some(normal) = normal{
                event_writer.send(CollisionEvent::new(normal,rect,circle));
                println!("{:?},{:?}",in_x_bounds,in_y_bounds);
                println!("{:?} <= {:?} + {:?}  / 2.0 && {:?} >= rel_rect_pos.x - {:?}  / 2.0", rel_circle_pos.x, rel_rect_pos.x, rect_coll.width, rel_circle_pos.x, rect_coll.width)
            }
        }
    }
}

fn line_collision(
    normal:         Vec3,
    rel_circle_pos: Vec3,
    rel_rect_pos:   Vec3,
    circle_coll:    &CircleCollider,
    rect_coll:      &RectangleCollider
) -> Option<Vec3>{
    let is_colliding = match normal {
        Vec3::X     => {(rel_circle_pos - normal * circle_coll.radius).x <= rel_rect_pos.x + rect_coll.width  / 2.0},
        Vec3::NEG_X => {(rel_circle_pos - normal * circle_coll.radius).x >= rel_rect_pos.x + rect_coll.width  / 2.0},
        Vec3::Y     => {(rel_circle_pos - normal * circle_coll.radius).y <= rel_rect_pos.x + rect_coll.height / 2.0},
        Vec3::NEG_Y => {(rel_circle_pos - normal * circle_coll.radius).y >= rel_rect_pos.x + rect_coll.height / 2.0},
        _           => {panic!("normal is fucked up")}
    };
    if !is_colliding {return None;}
    Some(normal)
}

fn corner_collision(
    above:          bool,
    to_right:       bool,
    rel_circle_pos: Vec3,
    rel_rect_pos:   Vec3,
    circle_coll:    &CircleCollider,
    rect_coll:      &RectangleCollider
) -> Option<Vec3>{
    let polarity = |x| if x {1.0} else {-1.0};

    let corner = 
        rel_rect_pos + Vec3::new(
            rect_coll.width  / 2.0 * polarity(to_right),
            rect_coll.height / 2.0 * polarity(above),0.0);
    
    if corner.distance_squared(rel_circle_pos) > circle_coll.radius * circle_coll.radius{return None;}
    Some((rel_circle_pos - corner).normalize())
}
