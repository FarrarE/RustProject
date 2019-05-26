use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage, WriteExpect, LazyUpdate, Entity, ReadExpect},
};
use rand::Rng;

use crate::components::{Enemy};
use crate::config::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::config::GAME_CONFIGURATION;
use crate::resources::EnemyResource;

// Allows System to be accessed publicaly 
pub struct SpawnSystem;

impl<'s> System<'s> for SpawnSystem {
    type SystemData = (
      Entities<'s>,
      WriteStorage<'s, Enemy>,
      WriteStorage<'s, Transform>,
      WriteExpect<'s, EnemyResource>,
      Read<'s, Time>,
      ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut enemy, mut transforms, mut enemy_resource, time, lazy_update): Self::SystemData) {
        let mut rng = rand::thread_rng();
        if enemy_resource.num_enemies < GAME_CONFIGURATION.max_monster_count {
            let enemy_entity: Entity = entities.create();

            let mut trans = Transform::default();
            trans.set_xyz(rng.gen_range(0.0, 1000.0), rng.gen_range(0.0, 1000.0), 0.0);
            println!("Spawn at: {:?}", trans);
            lazy_update.insert(enemy_entity, enemy_resource.material.clone());
            lazy_update.insert(enemy_entity, enemy_resource.mesh.clone());
            lazy_update.insert(enemy_entity, enemy_resource.component.clone());
            lazy_update.insert(enemy_entity, trans);
            enemy_resource.num_enemies += 1;
        }
        
    }
}