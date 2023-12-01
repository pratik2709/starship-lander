pub struct Background;


fn n_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Quad{
            size: Vec2::new(10000.0,5000.0),
            ..default()
        })),
        transform: Transform::from_xyz(-500.0, 250.0, -500.0),
        material: materials.add(CustomMaterial {
            color: Color::GREEN
        }),
        ..default()
    });
}



impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

// This is the struct that will be passed to your shader
#[derive(TypePath, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color
}
