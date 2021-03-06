use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entity, World};
use amethyst::prelude::Builder;

use super::png_mesh_and_mat;
use crate::components::Player;
use crate::config::{ARENA_HEIGHT, ARENA_WIDTH};

const PLAYER_HEIGHT: f32 = 64.0;
const PLAYER_WIDTH: f32 = 64.0;

pub fn initialise_player(world: &mut World) -> Entity {
    let mut trans = Transform::default();
    let (mesh, material) = png_mesh_and_mat("player.png", [PLAYER_WIDTH, PLAYER_HEIGHT], world);

    //initialize the player to the center of the arena
    trans.set_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

    //build an entity that is a player and has transform and sprite components
    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(Player::new())
        .with(trans)
        .build()
}
