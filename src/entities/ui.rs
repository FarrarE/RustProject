
use amethyst::ecs::prelude::World;

extern crate amethyst;

use amethyst::{
    assets::{Loader},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

/// Initialises a ui scoreboard
fn initialise_headsupdisplay(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );

    let score_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        -50., -50., 1., 200., 50.,
    );

    let score = world
        .create_entity()
        .with(score_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        )).build();

    world.add_resource(ScoreText { score });
}