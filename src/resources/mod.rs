mod projectile;
mod play;
mod enemy;


pub use projectile::ProjectileResource;
pub use enemy::EnemyResource;
pub use play::PlayState;

use amethyst::prelude::World;

pub fn add_resources(world: &mut World) {
    world.add_resource(PlayState{ lives:3, enemy_count:0});
}