mod enemy;
mod player;
mod projectile;

pub use enemy::Enemy;
pub use player::Player;
pub use projectile::Projectile;

use amethyst::prelude::World;

pub fn register_components(world: &mut World) {
    world.register::<Player>();
    world.register::<Projectile>();
    world.register::<Enemy>();
}
