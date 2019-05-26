use amethyst::ecs::prelude::{Component, DenseVecStorage};

//The Player Component
pub struct Player {
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
    pub trigger_reset_timer: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            width: 64.0,
            height: 64.0,
            velocity: [100.0, 100.0],
            trigger_reset_timer: 5.0,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
