use noise::OpenSimplex;
use noise::Seedable;
use std::cmp;
use bevy::math::DVec2;

#[derive(Resource)]
#[derive(Debug)]
pub struct Heightmap {
    pub data: Vec<f32>,
    pub size: usize,
}

pub struct HeightmapPlugin;

#[derive(Component)]
struct MeshResource {
    handle: Handle<Mesh>,
}



#[derive(Component)]
pub struct SideTerrain {
    pub length: usize,
    heightmap: Vec<f32>
}


impl Plugin for HeightmapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Heightmap {
            data: vec![0.0; 50000],
            size: 50000,
        })
            .add_system(manipulate_heightmap);
    }
}


fn manipulate_heightmap(mut commands: Commands, mut heightmap: ResMut<Heightmap>,
                        keyboard_input: Res<Input<KeyCode>>,
                        mut meshes: ResMut<Assets<Mesh>>,
                        query: Query<&MeshResource>){
    let scan_radius = 1;
    // println!("{:?}", heightmap);
    if keyboard_input.just_pressed(KeyCode::Space) {
        for i in 0..heightmap.size {
            let height = heightmap.data[i];

            let mut height_sum = 0.0;
            let mut height_count = 0.0;

            for n in (i as isize - scan_radius as isize)..=(i as isize + scan_radius as isize) {
                if n >= 0 && n < heightmap.size as isize {
                    let height_of_neighbour = heightmap.data[n as usize];

                    height_sum += height_of_neighbour;
                    height_count += 1.0;
                }
            }

            let height_average = height_sum / height_count;
            heightmap.data[i] = height_average;
            // transform_go(i);


        }
        // println!("{:?}", heightmap.data);

        for mesh_resource in query.iter() {
            if let Some(mut mesh) = meshes.get_mut(&mesh_resource.handle) {
                let mut positions: Vec<[f32; 3]> = Vec::new();
                let mut indices: Vec<u32> = Vec::new();
                for i in 0..heightmap.size {
                    let h = heightmap.data[i];

                    positions.push([i as f32 + 0.0, 0.0, 0.0]); //lower left - at index 0
                    positions.push([i as f32 + 1.0, 0.0, 0.0]); //lower right - at index 1
                    positions.push([i as f32 + 0.0, h, 0.0]); //upper left - at index 2
                    positions.push([i as f32 + 1.0, h, 0.0]); //upper right - at index 3

                    // // Add triangle indices
                    let index_base: u32 = i as u32 * 4;
                    indices.push(index_base + 0);
                    indices.push(index_base + 1);
                    indices.push(index_base + 2);

                    indices.push(index_base + 2);
                    indices.push(index_base + 1);
                    indices.push(index_base + 3);
                }

                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    positions,
                );
                mesh.set_indices(Some((bevy::render::mesh::Indices::U32(indices))));
            }
        }

    }
}




fn terrain_spawn(mut commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<StandardMaterial>>,
                 mut custom_materials: ResMut<Assets<CustomMaterial>>,
                 mut materials2: ResMut<Assets<NormalMaterial>>,
                 mut heightmap: ResMut<Heightmap>,
                 images: Res<ImageAssets>,
                 asset_server: Res<AssetServer>
){
    let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::LineList);
    // println!("{:?}", heightmap);
    build_mesh(&mut heightmap, &mut mesh);

    // println!("{:?}", heightmap);
    let mesh_handle = meshes.add(mesh);
    let new_mesh_handle =mesh_handle.clone();
    let m = &meshes.get(&new_mesh_handle);
    commands.spawn((MaterialMeshBundle {
        mesh: mesh_handle,
        material: materials2.add(NormalMaterial::default()),
        transform: Transform::from_xyz(-3100.0, -100.0, 0.0),
        ..default()
    }, RigidBody::Fixed ))
        .insert(Terrain)
        .insert(MeshResource { handle:new_mesh_handle }).insert(
        Collider::from_bevy_mesh(m.unwrap(),
                                 &ComputedColliderShape::TriMesh).unwrap())
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS);
    // Collider::heightfield( heightmap.data.clone(), 2, 25000, Vec3::new(0.0,0.0,0.0) )


}

pub fn build_mesh(heightmap: &mut ResMut<Heightmap>, mesh: &mut Mesh) {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(10);


    let mut landing = Landing::new(10000);
    let seed: u64 = 142;
    let height: i32 = 100;
    let width: i32 = 50000;
    let start: f32 = 30.0;
    let end: f32 = 1000.0;
    let r: bool = false;

    // let terrain = landing.generate_exact_terrain(seed, height, width, start, end, r);
    let new_terrain = landing.setup_data();
    // println!("terrain is:: {:?}", new_terrain);

    let mut temp = 0.0;
    for i in 0..heightmap.size {
        let random = rng.gen_range(1..heightmap.size);
        if i+1 < new_terrain.len(){
            heightmap.data[i] = new_terrain[i][1] as f32;
            let h = heightmap.data[i];

            // Define the start and end points of the line segment
            let new_i = new_terrain[i][0];
            positions.push([new_i as f32, h, 0.0]);     // Start point
            positions.push([(new_i + 1.0) as f32, new_terrain[i + 1][1] as f32, 0.0]); // End point

            // Add line indices
            let index_base: u32 = i as u32 * 2;
            indices.push(index_base + 0);
            indices.push(index_base + 1);
        }

    }

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        positions,
    );
    mesh.set_indices(Some((bevy::render::mesh::Indices::U32(indices))));

}









