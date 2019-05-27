//mod stuff here
mod player;
mod projectile;
mod enemy;
mod spawn;


pub use self::{player::PlayerSystem, projectile::ProjectileSystem, enemy::EnemySystem, spawn::SpawnSystem};

//Returns the unit vector between a start and end point.
pub fn get_vector(start: (f32, f32), end: (f32, f32), velocity: f32 ) -> (f32 , f32) {
    let theta = f32::atan(start.0 - end.0 / start.1 - end.1);
    ((velocity * f32::cos(theta)), (velocity * f32::sin(theta)))
}