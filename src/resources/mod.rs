mod projectile;
mod play;

pub use projectile::ProjectileResource;
pub use play::PlayState;

use amethyst::prelude::World;

pub fn add_resources(world: &mut World) {
    world.add_resource(PlayState{ lives:3});
}