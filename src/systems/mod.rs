//mod stuff here
mod enemy;
mod player;
mod player_collision;
mod projectile;
mod projectile_collision;
mod spawn;

pub use self::{
    enemy::EnemySystem, player::PlayerSystem, player_collision::PlayerCollisionSystem,
    projectile::ProjectileSystem, projectile_collision::ProjectileCollisionSystem,
    spawn::SpawnSystem,
};

//Returns the unit vector between a start and end point.
pub fn get_vector(start: (f32, f32), end: (f32, f32)) -> (f32, f32) {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let mag = f32::sqrt(dx.powi(2) + dy.powi(2));

    (dx / mag, dy / mag)
}
