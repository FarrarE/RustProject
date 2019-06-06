mod enemy;
mod play;
mod projectile;

pub use enemy::EnemyResource;
pub use play::PlayState;
pub use projectile::ProjectileResource;

use amethyst::prelude::World;

pub fn add_resources(world: &mut World) {
    world.add_resource(PlayState {
        lives: 3,
        enemy_count: 0,
    });
}
