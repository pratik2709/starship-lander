fn follow_rocket_system(
    rocket_transform: Query<(&Transform), With<Rocket>>,
    mut camera_query: Query<&mut Transform, (With<Camera>,  Without<Rocket>)>,
) {
    for (rocket) in rocket_transform.iter() {
        for mut camera_transform in camera_query.iter_mut() {
            // println!("{:?} and {:?}",  camera_transform.translation.z, rocket.translation.y);
            // zoom in
            if rocket.translation.y < 20.0  {
                    camera_transform.translation.z -= 5.0;
                    if camera_transform.translation.z <= 250.0{
                        camera_transform.translation.z = 250.0
                    }

            }
                //zoom out
            else if rocket.translation.y > 20.0 {
                    camera_transform.translation.z += 1.0;
                    camera_transform.translation.y += 1.0;
                    if camera_transform.translation.z > 500.0{
                        camera_transform.translation.z = 500.0
                    }
                    if camera_transform.translation.y > 100.0{
                        camera_transform.translation.y = 100.0
                    }
            }

        }
    }
}
