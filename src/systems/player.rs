use crate::game::{Player, ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

//This system is responsible for moving the player based on provided input
pub struct MovePlayerSystem;

impl<'s> System<'s> for MovePlayerSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (players, mut transforms, time, input): Self::SystemData) {
        for(player, transform) in (&players, &mut transforms).join() {
            let vert = input.axis_value("vertical");
            let horiz = input.axis_value("horizontal");

            if let Some(movement) = vert {
                //println!("{:?}", vert);
                transform.translate_y(
                    player.velocity[1] * time.delta_seconds() * movement as f32,
                );
                
                // We make sure the player remains in the arena.
                let player_y = transform.translation().y;
                transform.set_y(
                    player_y
                        .max(player.height * 0.5)
                        .min(ARENA_HEIGHT - player.height * 0.5),
                );
            }
            if let Some(movement) = horiz {
                //println!("{:?}", horiz);
                transform.translate_x(
                    player.velocity[0] * time.delta_seconds() * (movement as f32),
                );
                
                // We make sure the player remains in the arena.
                let player_x = transform.translation().x;
                transform.set_x(
                    player_x
                        .max(player.width * 0.5)
                        .min(ARENA_WIDTH - player.width * 0.5),
                );
            }
        }
    }
}