use amethyst::prelude::*;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::winit::VirtualKeyCode;

use crate::entities::init_entities;
use crate::resources::add_resources;
// use crate::components::register_components;
use crate::resources::{PlayState};

pub struct GameState;


impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let world = state_data.world;
        //register_components(world);
        add_resources(world);
        init_entities(world);
        println!("GAMESTATE STARTED");
    }

    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{
        if let  StateEvent::Window(event) = event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit
            }
        }
        Trans::None
    }

    // Stop the game if the ship runs out of lives
    fn fixed_update(&mut self, state_data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans{
        println!("GAMESTATE FIXED UPDATE");
        let world = state_data.world;
        println!("FIXED_UPDATE: WORLD LOADED");
        let play_state = world.read_resource::<PlayState>();
        println!("FIXED_UPDATE: READ PLAYSTATE");
        if play_state.lives == 0 { Trans::Quit} else { Trans::None }
    }

    // This code tells Amethyst to run all the systems in your game data.
    fn update (&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans{
        state_data.data.update(&state_data.world);
        Trans::None
    }
}