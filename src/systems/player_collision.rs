
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Join, System, ReadStorage, WriteStorage, Write};

use crate::components::Player;
use crate::components::Enemy;
use crate::resources::PlayState;

pub struct PlayerCollisionSystem;

impl<'s> System<'s> for PlayerCollisionSystem {

    type SystemData = (
        Write<'s, PlayState>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
    );

    fn run(&mut self, (mut play_state, players, transforms, mut enemys): Self::SystemData) {
        for (player_component, player_transform) in (&players, &transforms).join() {
            // create a collision box for our player
            let player_left = player_transform.translation()[0];
            let player_top = player_transform.translation()[1] + player_component.height;
            let player_right = player_left + player_component.width;

            // check to see if our player has collided with any enemy
            for (enemy_component, enemy_transform) in (&mut enemys, &transforms).join() {
                // create a collision box for our enemy
                let enemy_left = enemy_transform.translation()[0];
                let enemy_bottom = enemy_transform.translation()[1];
                let enemy_right = enemy_left + enemy_component.width;

                // if the two collision boxes overlap,
                if ((player_left <= enemy_right && player_left >= enemy_left)
                    || (player_right <= enemy_left && player_right >= enemy_right))
                    && (player_top >= enemy_bottom) {
                    // we have a collision. Decrement the number of lives of the game
                    if play_state.lives > 0 {
                        play_state.lives -= 1;
                    }

                }
            }
        }
    }
}