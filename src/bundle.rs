use amethyst::core::bundle::{SystemBundle, Result};
use amethyst::ecs::prelude::{DispatcherBuilder};

use crate::systems::*;


/// A bundle is a convenient way to initialise related resources, components and systems in a
/// world.
///
/// Note that all components and resources need to be registered in the world before they
/// can be referenced in a system or in the state.
///
/// This bundle prepares the world for the space_shooter game.
pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(PlayerSystem, "player_system", &["input_system"]);
        builder.add(ProjectileSystem, "projectile_system", &[]);
        builder.add(EnemySystem, "enemy_system", &[]);
        builder.add(SpawnSystem, "spawn_system", &[]);
        // builder.add(LaserSystem, "laser_system", &["ship_system"]);
        builder.add(ProjectileCollisionSystem, "projectile_collision_system", &["projectile_system"]);
        builder.add(PlayerCollisionSystem, "player_collision_system", &[]);
        // builder.add(LivesSystem, "lives_system", &["collision_system"]);
        Ok(())
    }
}