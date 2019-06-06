use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Enemy, Player};
use crate::config::{ARENA_HEIGHT, ARENA_WIDTH};

use super::get_vector;

// Allows System to be accessed publicaly
pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (players, enemies, mut transforms, time): Self::SystemData) {
        let mut player_x = 0.0;
        let mut player_y = 0.0;

        for (player, player_pos) in (&players, &transforms).join() {
            player_x = player_pos.translation().x + player.width * 0.5;
            player_y = player_pos.translation().y + player.height * 0.5;
        }

        for (transform, enemy) in (&mut transforms, &enemies).join() {
            let start = (transform.translation().x, transform.translation().y);
            let end = (player_x, player_y);

            let (x, y) = get_vector(start, end);
            transform.translate_x(x * enemy.velocity * time.delta_seconds());
            transform.translate_y(y * enemy.velocity * time.delta_seconds());

            // We make sure the enemy remains in the arena.
            let enemy_y = transform.translation().y;
            transform.set_y(enemy_y.max(0.0).min(ARENA_HEIGHT - enemy.height));

            let enemy_x = transform.translation().x;
            transform.set_x(enemy_x.max(0.0).min(ARENA_WIDTH - enemy.width));
        }
    }
}
