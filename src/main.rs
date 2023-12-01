use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    reflect::{TypePath},
    window::WindowResolution,
};

use bevy::asset::LoadState;
use bevy_sprite3d::*;
use bevy_asset_loader::prelude::*;
use bevy_normal_material::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::dynamics::RigidBodySet;
use bevy_rapier3d::rapier::geometry::ColliderSet;
use noise::NoiseFn;
use rand::Rng;

include!("rocket.rs");
include!("terrain.rs");
include!("landing.rs");
include!("background.rs");
include!("camera.rs");
use bevy::prelude::*;
use bevy::render::view::Visibility;
extern crate noise;
use noise::{Perlin};


#[derive(Resource)]
pub struct Materials {
    rocket_materials: Handle<Image>,
    thrust_materials: Handle<Image>,
}

#[derive(Resource)]
struct WinSize {
    w: f32,
    h: f32,
}

#[derive(Resource, Default)]
struct ImageAssets {
    image: Handle<Image>,
    plume: Handle<Image>
}


#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
enum GameState {
    #[default]
    Loading,
    Ready,
}

fn setup(mut commands: Commands, images: Res<ImageAssets>, mut sprite_params: Sprite3dParams) {

    let t = Transform::from_xyz(500., 200.0, 0.).with_scale(Vec3::new(1.0, 1.0, 0.01)).translation;

    let sprite_3d: Sprite3dBundle = Sprite3d {
        image: images.image.clone(),
        pixels_per_metre: 400.,
        partial_alpha: false,
        unlit: true,
        transform: Transform::from_xyz(50., 50., 0.).with_scale(Vec3::new(20.0, 20.0, 0.01)),
        // pivot: Some(Vec2::new(0.5, 0.5)),
        ..default()
    }.bundle(&mut sprite_params);

    let plume: Sprite3dBundle = Sprite3d {
        image: images.plume.clone(),
        pixels_per_metre: 400.,
        partial_alpha: false,
        unlit: true,
        transform: Transform::from_xyz(0., -2.0, 0.).with_scale(Vec3::new(1.0, 1.0, 0.01)),
        ..default()
    } .bundle(&mut sprite_params);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(t.x, t.y, 280.0),
        ..default()
    });

    let new_mesh_handle = &sprite_3d.pbr.mesh;
    let m = &sprite_params.meshes.get(new_mesh_handle);
    let coll = Collider::from_bevy_mesh(m.unwrap(), &ComputedColliderShape::TriMesh).unwrap();

    let img = &sprite_params.images.get(&images.image.clone()).unwrap();
    // println!("{:?}", img.texture_descriptor.size.width);

    commands
        .spawn(sprite_3d)
        .insert(Rocket)
        .insert(Speed::default())
        .insert(RigidBody::Dynamic)
        .insert(Sensor)
        .insert(GravityScale(1.5))
        .insert(Velocity::zero())
        .insert(ExternalForce {
           force: Vec3::new(0.0, 0.0, 0.0),
           torque: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Collider::cuboid(0.35, 0.35, 0.01))
        .insert(AdditionalMassProperties::Mass(0.1))
        .with_children(|parent|{
            parent.spawn(plume).insert(Plume).insert(
                Visibility::Hidden);
        });

    commands.insert_resource(WinSize {
        w: 1000.0,
        h: 500.0,
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1000.0, 500.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_startup_system(
            |asset_server: Res<AssetServer>, mut assets: ResMut<ImageAssets>| {
                assets.image = asset_server.load("lander.png");
                assets.plume = asset_server.load("fire_3.png");
            },
        )
        .add_system(
            (|asset_server: Res<AssetServer>,
              assets: Res<ImageAssets>,
              mut next_state: ResMut<NextState<GameState>>| {
                if asset_server.get_load_state(assets.image.clone()) == LoadState::Loaded {
                    next_state.set(GameState::Ready);
                }
            })
            .run_if(in_state(GameState::Loading)),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_state::<GameState>()
        .add_systems(OnEnter(GameState::Ready), setup)
        // .add_system( setup_background.in_schedule(OnEnter(GameState::Ready)) )
        .add_systems(OnEnter(GameState::Ready), terrain_spawn)
        .add_systems(OnEnter(GameState::Ready), n_setup)
        .add_systems(PostUpdate, rocket_movement)
        .add_systems(PostUpdate, display_events)
        .add_systems(PostUpdate, follow_rocket_system)
        .add_plugins(NormalMaterialPlugin)
        .add_plugins(HeightmapPlugin)
        .add_plugin(Sprite3dPlugin)
        .add_plugins(LunarModuleMovementPlugin)
        .insert_resource(ImageAssets::default())
        .run();
}
