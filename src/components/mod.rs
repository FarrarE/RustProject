mod player;
mod projectile;
mod enemy;

pub use player::Player;
pub use projectile::Projectile;
pub use enemy::Enemy;

use amethyst::prelude::World;

pub fn register_components(world: &mut World) {
    world.register::<Player>();
    world.register::<Projectile>();
    world.register::<Enemy>();
}