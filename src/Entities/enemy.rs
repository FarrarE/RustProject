use amethyst::prelude::Builder;
use amethyst::ecs::prelude::{Entity, World};
use amethyst::core::transform::{Transform};


use crate::components::Enemy as EnemyComponent;
use crate::resources::EnemyResource;
use super::png_mesh_and_mat;
use crate::config::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::config::GAME_CONFIGURATION;


const ENEMY_HEIGHT: f32 = 50.0;
const ENEMY_WIDTH: f32 = 50.0;

pub fn initialise_enemy_resource(world: &mut World) -> EnemyResource {
    let (mesh, material) = png_mesh_and_mat("HappyPie.png", [ENEMY_WIDTH, ENEMY_HEIGHT], world);

    let enemy_resource = EnemyResource {
        mesh,
        material,
        component: EnemyComponent {
            velocity: GAME_CONFIGURATION.monster_velocity,
            width: ENEMY_WIDTH,
            height: ENEMY_HEIGHT,
            
        },
        num_enemies: 0,
    };
    world.add_resource(enemy_resource.clone());
    enemy_resource
}