//mod stuff here
mod player;
mod projectile;
mod enemy;
mod spawn;


pub use self::{player::PlayerSystem, projectile::ProjectileSystem, enemy::EnemySystem, spawn::SpawnSystem};

//Returns the unit vector between a start and end point.
<<<<<<< HEAD
<<<<<<< HEAD
pub fn get_vector(start: (f32, f32), end: (f32, f32)) -> (f32 , f32) {
=======
pub fn get_vector(start: (f32, f32), end: (f32, f32)) -> (f32 , f32) {
    // let left = end.0 < start.0;
    // let down = end.1 < start.1;
    // let theta = f32::atan(f32::abs(start.0 - end.0) / f32::abs(start.1 - end.1));
    // let mut x = (velocity * f32::cos(theta));
    // let mut y = (velocity * f32::sin(theta));
    
    // println!("theta {}", theta);
    // if left && down {    //if moving left and down
    //     (-x, -y)
    // }
    // else if left && !down { //if left and up
    //     (-x, y)
    // }
    // else if !left && down {   //right and down
    //     (x, -y)
    // }
    // else {
    //     (x, y)
    // }
>>>>>>> Pies now home in on the player
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let mag = f32::sqrt(dx.powi(2) + dy.powi(2));

<<<<<<< HEAD
    (dx / mag, dy / mag)
=======
pub fn get_vector(start: (f32, f32), end: (f32, f32), velocity: f32 ) -> (f32 , f32) {
    let theta = f32::atan(start.0 - end.0 / start.1 - end.1);
    ((velocity * f32::cos(theta)), (velocity * f32::sin(theta)))
>>>>>>> Initial movement pass
=======

    (dx / mag, dy / mag)
>>>>>>> Pies now home in on the player
}