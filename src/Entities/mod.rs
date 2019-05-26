pub mod player;
pub mod projectile;
pub mod camera;

use amethyst::ecs::prelude::World;
use amethyst::renderer::{PngFormat, Texture, TextureMetadata, PosTex, Mesh, Material, MaterialDefaults};
use amethyst::assets::{AssetStorage, Loader, Handle};
use amethyst::core::nalgebra::{Vector2, Vector3};

//pub use self::entity??::{methodnames} //to make methods available elsewhere 
pub use self::projectile::fire_projectile;


pub fn init_entities(world: &mut World) {
    player::initialise_player(world);
    camera::initialise_camera(world);
    projectile::initialise_projectile_resource(world);
}

pub fn png_mesh_and_mat(name: &'static str, png_sz: [f32; 2], world: &mut World) -> (Handle<Mesh>, Material) {
    let loader = world.read_resource::<Loader>();
    
    let albedo = loader.load(
        name,
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    );

    let mat_defaults = world.read_resource::<MaterialDefaults>();

    let material = Material {
        albedo,
        ..mat_defaults.0.clone()
    };

    let vertices = create_png_vertices(0.0, 0.0,png_sz[0],png_sz[1]);

    let mesh = loader.load_from_data(
        vertices.into(),
        (),
        &world.read_resource::<AssetStorage<Mesh>>());
    (mesh, material)
}

pub fn create_png_vertices(left: f32, bottom: f32, right:f32, top:f32) -> Vec<PosTex> {
    vec![
        PosTex {
            position: Vector3::new(left, bottom, 0.0),
            tex_coord: Vector2::new(0.0, 0.0),
        },
        PosTex {
            position: Vector3::new(right, bottom, 0.0),
            tex_coord: Vector2::new(1.0, 0.0),
        },
        PosTex {
            position: Vector3::new(left, top, 0.0),
            tex_coord: Vector2::new(0.0, 1.0),
        },
        PosTex {
            position: Vector3::new(right, top, 0.0),
            tex_coord: Vector2::new(1.0, 1.0),
        },
        PosTex {
            position: Vector3::new(left, top, 0.),
            tex_coord: Vector2::new(0.0, 1.0),
        },
        PosTex {
            position: Vector3::new(right, bottom, 0.0),
            tex_coord: Vector2::new(1.0, 0.0),
        },
    ]
}