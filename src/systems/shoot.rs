use crate::game::{Player, Projectile, ARENA_HEIGHT, ARENA_WIDTH};

use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, is_key_down},
};


pub struct ShootSystem;

impl<'s> System<'s> for ShootSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Projectile>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (players, mut transforms, projectiles, input): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {

            if let Some(fire) = input.action_is_down("fire") {
                if fire {
                    println!("FIRE!!!: {:?}", transform.translation());
                }
                
            }
        }
    }
}