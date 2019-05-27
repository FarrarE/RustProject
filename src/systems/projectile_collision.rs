use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entities, Join, System, ReadStorage, WriteStorage};

use crate::components::Projectile;
use crate::components::Enemy;

pub struct ProjectileCollisionSystem;

impl<'s> System<'s> for ProjectileCollisionSystem {

    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Projectile>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
    );

    fn run(&mut self, (entities, projectiles, transforms, mut enemies): Self::SystemData) {

      // For each projectile
      for (projectile_entity, enemy_entity, projectile_component, projectile_transform) in (&*entities, &*entities, &projectiles, &transforms).join() {
          
          // Defines collision box for projectile
          let projectile_left = projectile_transform.translation()[0];
          let projectile_right = projectile_left + projectile_component.width;
          let projectile_top = projectile_transform.translation()[1] + projectile_component.height;
          let projectile_bottom = projectile_transform.translation()[1];


          // Checks to see if an enemy overlaps with a projectile
          for (enemy_component, enemy_transform) in (&mut enemies, &transforms).join() {
              // Set up a collision box for our enemy
              let enemy_left = enemy_transform.translation()[0];
              let enemy_bottom = enemy_transform.translation()[1];
              let enemy_top = enemy_transform.translation()[1] +  enemy_component.height;
              let enemy_right = enemy_left + enemy_component.width;

              // If the two items overlap,
              if ((projectile_left <= enemy_right && projectile_left >= enemy_left)
                    || 
                    (projectile_right <= enemy_left && projectile_right >= enemy_right))
                    && ((projectile_top >= enemy_bottom && projectile_bottom <= enemy_top) 
                    ||
                    (enemy_top >= projectile_bottom && enemy_bottom <= projectile_top)
                    ){
                  // we have a collision. Delete the projectile
                  //let _result = entities.delete(projectile_entity);
                  // let the enemy system know the enemy is ready for respawn/relocation
                  let _result = entities.delete(enemy_entity);
                }
            }
        }
    }
}