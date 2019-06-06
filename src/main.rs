//#![deny(missing_docs)]
extern crate amethyst;
mod bundle;
pub mod components;
mod config;
pub mod entities;
pub mod resources;
mod state;
pub mod systems;

pub use crate::bundle::GameBundle;
pub use crate::config::GameConfiguration;
pub use crate::config::GAME_CONFIGURATION;
pub use crate::state::GameState;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, PosTex, RenderBundle, Stage};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::utils::application_root_dir;

// Used for the with_transparency()
use gfx::preset::blend::ALPHA;
use gfx_core::state::ColorMask;

const BACKGROUND_COLOUR: [f32; 4] = [0.25, 0.25, 0.25, 0.0];

pub fn run() -> Result<(), amethyst::Error> {
    let _ = &config::GAME_CONFIGURATION;
    let application_root = application_root_dir();
    let display_config_path = format!("{}/resources/display_config.ron", application_root);
    let display_config = DisplayConfig::load(&display_config_path);
    let key_bindings_path = format!("{}/resources/bindings_config.ron", application_root);
    let resources_path = format!("{}/texture", application_root);

    let pipe = {
        Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target(BACKGROUND_COLOUR, 1.0)
                .with_pass(DrawFlat::<PosTex>::new().with_transparency(
                    ColorMask::all(),
                    ALPHA,
                    None,
                ))
                .with_pass(DrawUi::new()),
        )
    };

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(GameBundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?;

    // Create a game with out game data and our GameState.
    let mut game = Application::new(resources_path, GameState, game_data)?;

    Ok(game.run())
}

fn main() {
    amethyst::start_logger(Default::default());
    if let Err(e) = run() {
        println!("ERROR: {}", e);
        std::process::exit(1);
    }
}
