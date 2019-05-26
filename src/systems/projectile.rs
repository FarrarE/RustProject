use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Projectile};
use crate::config::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::config::GAME_CONFIGURATION;

// Allows System to be accessed publicaly 
pub struct ProjectileSystem;

impl<'s> System<'s> for ProjectileSystem {
    type SystemData = (
      Entities<'s>,
      ReadStorage<'s, Projectile>,
      WriteStorage<'s, Transform>,
      Read<'s, Time>,
    );

    fn run(&mut self, (entities, projectiles, mut transforms, time): Self::SystemData) {

      for (projectile_entity, projectile_component, projectile_transform) in (&*entities, &projectiles, &mut transforms).join() {

        projectile_transform.translate_y(projectile_component.rise * GAME_CONFIGURATION.projectile_velocity * time.delta_seconds());
        projectile_transform.translate_x(projectile_component.run * GAME_CONFIGURATION.projectile_velocity * time.delta_seconds());

        // Delete the projectile if it has gone off the screen
        if projectile_transform.translation()[1] > ARENA_HEIGHT || projectile_transform.translation()[1] < 0.0 {
            let _result = entities.delete(projectile_entity);
        }

        // Delete the projectile if it has gone off the screen
        else if projectile_transform.translation()[0] > ARENA_WIDTH || projectile_transform.translation()[0] < 0.0 {
            let _result = entities.delete(projectile_entity);
        }
      }
        
    }
}