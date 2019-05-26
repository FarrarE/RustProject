use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, is_key_down},
};

use crate::components::{Player, Projectile};
use crate::config::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::config::GAME_CONFIGURATION;

// Allows System to be accessed publicaly 
pub struct ProjectileSystem;

impl<'s> System<'s> for ProjectileSystem {
    type SystemData = (
      Entities<'s>,
      ReadStorage<'s, Projectile>,
      WriteStorage<'s, Transform>,
      Read<'s, InputHandler<String, String>>,
      Read<'s, Time>,
    );

    fn run(&mut self, (entities, projectiles, mut transforms, input, time): Self::SystemData) {

      for (projectile_entity, _projectile_component, projectile_transform) in (&*entities, &projectiles, &mut transforms).join() {

        projectile_transform.translate_y(rise * GAME_CONFIGURATION.projectile_velocity * time.delta_seconds());
        projectile_transform.translate_x(run * GAME_CONFIGURATION.projectile_velocity * time.delta_seconds());

        // Delete the laser if it has gone off the screen
        if projectile_transform.translation()[1] > 1024. {
            let _result = entities.delete(projectile_entity);
        }
      }
        
    }
}