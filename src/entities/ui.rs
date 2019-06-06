use amethyst::{
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};
 
impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        initialise_scoreboard(world);
    }
}
 
// --snip--

/// Initialises a ui scoreboard
fn initialise_hudsupdisplay(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        -50., -50., 1., 200., 50.,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        )).build();

    world.add_resource(ScoreText { score });
}