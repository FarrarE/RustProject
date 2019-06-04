//! The set of items used by the Game Designer to tune the game after the coding is complete (e.g. player speed)

use amethyst::config::Config;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

/// "Constants" that control the game mechanics
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameConfiguration {
    /// The effect each key press has on the ship's speed
    #[serde(default)]
    pub player_velocity: f32,
    /// asteroid vertical velocity
    #[serde(default)]
    pub monster_velocity: f32,
    /// how long to wait before the first asteroid falls (sec)
    #[serde(default)]
    pub wait_for_first_spawn: f32,

    /// how long between spawns
    #[serde(default)]
    pub monster_spawn_delay: f32,
    /// laser vertical velocity
    #[serde(default)]
    pub projectile_velocity: f32,
    /// how long to wait after firing a laser before can fire again
    #[serde(default)]
    pub trigger_reset_timeout: f32,

    #[serde(default)]
    pub max_monster_count: u8,
}

// Default values
pub const PLAYER_VELOCITY: f32 = 10.0;
pub const MONSTER_VELOCITY: f32 = 8.0;
pub const PROJECTILE_VELOCITY: f32 = 60.0;
pub const WAIT_FOR_FIRST_SPAWN: f32 = 2.0;
pub const MONSTER_SPAWN_DELAY: f32 = 0.3;
pub const TRIGGER_RESET_TIMEOUT: f32 = 0.5;
pub const MAX_MONSTER_COUNT: u8 = 100;

pub const ARENA_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: f32 = 1000.0;

impl Default for GameConfiguration {
    fn default() -> Self {
        GameConfiguration {
            player_velocity: PLAYER_VELOCITY,
            monster_velocity: MONSTER_VELOCITY,
            monster_spawn_delay: MONSTER_SPAWN_DELAY,
            wait_for_first_spawn: WAIT_FOR_FIRST_SPAWN,
            projectile_velocity: PROJECTILE_VELOCITY,
            trigger_reset_timeout: TRIGGER_RESET_TIMEOUT,
            max_monster_count: MAX_MONSTER_COUNT,
        }
    }
}

lazy_static! {
    /// The actual values for the [game configuration](struct.GameConfiguration.html)</a>.
    ///
    /// The game configuration is automatically loaded on startup
    /// from the file "game_config.ron" in resources.
    ///
    /// It's a good pattern for managing the game configuration
    /// so that the game designer can change parameters and balance
    /// the game without having to recompile the code.
    ///
    /// This variable looks to the remaining code as if it were a set of constants.
    pub static ref GAME_CONFIGURATION: GameConfiguration = {
        let game_config_path = format!(
            "{}/resources/game_config.ron",
            env!("CARGO_MANIFEST_DIR")
        );
        GameConfiguration::load(&game_config_path)
    };

}
