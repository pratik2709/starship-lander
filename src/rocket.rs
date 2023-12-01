const ROCKET_SPRITE: &str = "lander.png";
const ROCKET_THRUST: &str = "fire_3.png";
const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
struct Rocket;

#[derive(Component)]
struct Plume;

#[derive(Component)]
struct Hexagon;

#[derive(Component)]
struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self(500.0)
    }
}

#[derive(Component)]
struct Thrust;

/// Represents a point with x and y coordinates
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Terrain;

#[derive(Component, Debug)]
struct TerrainRandom(f32, usize);

pub struct LunarModuleMovementPlugin;

#[derive(Resource, Debug)]
pub struct LunarModuleMovement {
    pub velocity: Vec3,
}

impl Plugin for LunarModuleMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LunarModuleMovement {
            velocity: Vec3::new(20.0 + rand::random::<f32>() * 20.0, 0.0, 0.0),
        });
        //.add_system(manipulate_heightmap);
    }
}

fn rocket_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform), (With<Rocket>)>,
    mut ext_forces: Query<&mut ExternalForce>,
    mut plume_query: Query<&mut Visibility, With<Plume>>,
    mut heightmap: ResMut<Heightmap>,
    mut lunar_movement: ResMut<LunarModuleMovement>,
    mut camera_query: Query<&mut Transform, (With<Camera>,  Without<Rocket>)>,
) {
    for (speed, mut transform) in query.iter_mut() {
            if keyboard_input.pressed(KeyCode::Left) {
                let mut angle = 0.0;
                if transform.rotation.z < 0.0 {
                    angle = transform.rotation.to_axis_angle().1;
                } else {
                    angle = std::f32::consts::TAU - transform.rotation.to_axis_angle().1;
                }
                let x = transform.translation.x + f32::sin(angle) * 5.0;
                let y = transform.translation.y + f32::cos(angle) * 5.0;
                let point = Vec3::new(x, y, transform.translation.z);
                let mut rotation = Quat::from_rotation_z(-1.0 * 2.0 * TIME_STEP);
                transform.translation = point + rotation * (transform.translation - point);
                transform.rotation *= rotation;


            } else if keyboard_input.pressed(KeyCode::Right) {
                let mut angle = 0.0;
                if transform.rotation.z < 0.0 {
                    angle = transform.rotation.to_axis_angle().1;
                } else {
                    angle = std::f32::consts::TAU - transform.rotation.to_axis_angle().1;
                }
                let x = transform.translation.x + f32::sin(angle) * 5.0;
                let y = transform.translation.y + f32::cos(angle) * 5.0;
                let point = Vec3::new(x, y, transform.translation.z);
                let mut rotation = Quat::from_rotation_z(1.0 * 2.0 * TIME_STEP);
                transform.translation = point + rotation * (transform.translation - point);
                transform.rotation *= rotation;


            } else if keyboard_input.pressed(KeyCode::W) {
                let forward_dir = transform.up();
                let mut force = forward_dir * 3.0;
                for mut ext_force in ext_forces.iter_mut()  {
                    ext_force.force = force;
                    for mut visible in plume_query.iter_mut() {
                        *visible = Visibility::Visible;
                    }
                }
            }
            else if keyboard_input.pressed(KeyCode::A) {
                transform.translation.x -= 1.0;
                // print_dat(*transform, heightmap.data.clone())
            }
            else if keyboard_input.pressed(KeyCode::D) {
                transform.translation.x += 1.0;

                // print_dat(*transform, heightmap.data.clone())
            }
            else {
                for mut ext_force in ext_forces.iter_mut()  {
                    ext_force.force = Vec3::new(0.0, 0.0, 0.0);
                    for mut visible in plume_query.iter_mut() {
                        *visible = Visibility::Hidden;
                    }
                }

            };

        let mut angle = 0.0;
        if transform.rotation.z < 0.0 {
            angle = transform.rotation.to_axis_angle().1;
        } else {
            angle = std::f32::consts::TAU - transform.rotation.to_axis_angle().1;
        }
        let x = transform.translation.x + f32::sin(angle) * 5.0;
        let y = transform.translation.y + f32::cos(angle) * 5.0;
        let point = Vec3::new(x, y, transform.translation.z);

        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation.x = point.x;
            camera_transform.translation.y = point.y;
        }


    }
}


fn display_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut rocket_query: Query<(&mut Transform), (With<Rocket>)>,
    mut velocities: Query<(&mut Velocity), (With<Rocket>)>,
    mut gravity: Query<(&mut GravityScale), (With<Rocket>)>,
    mut heightmap: ResMut<Heightmap>,
) {
    for collision_event in collision_events.iter() {
        // println!("Received collision event: {collision_event:?}");
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, event) => {
                // @TODO: Destroy the non-player entity
                for (mut transform) in rocket_query.iter_mut() {
                    for mut vel in velocities.iter_mut() {
                        for mut grav in gravity.iter_mut() {
                            vel.linvel = Vec3::new(0.0, 0.0, 0.0);
                            grav.0 = 0.0;
                            let shift: f32 = 168.0;
                            // println!("{:?}", (transform.translation.x + shift) as usize);
                            let mut bottom_left_x = ((transform.translation.x + shift) - 2.5) as usize;
                            let mut bottom_right_x = ((transform.translation.x + shift) + 2.5) as usize;
                            println!("Flat plane is is:: {:?}, {:?}", heightmap.data[bottom_left_x],
                                     heightmap.data[bottom_right_x]);
                            // println!("Rotation is:: {:?}", transform.rotation);
                            if (heightmap.data[bottom_left_x] == heightmap.data[bottom_right_x] ||
                                (heightmap.data[bottom_right_x] - heightmap.data[bottom_left_x]) <= 3.0) {
                                println!("stable plane found");
                                // check if speed of rocket is lesser than some value
                                // check if in upright position
                                // in that case create letters nice landing
                                // for mut vel in velocities.iter_mut() {
                                //     println!("Velocity is:: {:?}", vel);
                                //     println!("Rotation is:: {:?}", transform.rotation);
                                //     if vel.linvel.x < 20.0 && vel.linvel.y < -15.0{
                                //         // check if upright
                                //     }
                                // }
                            }
                            else{
                                // println!("Entities :: {:?} and {:?}", first_entity, second_entity);
                                commands.entity(*second_entity).despawn_recursive();

                            }
                        }
                    }
                }
                // println!("Collision started!!");

            }
            CollisionEvent::Stopped(first_entity, second_entity, event) => {
                // println!("Collision stopped!!");
                for mut grav in gravity.iter_mut() {
                    grav.0 = 1.5;
                }
            }
        }
    }
}
