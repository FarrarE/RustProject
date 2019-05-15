extern crate amethyst;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};
//use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

pub const ARENA_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: f32 = 1000.0;

pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        //remove once systems interact with it
        //world.register::<Player>();

        //initialize actors/components
        initialize_player(world, sprite_sheet_handle);
        initialize_camera(world);
    }
}

//The Player Component
pub struct Player {
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
}

impl Player {
    fn new() -> Player {
        Player {
            width: 64.0,
            height: 64.0,
            velocity: [100.0, 100.0],
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_player(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut trans = Transform::default();

    //initialize the player to the center of the arena
    trans.set_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

    //assign a sprite to the player
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 0,
    };

    //build an entity that is a player and has transform and sprite components
    world.create_entity()
        .with(sprite_render)
        .with(Player::new())
        .with(trans)
        .build();
}

// Projectile Component
pub struct Projectile {
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
}

impl Projectile {
    fn new() -> Projectile {
        Projectile {
            width: 10.0,
            height: 10.0,
            velocity: [150.0, 150.0],
        }
    }
}

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;
}


//Asset Loading
fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/player.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

//Create a camera entity
fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}