use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Read, System, WriteExpect, LazyUpdate, Entity, ReadExpect},
};
use rand::Rng;

use crate::entities::ENEMY_HEIGHT;
use crate::config::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::config::GAME_CONFIGURATION;
use crate::resources::EnemyResource;

// Allows System to be accessed publicaly 
pub struct SpawnSystem;

impl<'s> System<'s> for SpawnSystem {
    type SystemData = (
      Entities<'s>,
      WriteExpect<'s, EnemyResource>,
      Read<'s, Time>,
      ReadExpect<'s, LazyUpdate>,
    );

    //The order of items in the SystemData type needs to match the order of the names in the tuple argument for run()

    fn run(&mut self, (entities, mut enemy_resource, time, lazy_update): Self::SystemData) {
        let mut rng = rand::thread_rng();

        if enemy_resource.time_till_next_spawn > 0.0 {
            enemy_resource.time_till_next_spawn -= time.delta_seconds();
        }

        if enemy_resource.time_till_next_spawn <= 0.0 &&
            enemy_resource.num_enemies < GAME_CONFIGURATION.max_monster_count 
        {
            let enemy_entity: Entity = entities.create();

            let mut trans = Transform::default();

            //add a small random element to the speed of each pie when it is created
            let mut enemy_component = enemy_resource.component.clone();
            enemy_component.velocity += rng.gen_range(0.0, 10.0);

            //edge numbers start with edge 0 on the left of the screen and go clockwise
            let location = rng.gen_range(0.0, ARENA_HEIGHT - ENEMY_HEIGHT);
            let edge = rng.gen_range(0, 4);
            
            //println!("EDGE: {}", edge);
            match edge {
                0 => {trans.set_xyz(0.0, location, 0.0);},
                1 => {trans.set_xyz(location, ARENA_HEIGHT - enemy_component.height, 0.0);},
                2 => {trans.set_xyz(ARENA_WIDTH - enemy_component.width, location, 0.0);},
                3 => {trans.set_xyz(location, 0.0, 0.0);},
                _ => {},
            }
            
            

            lazy_update.insert(enemy_entity, enemy_resource.material.clone());
            lazy_update.insert(enemy_entity, enemy_resource.mesh.clone());
            lazy_update.insert(enemy_entity, enemy_component);
            lazy_update.insert(enemy_entity, trans);
            enemy_resource.num_enemies += 1;
            enemy_resource.time_till_next_spawn = GAME_CONFIGURATION.monster_spawn_delay;
        }
        
    }
}