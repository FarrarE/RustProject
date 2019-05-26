use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Enemy};
use crate::config::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::config::GAME_CONFIGURATION;

// Allows System to be accessed publicaly 
pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
      Entities<'s>,
      ReadStorage<'s, Enemy>,
      WriteStorage<'s, Transform>,
      Read<'s, Time>,
    );

    fn run(&mut self, (entities, enemy, mut transforms, time): Self::SystemData) {

        
    }
}