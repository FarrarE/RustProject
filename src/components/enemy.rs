use amethyst::ecs::prelude::{Component, DenseVecStorage};

//The Enemy Component
pub struct Enemy {
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
    pub trigger_reset_timer: f32,
}

impl Enemy {
    pub fn new() -> Enemy {
        Enemy {
            width: 50.0,
            height: 50.0,
            velocity: 100.0,
            trigger_reset_timer: 5.0,
        }
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
