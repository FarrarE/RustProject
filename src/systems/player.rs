use crate::components::Player;
use crate::resources::ProjectileResource;
use crate::entities::fire_projectile;
use crate::config::{ARENA_HEIGHT, ARENA_WIDTH, GAME_CONFIGURATION};

use amethyst::{
    core::{timing::Time, transform::Transform, nalgebra::Vector3},
    ecs::prelude::{Join, Read, ReadStorage, ReadExpect, System, WriteStorage, Entities, LazyUpdate},
    input::InputHandler,
};

//This system is responsible for moving the player based on provided input
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, ProjectileResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut players, mut transforms, time, input, projectile_resource, lazy_update): Self::SystemData) {
        
        for(player, transform) in (&mut players, &mut transforms).join() {
            let vert = input.axis_value("vertical");
            let horiz = input.axis_value("horizontal");

            let mut rise = 0.0;
            let mut run = 0.0;

            //count down towards next shot
            if player.trigger_reset_timer > 0.0 {
                player.trigger_reset_timer -= time.delta_seconds();
            }

            if let Some(fire) = input.action_is_down("fire") {
                //
                if fire && player.trigger_reset_timer <= 0.0 {

                    
                    //build a position vector for a projectile to spawn at
                    let fire_pos = Vector3::new(
                      transform.translation().x,
                      transform.translation().y,
                      0.0,
                    );

                    if let Some((x, y)) = input.mouse_position() {
                      run = (x as f32 - fire_pos.x);
                      rise = -(y as f32 - fire_pos.y);
                    }
                    

                    println!("FIRE!!!: {} / {}", rise, run);

                    //fire(STUFF)
                    fire_projectile(&entities, &projectile_resource, fire_pos, &lazy_update, rise, run);
                    //reset the timer
                    player.trigger_reset_timer = GAME_CONFIGURATION.trigger_reset_timeout;
                }
                
            }

            if let Some(movement) = vert {
                //println!("{:?}", vert);
                transform.translate_y(
                    player.velocity[1] * time.delta_seconds() * movement as f32,
                );
                
                // We make sure the player remains in the arena.
                let player_y = transform.translation().y;
                transform.set_y(
                    player_y
                        .max(0.0)
                        .min(ARENA_HEIGHT - player.height),
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
                        .max(0.0)
                        .min(ARENA_WIDTH - player.width),
                );
            }
        }
    }
}