use crate::components::{Player, Projectile};
use crate::config::{ARENA_HEIGHT, ARENA_WIDTH};

use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, is_key_down},
};


pub struct ProjectileSystem;

impl<'s> System<'s> for ProjectileSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Projectile>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, projectiles, input): Self::SystemData) {
        
    }
}