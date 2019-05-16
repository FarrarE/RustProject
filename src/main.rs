#![deny(missing_docs)]
extern crate amethyst;

mod config;
mod state;
mod bundle;
pub mod components;
pub mod entities;
pub mod systems;
pub mod resources;

pub use crate::config::GameConfiguration;
pub use crate::config::GAME_CONFIGURATION;
pub use crate::state::GameState;
pub use crate::bundle::GameBundle;

use amethyst::prelude::*;
use amethyst::utils::application_root_dir;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{DisplayConfig, RenderBundle, Pipeline, Stage, DrawFlat, PosTex};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::input::InputBundle;

// Used for the with_transparency()
use gfx_core::state::{ColorMask};
use gfx::preset::blend::ALPHA;

const BACKGROUND_COLOUR: [f32; 4] = [0.25, 0.25, 0.25, 0.0];

pub fn run() -> Result<(), amethyst::Error> {
  let _ = &config::GAME_CONFIGURATION;
  let application_root = application_root_dir();
  let display_config_path =  format!("{}/resources/display_config.ron", application_root);
  let display_config = DisplayConfig::load(&display_config_path);
  let key_bindings_path = application_root.join("resources/bindings_config.ron");
  let resources_path = format!("{}/texture", application_root);

    let pipe = {
      Pipeline::build()
          .with_stage(
              Stage::with_backbuffer()
                  .clear_target(BACKGROUND_COLOUR, 1.0)
                  .with_pass(DrawFlat::<PosTex>::new().with_transparency(ColorMask::all(), ALPHA, None))
                  .with_pass(DrawUi::new())
          )

    };

    let game_data = GameDataBuilder::default()
    .with_bundle(
        InputBundle::<String, String>::new().with_bindings_from_file(
            &key_bindings_path
        )?,
    )?
    .with_bundle(GameBundle)?
    .with_bundle(TransformBundle::new())?
    .with_bundle(UiBundle::<String, String>::new())?
    .with_bundle(RenderBundle::new(pipe, Some(display_config)))?;

    // Create a game with out game data and our GameState.
    let mut game = Application::new(
        resources_path,
        GameState,
        game_data)?;

    Ok(
        game.run(),
    )

}

/*
use crate::game::Game;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    //load configs
    let path = format!("{}/resources/display_config.ron", application_root_dir()); 
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new()),
                //.with_pass(DrawUi::new()),
        );

    let binding_path = format!(
        "{}/resources/bindings_config.ron",
        application_root_dir()
    );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;


    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config))
        .with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::MovePlayerSystem, "move_player_system", &["input_system"])
        .with(systems::ShootSystem, "shoot_system", &["input_system"]);

    let mut game = Application::new("./", Game, game_data)?;

    game.run();

    Ok(())
}*/