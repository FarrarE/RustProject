mod enemy;
mod player;
mod projectile;
mod ui;

pub use enemy::Enemy;
pub use player::Player;
pub use projectile::Projectile;
pub use ui::UI;

use amethyst::prelude::World;

pub fn register_components(world: &mut World) {
    world.register::<Player>();
    world.register::<Projectile>();
    world.register::<Enemy>();
    world.register::<UI>();
}
