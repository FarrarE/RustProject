use amethyst::prelude::Builder;
use amethyst::ecs::prelude::{Entity, World};
use amethyst::core::transform::{Transform};

use crate::components::Enemy;
use super::png_mesh_and_mat;
use crate::config::{ARENA_HEIGHT, ARENA_WIDTH};


const ENEMY_HEIGHT: f32 = 50.0;
const ENEMY_WIDTH: f32 = 50.0;

pub fn initialise_enemy(world: &mut World) -> Entity {
    let mut trans = Transform::default();
    let (mesh, material) = png_mesh_and_mat(
        "HappyPie.png",
        [ENEMY_WIDTH, ENEMY_HEIGHT],
        world);

    //initialize the enemy to the center of the arena
    trans.set_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);


    //build an entity that is a player and has transform and sprite components
    world.create_entity()
        .with(mesh)
        .with(material)
        .with(Enemy::new())
        .with(trans)
        .build()
}